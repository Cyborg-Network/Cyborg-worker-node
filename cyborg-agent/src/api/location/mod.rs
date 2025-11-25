use serde::{Deserialize, Serialize};
use anyhow::{Error, Result};
use serde_json::Value;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct IpLocation {
    loc: Option<String>,
}

type Coordinates = (i32, i32);

#[derive(Serialize, Deserialize)]
pub struct Location {
    coordinates: Coordinates,
}

impl Location {
    pub async fn get_location() -> Result<Location>{
        // Try getting GPS location first
        if let Ok((lat, lon)) = get_gps_location() {
            Ok(Location{
                coordinates: f64_to_i32_coordinates(lat, lon)
            })
        } else if let Ok((lat, lon)) = get_ip_location().await {
            // Fallback to IP-based geolocation
            println!("Failed to get GPS location. Falling back to IP-based geolocation.");
            Ok(Location{
                coordinates: f64_to_i32_coordinates(lat, lon)
            })
        } else {
            Err(anyhow::anyhow!("Failed to get location"))
        }
    }
}

fn f64_to_i32_coordinates(lat: f64, lon: f64) -> Coordinates {
    let lat_i32 = (lat * 1_000_000.0).round() as i32;
    let lon_i32 = (lon * 1_000_000.0).round() as i32;

    (lat_i32, lon_i32)
}

fn get_gps_location() -> Result<(f64, f64), Error> {
    // Use gpspipe to get single GPS datum
    let output = Command::new("gpspipe")
        .arg("-w") 
        .arg("-n").arg("1")
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute gpspipe command"));
    }

    // Convert GPS data to string
    let gps_data = String::from_utf8_lossy(&output.stdout);
    println!("GPS data: {}", gps_data); // Debugging purposes

    let json: Value = serde_json::from_str(&gps_data)?;

    // Extract latitude and longitude (adjust based on the actual JSON structure)
    if let Some(lat) = json["lat"].as_f64() {
        if let Some(lon) = json["lon"].as_f64() {
            return Ok((lat, lon));
        }
    }

    Err(anyhow::anyhow!("Failed to extract GPS coordinates from JSON"))
}

async fn get_ip_location() -> Result<(f64, f64), Error> {
    let url = "https://ipinfo.io/json";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let ip_info: IpLocation = response.json().await?;

        let loc = ip_info.loc.ok_or_else(|| anyhow::anyhow!("Failed to get location via IP."))?;

        let loc_parts: Vec<&str> = loc.split(',').collect();
        
        if loc_parts.len() == 2 {
            let lat = loc_parts[0].parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse latitude"))?;
            let lon = loc_parts[1].parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse longitude"))?;

            return Ok((lat, lon));
        }

        Err(anyhow::anyhow!("Failed to get location via IP."))
    } else {
        Err(anyhow::anyhow!("Failed to get location via IP."))
    }
}