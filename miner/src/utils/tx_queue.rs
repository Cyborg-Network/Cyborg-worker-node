use crate::{error::Result, types::MinerIdentity};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sled::{self};
use crate::global_config::TX_QUEUE_DB_PATH;
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

/// Async transaction executor closure type.
type TxExecutor =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<TxOutput>> + Send>> + Send + Sync>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TxOutput {
    RegistrationInfo(MinerIdentity),
    Success,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistentTx {
    pub id: u64,
    pub retry_count: u32,
    pub timestamp: u64,
    pub data: Option<Vec<u8>>,
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

pub static TRANSACTION_QUEUE: OnceCell<TransactionQueue> = OnceCell::new();

impl TransactionQueue {
    pub async fn new() -> Self {
        let db_path = TX_QUEUE_DB_PATH.as_str(); 
        let db = tokio::task::spawn_blocking(move || sled::open(db_path))
            .await
            .expect("Failed to join blocking task")
            .expect("Failed to open sled DB");
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut restored_count = 0;

        // Restore persisted transactions
        {
            let mut queue_lock = queue.lock().await;
            for item in db.iter() {
                if let Ok((_, value)) = item {
                    if let Ok(persistent_tx) = bincode::deserialize::<PersistentTx>(&value) {
                        let tx_obj = Transaction {
                            id: persistent_tx.id,
                            executor: Box::new(|| {
                                Box::pin(async {
                                    println!("[TX-QUEUE] Restored tx executed.");
                                    Ok(TxOutput::Success)
                                })
                            }),
                            responder: None,
                            retry_count: persistent_tx.retry_count,
                        };
                        queue_lock.push_back(tx_obj);
                        restored_count += 1;
                    }
                }
            }
        }

        println!(
            "[TX-QUEUE] Initialized sled DB at {db_path}. Restored {restored_count} txs."
        );

        let tx_queue = Self {
            inner: queue,
            processing: Arc::new(AtomicBool::new(false)),
            db,
        };

        if restored_count > 0 {
            println!("[TX-QUEUE] Resuming restored transactions...");
            tx_queue.start_processing();
        }

        tx_queue
    }

    pub async fn enqueue<F, Fut>(&self, executor: F) -> Result<oneshot::Receiver<Result<TxOutput>>>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<TxOutput>> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let tx_id = self.next_id().await;

        let tx_obj = Transaction {
            id: tx_id,
            executor: Box::new(move || Box::pin(executor())),
            responder: Some(tx),
            retry_count: 0,
        };

        let persistent_tx = PersistentTx {
            id: tx_id,
            retry_count: 0,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            data: None,
        };

        self.persist_tx(&persistent_tx).await;
        self.inner.lock().await.push_back(tx_obj);

        println!(
            "[TX-QUEUE] Enqueued tx #{}. Queue size: {}",
            tx_id,
            self.inner.lock().await.len()
        );

        self.start_processing();

        Ok(rx)
    }

    fn start_processing(&self) {
        if self.processing.swap(true, Ordering::SeqCst) {
            return;
        }

        let inner = Arc::clone(&self.inner);
        let processing_flag = Arc::clone(&self.processing);
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
                                TransactionQueue::delete_persisted(&db, tx.id).await;
                                if let Some(responder) = tx.responder.take() {
                                    let _ = responder.send(Ok(result));
                                }
                            }
                            Err(e) if tx.retry_count < MAX_RETRIES => {
                                println!("[TX-QUEUE] Tx #{} failed: {}", tx.id, e);
                                tx.increment_retry();

                                let delay_ms = 1000 * 2u64.pow(tx.retry_count().min(10));
                                println!(
                                    "[TX-QUEUE] Retrying tx #{} after {} ms (attempt #{})",
                                    tx.id,
                                    delay_ms,
                                    tx.retry_count()
                                );
                                sleep(Duration::from_millis(delay_ms)).await;

                                let mut queue = inner.lock().await;
                                queue.push_front(tx);
                            }
                            Err(e) => {
                                println!("[TX-QUEUE] Tx #{} permanently failed: {}", tx.id, e);
                                TransactionQueue::delete_persisted(&db, tx.id).await;
                                if let Some(responder) = tx.responder.take() {
                                    let _ = responder.send(Err(e));
                                }
                            }
                        }
                    }
                    None => {
                        println!("[TX-QUEUE] Queue empty. Halting processor.");
                        processing_flag.store(false, Ordering::SeqCst);
                        break;
                    }
                }
            }
        });
    }

    async fn persist_tx(&self, tx: &PersistentTx) {
        let db = self.db.clone();
        let key = tx.id.to_be_bytes();
        let value = bincode::serialize(tx).unwrap();
        let _ = tokio::task::spawn_blocking(move || db.insert(key, value)).await;
    }

    async fn delete_persisted(db: &sled::Db, id: u64) {
        let key = id.to_be_bytes();
        let db = db.clone();
        let _ = tokio::task::spawn_blocking(move || db.remove(key)).await;
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
