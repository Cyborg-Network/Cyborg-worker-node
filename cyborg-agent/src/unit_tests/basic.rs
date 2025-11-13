use anyhow::{anyhow, Context, Result};
use serde_json::Value;

pub async fn run(args: Vec<&'static str>) -> Result<Value> {
    use crate::client::build_command;
    let args = args.into_iter().map(|s| s.to_string()).collect();
    let command = build_command(args).context("Failed to build command")?;
    Ok(command
        .run(Value::Null)
        .await
        .map_err(|_| anyhow!("Error occured while running command"))?)
}
