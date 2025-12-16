use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::{Command, Stdio};
use std::str;
use std::sync::Arc;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use reqwest::Client;

use types::substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType;
use types::MinerConfig;

use crate::error::Result;
use crate::global_config::CYBORG_MINER_DOMAIN_NAME;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GoogleGeoResponse {
    location: GoogleLocation,
    accuracy: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GoogleLocation {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct WifiAccessPoint {
    macAddress: String,
    signalStrength: i32,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
struct GeoRequest {
    considerIp: bool,
    wifiAccessPoints: Vec<WifiAccessPoint>,
}

#[derive(Deserialize, Debug)]
struct IpLocation {
    loc: Option<String>,
}

type Coordinates = (i32, i32);

#[derive(Serialize, Deserialize)]
pub struct Location {
    coordinates: Coordinates,
}

pub async fn gather_worker_spec(_miner_type: Arc<MinerType>) -> Result<MinerConfig> {
    let domain = &*CYBORG_MINER_DOMAIN_NAME;

    /*
    if current_id_has_changed {
        domain = new_domain_name;
        set_new_domain_name_in_systemd_environment(domain);
    }
    */

    let location = Location::get_location().await;

    let ram = return_total_memory();

    let cpu = get_cpu_cores();

    let storage = return_total_storage();

    Ok(MinerConfig {
        domain: domain.to_string(),
        latitude: location.coordinates.0,
        longitude: location.coordinates.1,
        ram,
        storage,
        cpu,
    })
}

fn get_cpu_cores() -> u16 {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.cpus().len() as u16
}

impl Location {
    pub async fn get_location() -> Location {
        // Try getting GPS location first
        if let Ok((lat, lon)) = get_gps_location() {
            return Location {
                coordinates: f64_to_i32_coordinates(lat, lon),
            };
        }

        // Try Google Wi-Fi Geolocation
        // if let Ok((lat, lon)) = get_geo_location().await {
        //     println!("Falling back to Google Wi-Fi Geolocation.");
        //     return Location {
        //         coordinates: f64_to_i32_coordinates(lat, lon),
        //     };
        // }

        match get_geo_location().await {
            Ok((lat, lon)) => {
                println!("Failed to get GPS location. Falling back to Wifi based geolocation.");
                println!("Longitude and Latitude are {} {}", lat, lon);
                Location {
                    coordinates: f64_to_i32_coordinates(lat, lon),
                }
            }
            Err(e) => {
                println!("Error getting WiFi based location: {}", e);
                if let Ok((lat, lon)) = get_ip_location().await {
                    Location {
                        coordinates: f64_to_i32_coordinates(lat, lon),
                    }
                } else {
                    panic!("Failed to get the location: {}", e);
                }
            }
        }
    }
}

fn f64_to_i32_coordinates(lat: f64, lon: f64) -> Coordinates {
    let lat_i32 = (lat * 1_000_000.00).round() as i32;
    let lon_i32 = (lon * 1_000_000.00).round() as i32;

    (lat_i32, lon_i32)
}

fn get_gps_location() -> Result<(f64, f64)> {
    // Use gpspipe to get single GPS datum
    let output = Command::new("gpspipe")
        .arg("-w")
        .arg("-n")
        .arg("1")
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute gpspipe".into());
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

    Err("Failed to extract latitude and longitude from GPS data".into())
}

async fn get_geo_location() -> Result<(f64, f64)> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,BSSID,SIGNAL", "dev", "wifi"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut wifi_list = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 {
            let bssid_parts = &parts[1..parts.len() - 1];
            let bssid = bssid_parts.join(":").replace("\\:", ":");

            let signal_str = parts.last().unwrap_or(&"0");
            let signal = signal_str.parse::<i32>().unwrap_or(0);

            wifi_list.push(WifiAccessPoint {
                macAddress: bssid.to_uppercase(),
                signalStrength: -signal,
            });
        }
    }

    if wifi_list.is_empty() {
        return Err("No Wi-Fi networks found".into());
    }

    let geo_request = GeoRequest {
        considerIp: true,
        wifiAccessPoints: wifi_list,
    };

    // let url = format!(
    //     "https://www.googleapis.com/geolocation/v1/geolocate?key={}",
    //     geo_api
    // );

    // let client = Client::new();
    // let resp: GoogleGeoResponse = client
    //     .post(&url)
    //     .json(&geo_request)
    //     .send()
    //     .await?
    //     .json()
    //     .await?;

    let url = "https://gpsproxy.taila87663.ts.net/geo";
    let client = Client::new();

    #[derive(serde::Deserialize)]
    struct LocationResponse {
        lat: f64,
        lon: f64,
    }

    let resp: LocationResponse = client
        .post(url)
        .json(&geo_request)
        .send()
        .await?
        .json()
        .await?;

    Ok((resp.lat, resp.lon))
}

async fn get_ip_location() -> Result<(f64, f64)> {
    let url = "https://ipinfo.io/json";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let ip_info: IpLocation = response.json().await?;

        let loc = ip_info.loc.ok_or("Failed to get location via IP.")?;

        let loc_parts: Vec<&str> = loc.split(',').collect();

        if loc_parts.len() == 2 {
            let lat = loc_parts[0]
                .parse::<f64>()
                .map_err(|_| "Failed to parse latitude")?;
            let lon = loc_parts[1]
                .parse::<f64>()
                .map_err(|_| "Failed to parse longitude")?;

            return Ok((lat, lon));
        }

        Err("Failed to get location via IP.".into())
    } else {
        Err("Failed to get location via IP.".into())
    }
}

#[allow(dead_code)]
pub async fn get_memory() -> Result<String> {
    let output = Command::new("free") // `ps` command...
        .arg("-h") // with argument `axww`...
        .stdout(Stdio::piped()) // of which we will pipe the output.
        .output() // Use output() instead of spawn() to automatically wait for the process
        .unwrap(); // and assert everything went right.

    let result = str::from_utf8(&output.stdout).unwrap();

    // Process the output to extract the memory information
    let lines: Vec<&str> = result.lines().collect();
    for line in lines {
        if line.to_lowercase().contains("mem") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                let res = parts[1]; // Get the total memory value
                return Ok(res.to_string());
            }
        }
    }

    Err("Failed to extract memory information".into())
}

pub fn return_total_memory() -> u64 {
    let mut system =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::new()));
    system.refresh_memory();

    system.total_memory()
}

//disk space needs to be retrieved like this, because the rust crates will return total diskspaces
//and are not filterable by mountpoint like this is, which causes issues on VPS
//this should only be a workaround until we have a better solution
pub fn return_total_storage() -> u64 {
    let output = Command::new("df")
        .arg("--block-size=1")
        .arg("--total")
        .arg("-B1")
        .output()
        .expect("Failed to execute command");

    // Print the raw command output for debugging
    println!(
        "Command output: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    let mut total_space: u64 = 0;

    for line in stdout.lines().skip(1) {
        // Skip the header line
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Check if the first column (filesystem) starts with "/dev/"
        if let Some(filesystem) = parts.first() {
            if filesystem.starts_with("/dev/") {
                if let Some(space) = parts.get(1) {
                    total_space += space.parse::<u64>().unwrap_or(0);
                }
            }
        }
    }

    // Print the total disk space found
    println!("Total disk space from /dev/: {}", total_space);
    total_space
}
