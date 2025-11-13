use std::{process::Stdio, sync::{Arc, RwLock}};
use tempfile::TempDir;
use tokio::{fs, process::Command};
use crate::{
    api::{dbus::watch_for_zk_stage_update, logs, HealthStatus, Init, Usage}, auth::{self, WsAuthRequest}, crypto::{decode_polkadot_address, encrypt_message}, error_handling::{construct_client_error_message, ClientError}, formats::{self, OptionalStatusCode, OptionalUuid}, AgentConfig, CurrentTask, TaskOwner, TASK_CONTAINER_PREFIX
};
use anyhow::{anyhow, Result};
use futures::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use tokio::{
    task::JoinHandle,
    sync::Mutex,
    io::{AsyncReadExt, AsyncWriteExt}, 
    select, 
    net::{TcpListener, TcpStream}
};
use tokio_tungstenite::{
    WebSocketStream, accept_async, tungstenite::Message
};
use uuid::Uuid;
use http::{Response, StatusCode};

const HTTP_ADDR: &str = "0.0.0.0:8080";
const WS_ADDR: &str = "0.0.0.0:8081";

const DEPOSIT_CONTAINER_KEYS_SCRIPT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/deposit_container_key.sh"));
const DEPOSIT_NATIVE_KEYS_SCRIPT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/deposit_native_key.sh"));

#[derive(Deserialize, Serialize)]
/// the required format for messages within text websocket frames
struct Messages {
    #[serde(rename = "type")]
    request_type: RequestType,
    /// randomly generated id for the message
    #[serde(with = "formats::SerdeUuid")]
    id: Uuid,
    /// reference to the previous message, if any
    #[serde(rename = "ref")]
    reference_id: OptionalUuid,
    /// ms since UNIX epoch
    timestamp: String,
    /// a status code
    status_code: OptionalStatusCode,
    args: Vec<String>,
    /// timeout value in ms
    timeout: Option<u64>,
    data: Value,
}

#[derive(Deserialize, Serialize)]
pub enum RequestType {
    /// a new request
    #[serde(rename = "syn")]
    Syn,
    #[serde(rename = "ack")]
    /// a response (acknowledgement) to a previous request
    Ack,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "endpoint")]
enum WsMessageFormat {
    Request(WsApiRequest),
    Auth(WsAuthRequest),
    Test(WsTestRequest),
}

#[derive(Deserialize, Serialize, Debug)]
struct WsApiRequest {
    target_ip: String,
    request_type: WsApiRequestType,
}

#[derive(Deserialize, Serialize, Debug)]
enum WsApiRequestType {
    Init,
    Usage,
    CreateContainerKey(CreateContainerKeyRequest),
    DepositContainerKey(DepositContainerKeyRequest),
}

#[derive(Deserialize, Serialize, Debug)]
struct DepositContainerKeyRequest {
    task_id: String,
    key: String,
}

#[derive(Deserialize, Serialize)]
struct DepositContainerKeyResponse {
    success: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateContainerKeyRequest {
    task_id: String,
}

#[derive(Deserialize, Serialize)]
struct CreateContainerKeyResponse {
    pub_key: String,
    priv_key: String
}

#[derive(Deserialize, Serialize, Debug)]
struct WsAuthResponse{
    response_type: String,
    node_public_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct WsTestRequest {
    target_ip: String,
}

#[derive(Debug)]
enum TaskIdentifier<'a> {
    Container(&'a str),
    Native(&'a str),
}

#[derive(Debug)]
struct DepositPublicKeyArgs<'a> {
    identifier: &'a str,
    script: &'a str,
}

async fn handle_http_request(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf).await?;

    let error_response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";

    // Try to parse the request from the raw buffer
    let request_str = String::from_utf8_lossy(&buf[..bytes_read]);

    if let Some(request_line) = request_str.lines().next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() == 3 {
            let method = parts[0];
            let path = parts[1];

            // Only handle GET requests for simplicity
            if method == "GET" && path == "/check-health" {
                let is_healthy = HealthStatus::get_health_status().await.ok();

                if let Some(is_healthy) = is_healthy {

                    let response = if is_healthy {
                        Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body("{\"isActive\": \"true\"}".to_string())?
                    } else {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header("Content-Type", "application/json")
                            .body("{\"isActive\": \"false\"}".to_string())?
                    };

                    let response_str = format!(
                        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                        response.status().as_u16(),
                        response.status().canonical_reason().unwrap_or(""),
                        response.body().len(),
                        response.headers().get("Content-Type").ok_or(anyhow!("Failed getting the header"))?.to_str()?,
                        response.body(),
                    );

                    stream.write_all(response_str.as_bytes()).await?;
                    stream.flush().await?;
                    return Ok(());
                } else{
                    return Ok(());
                }
            } else {
                stream.write_all(error_response.as_bytes()).await?;
                stream.flush().await?;
            }
        }
    }

    stream.write_all(error_response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

async fn handle_ws_connections(stream: TcpStream, current_task: CurrentTask) -> Result<()> {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("WebSocket handshake has been successfully completed");

        let guard = current_task.read().await;
        let task_ref = guard.as_ref().ok_or_else(|| anyhow!("No current task!"))?;

        let public_key_bytes = decode_polkadot_address(&task_ref.task_owner);

        let (ws_sender, mut ws_receiver) = ws_stream.split();

        let zk_stage: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

        let zk_stage_updating_clone = Arc::clone(&zk_stage);

        tokio::spawn(async move {
            println!("Print 1");
            if let Err(e) = watch_for_zk_stage_update(zk_stage_updating_clone).await {
                eprintln!("Error watching for zk stage update: {}", e);
            }
        });

        let ws_sender = Arc::new(Mutex::new(ws_sender));
        let log_storage: logs::LogsStorage = Arc::new(Mutex::new(Vec::new()));
        let diffie_hellman_key = Arc::new(RwLock::new(None));

        let mut streaming_task: Option<JoinHandle<()>> = None;

        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(message)) => {
                    println!("Message received: {}", message);
                    match from_str::<WsMessageFormat>(&message) {
                        Ok(msg) => match msg {
                            WsMessageFormat::Request(request) => {
                                match request.request_type {

                                    WsApiRequestType::Usage => {
                                        if streaming_task.is_none() {
                                            let stream_usage_diffie_hellman_key = Arc::clone(&diffie_hellman_key);
                                            let stream_usage_log_storage = Arc::clone(&log_storage);
                                            let stream_usage_ws_sender = Arc::clone(&ws_sender);
                                            let stream_usage_zk_stage = Arc::clone(&zk_stage);

                                            streaming_task = Some(tokio::spawn(async move {
                                               let sender = stream_usage_ws_sender; 

                                               if let Err(e) = Usage::stream_usage( 
                                                    &sender, 
                                                    stream_usage_diffie_hellman_key, 
                                                    stream_usage_log_storage,
                                                    stream_usage_zk_stage,
                                                ).await {
                                                    let mut sender_guard = sender.lock().await;
                                                    let _ =sender_guard.send(
                                                       Message::Text(construct_client_error_message(e))).await;
                                                }
                                            }));
                                        }
                                    }

                                    WsApiRequestType::Init => {
                                        let init_res = Init::return_init_message(&diffie_hellman_key).await;
                                        
                                        match init_res {
                                            Ok(init_message) => {
                                                let mut sender_guard = ws_sender.lock().await;
                                                if let Err(e) = sender_guard.send(Message::Text(init_message)).await {
                                                    println!("Failed to send init message: {}", e);
                                                }
                                            }
                                            Err(e) => {
                                                let mut sender_guard = ws_sender.lock().await;
                                                if let Err(e) = sender_guard.send(Message::Text(construct_client_error_message(e))).await {
                                                    println!("Failed to send init message: {}", e);
                                                }
                                            }
                                        }
                                    }

                                    WsApiRequestType::CreateContainerKey(request) => {
                                        if let Err(e) = handle_create_container_ssh_key(request.task_id, &diffie_hellman_key, &ws_sender).await {
                                            println!("Failed to send request key response message, sending client error.");
                                            let mut sender_guard = ws_sender.lock().await;
                                            sender_guard.send(Message::Text(construct_client_error_message(e))).await?;
                                        }
                                    }

                                    WsApiRequestType::DepositContainerKey(request) => {
                                        if let Err(e) = handle_deposit_container_ssh_key(request.task_id, &diffie_hellman_key, request.key, &ws_sender).await {
                                            println!("Failed to send request key response message, sending client error.");
                                            let mut sender_guard = ws_sender.lock().await;
                                            sender_guard.send(Message::Text(construct_client_error_message(e))).await?;
                                        }
                                    }
                                }
                            }

                            WsMessageFormat::Auth(request) => {
                              let auth_res = auth::construct_auth_response(request, &diffie_hellman_key, &log_storage, public_key_bytes);

                              match auth_res {
                                Ok(response) => {
                                    let mut sender_guard = ws_sender.lock().await;
                                    if let Err(e) = sender_guard.send(Message::Text(response)).await {
                                        println!("Failed to send auth message: {}", e);
                                    }
                                }
                                Err(e) => {
                                    let mut sender_guard = ws_sender.lock().await;
                                    if let Err(e) = sender_guard.send(Message::Text(construct_client_error_message(e))).await {
                                        println!("Failed to send auth message: {}", e);
                                }
                                }
                              }
                            }

                            WsMessageFormat::Test(_) => {
                               let mut sender_guard = ws_sender.lock().await;
                                if let Err(e) = sender_guard.send(Message::Text("Test".to_string())).await {
                                    println!("Failed to send auth message: {}", e);
                                } 
                            }
                        }
                        _ => { 
                            let mut sender_guard = ws_sender.lock().await;
                            if let Err(e) = sender_guard.send(Message::Text(construct_client_error_message(ClientError::InvalidRequestError))).await {
                                println!("Failed to send auth message: {}", e);
                            } 
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub async fn run_client<'a>(config: AgentConfig<'a>) -> Result<()> {
    let http_listener = TcpListener::bind(HTTP_ADDR).await?;
    let ws_listener = TcpListener::bind(WS_ADDR).await?;

    let http_task = tokio::spawn(async move {
        loop {
            match http_listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_http_request(stream).await {
                            eprintln!("HTTP handler error: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("HTTP listener error: {:?}", e);
                    break;
                }
            }
        }
    });

    let ws_task = tokio::spawn(async move {
        loop {
            match ws_listener.accept().await {
                Ok((stream, _)) => {
                    let current_task = Arc::clone(&config.current_task);
                    tokio::spawn(async move {
                        if let Err(e) = handle_ws_connections(stream, current_task).await {
                            eprintln!("WebSocket handler error: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("WebSocket listener error: {:?}", e);
                    break;
                }
            }
        }
    });

    select! {
        res = http_task => {
            if let Err(e) = res {
                eprintln!("HTTP task panicked: {:?}", e);
            }
        },
        res = ws_task => {
            if let Err(e) = res {
                eprintln!("WebSocket task panicked: {:?}", e);
            }
        },
    }

    Ok(())
}

fn construct_container_name(task_id: String) -> String {
    format!("{}{}", *TASK_CONTAINER_PREFIX, task_id)
}

async fn generate_ssh_keypair() -> Result<CreateContainerKeyResponse, ClientError> {
    let temp_dir = TempDir::new()
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?; 

    let key_path = temp_dir.path().join("id_ed25519");
    let key_path_str = key_path.to_string_lossy().to_string();
    let pub_key_path = format!("{}.pub", key_path_str);

    let output = Command::new("ssh-keygen")
        .arg("-t").arg("ed25519")
        .arg("-f").arg(&key_path)
        .arg("-N").arg("")  // No passphrase
        .arg("-q")  // Quiet
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    if !output.status.success() {
        // sanitize this code for client errors
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(ClientError::CreateContainerKeyError(format!(
            "ssh-keygen failed (code {:?}):\nSTDOUT:\n{}\nSTDERR:\n{}",
            output.status.code(),
            stdout,
            stderr
        )));
    }

    let priv_key = fs::read_to_string(&key_path).await
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;
    
    let pub_key = fs::read_to_string(&pub_key_path).await
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    let _ = fs::remove_file(&pub_key_path).await;

    Ok(CreateContainerKeyResponse {
        priv_key,
        pub_key: pub_key.trim().to_string(),
    })
}

async fn deposit_public_key<'a>(
    task_identifier: TaskIdentifier<'a>,
    public_key: &str,
) -> Result<(), ClientError> {
    let args = match task_identifier {
        TaskIdentifier::Container(container_name) => { 
            DepositPublicKeyArgs {
                identifier: container_name,
                script: &DEPOSIT_CONTAINER_KEYS_SCRIPT
            }
        },
        TaskIdentifier::Native(active_user) => {
            DepositPublicKeyArgs {
                identifier: active_user,
                script: &DEPOSIT_NATIVE_KEYS_SCRIPT
            }
        }
    };

    let mut child = Command::new("bash")
        .arg("-s")
        .arg("--")
        .arg(args.identifier)
        .arg(public_key)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(args.script.as_bytes()).await
            .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;
    }

    let output = child.wait_with_output().await
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ClientError::CreateContainerKeyError(
            format!("Script failed: {}", stderr)
        ));
    }

    Ok(())
}

async fn handle_create_container_ssh_key(
    task_id: String, 
    diffie_hellman_key: &Arc<RwLock<Option<[u8; 32]>>>,
    sender: &Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>
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

    let container_name = construct_container_name(task_id);
    
    let keypair = generate_ssh_keypair().await?;
    
    deposit_public_key(TaskIdentifier::Container(&container_name), &keypair.pub_key).await?;
    
    let data_string = serde_json::to_string(&keypair)
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    let encrypted_message = encrypt_message("KeyPairReturned", &diffie_hellman_key_copy, data_string)
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;
    
    let encrypted_message_str = serde_json::to_string(&encrypted_message)
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    let mut sender_guard = sender.lock().await;
    sender_guard.send(Message::Text(encrypted_message_str)).await
        .map_err(|e| ClientError::CreateContainerKeyError(e.to_string()))?;

    Ok(())
}

async fn handle_deposit_container_ssh_key(
    task_id: String, 
    diffie_hellman_key: &Arc<RwLock<Option<[u8; 32]>>>,
    key: String, 
    sender: &Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, 
    Message>>>
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

    let container_name = construct_container_name(task_id);

    let mut child = Command::new("bash")
        .arg("-s")
        .arg("--")
        .arg(container_name)
        .arg(key)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(DEPOSIT_CONTAINER_KEYS_SCRIPT.as_bytes()).await
            .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;
    }

    let output = child.wait_with_output().await
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;

    let response = if output.status.success() {
        println!("Script succeeded");
        DepositContainerKeyResponse {
            success: true,
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script failed: {}", stderr);
        DepositContainerKeyResponse {
            success: false,
        }
    };

    let response_string = serde_json::to_string::<DepositContainerKeyResponse>(&response)
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;

    let encrypted_message = encrypt_message("PubKeyDeposited", &diffie_hellman_key_copy, response_string)
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;
    
    let encrypted_message_str = serde_json::to_string(&encrypted_message)
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;

    let mut sender_guard = sender.lock().await;
    sender_guard.send(Message::Text(encrypted_message_str)).await
        .map_err(|e| ClientError::DepositContainerKeyError(e.to_string()))?;

    Ok(())
}
