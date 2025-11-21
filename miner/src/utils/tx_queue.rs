use crate::error::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sled;

use crate::global_config::TX_QUEUE_DB_PATH;
use crate::types::MinerIdentity; 
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};

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
    pub retry_count: u32,
    pub timestamp: u64,
    // pub kind: TxKind,
}

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

pub struct TransactionQueue {
    inner: Arc<Mutex<VecDeque<Transaction>>>,
    processing: Arc<AtomicBool>,
    db: sled::Db,
}

pub static TRANSACTION_QUEUE: OnceCell<Arc<TransactionQueue>> = OnceCell::new();

impl TransactionQueue {
    /// Create/open sled DB and initialize queue.
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
            processing: Arc::new(AtomicBool::new(false)),
            db,
        }
    }

   
    pub async fn enqueue<F, Fut>(
        self: &Arc<Self>,
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
            retry_count: 0,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            
        };

        // persist then push to queue
        self.persist_tx(&persistent_tx).await;

        {
            let mut q = self.inner.lock().await;
            q.push_back(tx_obj);

            println!(
                "[TX-QUEUE] Enqueued tx #{}. Queue size: {}",
                tx_id,
                q.len()
            );
        }

        Ok(rx)
    }

    /// Start background processing. This method is idempotent.
    /// Processor stays alive (loops forever) and polls the queue, sleeping when empty.
    pub fn start_processing(&self) {
        // If already running, do nothing
        if self.processing.swap(true, Ordering::SeqCst) {
            return;
        }

        let inner = Arc::clone(&self.inner);
        let processing_flag = Arc::clone(&self.processing);
        let db = self.db.clone();

        tokio::spawn(async move {
            // keep processor alive until process exits
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

                                // delete persisted entry (run in blocking task)
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
                                let mut new_retry = tx.retry_count + 1;
                                // Save updated retry_count to DB
                                let _ = tokio::task::spawn_blocking({
                                    let db = db.clone();
                                    let id = tx.id;
                                    let retry = new_retry;
                                    move || {
                                        if let Ok(Some(v)) = db.get(id.to_be_bytes()) {
                                            if let Ok(mut ptx) = bincode::deserialize::<PersistentTx>(&v) {
                                                ptx.retry_count = retry;
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
                        // queue empty -> pause briefly then continue (processor stays running)
                        // set processing flag true (already true)
                        sleep(Duration::from_millis(EMPTY_SLEEP_MS)).await;
                        continue;
                    }
                }
            }

         
        });
    }

    /// Restore persisted transactions by using a build function that converts PersistentTx -> Transaction.
    /// This function returns the number of restored txs. After restore, call `start_processing()` (it will be no-op if already running).
    pub async fn restore_from_db<F>(&self, mut build_fn: F) -> usize
    where
        F: FnMut(PersistentTx) -> Transaction,
    {
        let mut restored_count = 0usize;
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
            for p in entries {
                let tx_obj = build_fn(p.clone());
                q.push_back(tx_obj);
                restored_count += 1;
            }
        }

        restored_count
    }

    /// write PersistentTx to sled
    async fn persist_tx(&self, tx: &PersistentTx) {
        let db = self.db.clone();
        let key = tx.id.to_be_bytes();
        let value = bincode::serialize(tx).unwrap();

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
