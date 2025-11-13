use serde_json::Value;
use std::{
    fs, 
    process::{Command, Stdio}, 
    sync::Arc, 
    path::PathBuf, 
    io::{BufRead, BufReader}
};
use tokio::sync::Mutex;
//use home::home_dir;
use anyhow::{Context, Result}; // Importing anyhow for better error handling
use fs2::FileExt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::PATHS;

pub type LogsStorage = Arc<Mutex<Vec<String>>>;

// Read JSON map from a file and return the Value or an error
fn read_json_map(file_path: PathBuf) -> Result<Value> {
    let data = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;
    
    let json_map: Value = serde_json::from_str(&data)
        .with_context(|| "Failed to parse JSON data")?;
    
    Ok(json_map)
}

// Fetch the deployment name from the JSON map based on input
fn get_deployment_name_from_json_map(task_id: &str, json_map: &Value) -> Option<String> {
    if let Value::Object(map) = json_map {
        if let Some(value) = map.get(task_id) {
            if let Some(deployment_name) = value.as_str() {
                println!("Deployment name from map: {}", deployment_name);
                return Some(deployment_name.to_string());
            } else {
                println!("Value is not a string: {:?}", value);
            }
        } else {
            print!("Task not found in map: {}", task_id);
        }
    } else {
        println!("JSON map is not an object: {:?}", json_map);
    }
    None
}

pub fn read_logs() -> Result<String> {
    let mut file = File::open(&PATHS.logs)?;
    
    file.lock_shared()?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    file.unlock()?;
    Ok(content)
}

pub async fn aggregate_new_logs(logs_storage: LogsStorage, task_id: u64) -> Result<(), Box<dyn std::error::Error>> {

    //let home_path = home_dir().context("Failed to get home directory")?;
    let file_path = PathBuf::from("/home/azureuser/Worker/deploymentsMap.json");
    let json_map = read_json_map(file_path)
        .context("Failed to read or parse the deploymentsMap.json file")?;
    
    let deployment_name = get_deployment_name_from_json_map(task_id.to_string().as_str(), &json_map)
        .context("Failed to get deployment name from JSON map")?;

    let mut process = Command::new("sudo")
        .arg("kubectl")
        .arg("logs")
        .arg("-l")
        .arg(format!("app={}", deployment_name))
        .arg("--all-containers=true")
        .arg("--follow") 
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute `kubectl logs -f` for deployment: {}", deployment_name))?;



    let stdout = process.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(log_line) = line {
            let mut logs = logs_storage.lock().await;
            logs.push(log_line);
        }
    }

    Ok(())
}

pub async fn retrieve_new_logs(logs_storage: LogsStorage) -> Vec<String> {
    let mut logs = logs_storage.lock().await;

    let new_logs = logs.clone();
    println!("Logs: {:?}", new_logs);
    logs.clear();

    new_logs
}