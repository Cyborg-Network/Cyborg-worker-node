use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct HealthStatus {}

impl HealthStatus {
    //probably going to be async in the future, so implemented as async now
    pub async fn get_health_status() -> Result<bool> {
        // Worker Node needs to perform a health check and return the result here
        println!("Health check performed!");
        Ok(true)
    }
}