use crate::error::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sled;
use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex as StdMutex},
};
use std::sync::atomic::Ordering;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};

use crate::global_config::TX_QUEUE_DB_PATH;
use crate::types::MinerIdentity;

const MAX_RETRIES: u32 = 500;
const EMPTY_SLEEP_MS: u64 = 250; // when queue empty, sleep briefly and continue

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TxOutput {
    RegistrationInfo(MinerIdentity),
    Success,
}

pub type TxExecutor =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<TxOutput>> + Send>> + Send + Sync>;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentTx {
    pub id: u64,
    pub task_key: String,
    pub payload: Option<Vec<u8>>, 
    pub retry_count: u32,
    pub timestamp: u64,
}

/// In-memory Transaction
pub struct Transaction {
    pub id: u64,
    pub executor: TxExecutor,
    pub responder: Option<oneshot::Sender<Result<TxOutput>>>,
    pub retry_count: u32,
}

impl Transaction {
    async fn execute(&self) -> Result<TxOutput> {
        (self.executor)().await
    }

    fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    fn retry_count(&self) -> u32 {
        self.retry_count
    }
}


type TxHandler = Arc<
    dyn Fn(PersistentTx) -> Pin<Box<dyn Future<Output = Result<TxOutput>> + Send>>
        + Send
        + Sync,
>;

pub struct TransactionQueue {
    inner: Arc<Mutex<VecDeque<Transaction>>>,
    processing: Arc<std::sync::atomic::AtomicBool>,
    db: sled::Db,
    registry: Arc<StdMutex<HashMap<String, TxHandler>>>,
}

pub static TRANSACTION_QUEUE: OnceCell<Arc<TransactionQueue>> = OnceCell::new();

impl TransactionQueue {
    /// Open sled DB and initialize queue + registry
    pub async fn new() -> Self {
        let db_path = TX_QUEUE_DB_PATH.as_str().to_string();
        let db = tokio::task::spawn_blocking(move || sled::open(db_path))
            .await
            .expect("Failed to join blocking task")
            .expect("Failed to open sled DB");

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut persisted_count = 0usize;

        {
            let db_clone = db.clone();
            let entries: Vec<_> = tokio::task::spawn_blocking(move || {
                let mut out = Vec::new();
                for item in db_clone.iter() {
                    if let Ok((_k, v)) = item {
                        if bincode::deserialize::<PersistentTx>(&v).is_ok() {
                            out.push(());
                        }
                    }
                }
                out
            })
            .await
            .unwrap();
            persisted_count = entries.len();
        }

        println!(
            "[TX-QUEUE] Initialized sled DB at {}. Found {} persisted tx records.",
            TX_QUEUE_DB_PATH.as_str(),
            persisted_count
        );

        Self {
            inner: queue,
            processing: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            db,
            registry: Arc::new(StdMutex::new(HashMap::new())),
        }
    }


  
    pub fn register_handler<F, Fut>(&self, task_key: &str, handler: F)
    where
        F: Fn(PersistentTx) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<TxOutput>> + Send + 'static,
    {
        let mut reg = self.registry.lock().expect("Registry mutex poisoned");
        reg.insert(
            task_key.to_string(),
            Arc::new(move |ptx: PersistentTx| Box::pin(handler(ptx))),
        );
    }


    /// Returns a oneshot receiver to await the tx result.
    pub async fn enqueue<F, Fut>(
        self: &Arc<Self>,
        task_key: &str,
        executor_fn: F,
    ) -> Result<oneshot::Receiver<Result<TxOutput>>>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<TxOutput>> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let tx_id = self.next_id().await;

        let boxed_executor: TxExecutor = Box::new(move || Box::pin(executor_fn()));

        let tx_obj = Transaction {
            id: tx_id,
            executor: boxed_executor,
            responder: Some(tx),
            retry_count: 0,
        };

        let persistent_tx = PersistentTx {
            id: tx_id,
            task_key: task_key.to_string(),
            payload: None, 
            retry_count: 0,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        };

        self.persist_tx(&persistent_tx).await;

        {
            let mut q = self.inner.lock().await;
            q.push_back(tx_obj);

            println!(
                "[TX-QUEUE] Enqueued tx #{} (task_key={}). Queue size: {}",
                tx_id,
                task_key,
                q.len()
            );
        }

        Ok(rx)
    }

    pub fn start_processing(&self) {
        if self.processing.swap(true, Ordering::SeqCst) {
            return;
        }

        let inner = Arc::clone(&self.inner);
        let db = self.db.clone();

        tokio::spawn(async move {
            loop {
                let tx_opt = {
                    let mut queue = inner.lock().await;
                    queue.pop_front()
                };

                match tx_opt {
                    Some(mut tx) => {
                        println!(
                            "[TX-QUEUE] Processing tx #{} (retry #{})",
                            tx.id,
                            tx.retry_count()
                        );

                        match tx.execute().await {
                            Ok(result) => {
                                println!("[TX-QUEUE] Tx #{} succeeded: {:?}", tx.id, result);

                                // delete persisted entry
                                let _ = tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let id = tx.id;
                                    move || {
                                        let key = id.to_be_bytes();
                                        let _ = db.remove(key);
                                    }
                                })
                                .await;

                                if let Some(responder) = tx.responder.take() {
                                    let _ = responder.send(Ok(result));
                                }
                            }
                            Err(e) if tx.retry_count < MAX_RETRIES => {
                                println!("[TX-QUEUE] Tx #{} failed: {}", tx.id, e);

                                // increment retry count and persist the updated retry count
                                let new_retry = tx.retry_count + 1;
                                let _ = tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let id = tx.id;
                                    let retry_val = new_retry;
                                    move || {
                                        if let Ok(Some(v)) = db.get(id.to_be_bytes()) {
                                            if let Ok(mut ptx) = bincode::deserialize::<PersistentTx>(&v) {
                                                ptx.retry_count = retry_val;
                                                let _ = db.insert(id.to_be_bytes(), bincode::serialize(&ptx).unwrap());
                                            }
                                        }
                                    }
                                })
                                .await;

                                let delay_ms = 1000u64.saturating_mul(2u64.pow(std::cmp::min(tx.retry_count(), 10)));
                                println!(
                                    "[TX-QUEUE] Retrying tx #{} after {} ms (attempt #{})",
                                    tx.id,
                                    delay_ms,
                                    new_retry
                                );
                                sleep(Duration::from_millis(delay_ms)).await;

                                let mut queue = inner.lock().await;
                                tx.retry_count = new_retry;
                                queue.push_front(tx);
                            }
                            Err(e) => {
                                println!("[TX-QUEUE] Tx #{} permanently failed: {}", tx.id, e);

                                // delete persisted entry
                                let _ = tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let id = tx.id;
                                    move || {
                                        let key = id.to_be_bytes();
                                        let _ = db.remove(key);
                                    }
                                })
                                .await;

                                if let Some(responder) = tx.responder.take() {
                                    let _ = responder.send(Err(e));
                                }
                            }
                        }
                    }
                    None => {
                        // queue empty -> sleep briefly then continue
                        sleep(Duration::from_millis(EMPTY_SLEEP_MS)).await;
                        continue;
                    }
                }
            }
        });
    }

    /// Returns number of restored transactions.
    pub async fn restore_from_db_using_registry(&self) -> usize {
        let mut restored_count = 0usize;

        // read persisted entries in a blocking task
        let entries: Vec<PersistentTx> = {
            let db = self.db.clone();
            tokio::task::spawn_blocking(move || {
                let mut out = Vec::new();
                for item in db.iter() {
                    if let Ok((_k, v)) = item {
                        if let Ok(p) = bincode::deserialize::<PersistentTx>(&v) {
                            out.push(p);
                        }
                    }
                }
                out
            })
            .await
            .unwrap()
        };

        {
            let mut q = self.inner.lock().await;
            let registry_guard = self.registry.lock().expect("Registry mutex poisoned");

            for p in entries {
                let tx_obj = if let Some(handler) = registry_guard.get(&p.task_key) {
                    let handler_clone = Arc::clone(handler);
                    let p_clone = p.clone();
                    let exec: TxExecutor = Box::new(move || {
                        let handler_clone = Arc::clone(&handler_clone);
                        let ptx = p_clone.clone();
                        Box::pin(async move { (handler_clone)(ptx).await })
                    });

                    Transaction {
                        id: p.id,
                        executor: exec,
                        responder: None,
                        retry_count: p.retry_count,
                    }
                } else {
                    let missing_key = p.task_key.clone();
                    let id_for_delete = p.id;
                   let exec: TxExecutor = Box::new(move || {
                    let missing_key_clone = missing_key.clone(); 

                    Box::pin(async move {
                        Err(crate::error::Error::Custom(format!(
                            "No handler registered for task_key '{}'",
                            missing_key_clone
                        )))
                    })
                });

                    Transaction {
                        id: p.id,
                        executor: exec,
                        responder: None,
                        retry_count: p.retry_count,
                    }
                };

                q.push_back(tx_obj);
                restored_count += 1;
            }
        }

        restored_count
    }

    async fn persist_tx(&self, tx: &PersistentTx) {
        let db = self.db.clone();
        let key = tx.id.to_be_bytes();
        let value = bincode::serialize(tx).unwrap_or_else(|e| {
            panic!("Failed to serialize PersistentTx: {}", e);
        });

        let _ = tokio::task::spawn_blocking(move || db.insert(key, value)).await;
    }

    async fn next_id(&self) -> u64 {
        let counter_key = b"tx_counter";
        let db = self.db.clone();

        tokio::task::spawn_blocking(move || {
            let next_id = db
                .update_and_fetch(counter_key, |old| {
                    let next = match old {
                        Some(v) => {
                            let mut arr = [0u8; 8];
                            arr.copy_from_slice(&v);
                            u64::from_be_bytes(arr) + 1
                        }
                        None => 1,
                    };
                    Some(next.to_be_bytes().to_vec())
                })
                .unwrap();

            let mut arr = [0u8; 8];
            arr.copy_from_slice(&next_id.unwrap());
            u64::from_be_bytes(arr)
        })
        .await
        .unwrap()
    }
}
