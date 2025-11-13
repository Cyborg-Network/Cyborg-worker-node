use anyhow::Result;
use futures::stream::SplitSink;
use serde::Serialize;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt, DiskExt};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::{self,sleep};
use crate::api::logs;
use crate::error_handling::ClientError;
use crate::crypto::encrypt_message;
use futures::SinkExt;

#[derive(Serialize, Debug)]
pub struct Usage {
    title: &'static str,
    cpu_usage: f32,
    mem_usage: u64,
    disk_usage: u64,
    recent_logs: String,
    zk_stage: u8,
}

#[derive(Serialize, Debug)]
pub struct MemoryAndDiskInfo {
    pub total_memory: u64,
    pub total_disk: u64,
}

use std::process::Command;
use std::str;

// this is here for the same reason mentioned in storage.rs
pub fn return_disk_usage() -> u64 {
    let output = Command::new("df")
        .arg("--block-size=1") 
        .arg("-B1")  
        .output()
        .expect("Failed to execute df command");

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    let mut total_used_space: u64 = 0;

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let Some(filesystem) = parts.get(0) {
            if filesystem.starts_with("/dev/") {
                if let Some(used_space) = parts.get(2) {  // Get the used space (3rd column)
                    total_used_space += used_space.parse::<u64>().unwrap_or(0);
                }
            }
        }
    }

    total_used_space
}

impl Usage {
    pub async fn get_usage_snapshot(log_storage: logs::LogsStorage, zk_stage: Arc<Mutex<u8>>) -> Result<Usage> {
        let mut system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory()
                .with_disks(),
        );

        // Refreshes and sleeps are needed for this to work properly as it measures usage over time

        system.refresh_cpu();
        system.refresh_disks_list();

        sleep(Duration::from_secs(1)).await;

        system.refresh_cpu();

        let zk_stage = *zk_stage.lock().await;

        let _ = system.disks().iter()
                .map(|disk| {
                    let total_space = disk.total_space();
                    let available_space = disk.available_space();
                    total_space - available_space // Calculate used space
                });

        let logs: String;

        if let Ok(log_result) = logs::read_logs(){
            logs = log_result;
        } else {
            logs = String::from("");
        }
        
        let metric_item = Usage {
            title: "Usage",
            cpu_usage: system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
                / system.cpus().len() as f32,
            mem_usage: system.used_memory() * 1024,
            disk_usage: return_disk_usage(),
            recent_logs: logs,
            zk_stage
        };
        
        println!("{:#?}", metric_item);

        Ok(metric_item)
    }

    pub async fn stream_usage(
        stream: &Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        diffie_hellman_key: Arc<RwLock<Option<[u8; 32]>>>,
        log_storage: Arc<Mutex<Vec<String>>>,
        zk_stage: Arc<Mutex<u8>>,
    ) -> Result<(), ClientError> {
        let diffie_hellman_key_copy = {
            let diffie_hellman_key_guard = diffie_hellman_key.read()
                .map_err(|e| ClientError::UsageError(e.to_string()))?;
         
            if let Some(key) = *diffie_hellman_key_guard {
                key
            } else {
                return Err(ClientError::AuthError("No diffie hellman key found".to_string()));
            }
        };
    
        let mut query_interval = time::interval(Duration::from_secs(2));
    
        loop {
            query_interval.tick().await;

            let log_storage_clone = Arc::clone(&log_storage);
    
            let usage_snapshot = Usage::get_usage_snapshot(log_storage_clone, Arc::clone(&zk_stage)).await
                .map_err(|e| ClientError::UsageError(e.to_string()))?;
    
            let usage_snapshot = serde_json::to_string(&usage_snapshot)
                .map_err(|e| ClientError::UsageError(e.to_string()))?;
    
            let encrypted_message = encrypt_message("Usage", &diffie_hellman_key_copy, usage_snapshot);
    
            let encrypted_message = serde_json::to_string(&encrypted_message)
                .map_err(|e| ClientError::UsageError(e.to_string()))?;
    
            println!("Sending usage message: {:?}", encrypted_message);

            let mut stream_guard = stream.lock().await;
    
            if let Err(e) = stream_guard.send(Message::Text(encrypted_message)).await {
                return Err(ClientError::UsageError(e.to_string())); // Return the error
            }
        }
    }
}
