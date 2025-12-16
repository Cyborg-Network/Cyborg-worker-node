use anyhow::{Context, Result};
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use reqwest::Client;
use serde::Serialize;
use sysinfo::{CpuExt, NetworksExt, System, SystemExt};

mod memory;
mod storage;

#[derive(Serialize)]
pub struct Specs {
    cpus: Vec<String>,
    memory: u64,
    disk: u64,
    networks: Vec<String>,
    os: String,
    linux_version: String,
    kernel: String,
    serverhostname: String,
    csc_connected: bool,
    csc_version: String,
    ip: String,
}

impl Specs {
    pub async fn get_specs() -> Result<Specs> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut cpus = sys
            .cpus()
            .iter()
            .map(|cpu| cpu.brand().to_string())
            .collect::<Vec<_>>();
        cpus.sort_unstable();
        cpus.dedup();

        let _ = memory::get_memory().await?;

        let total_memory = memory::return_total_memory();
        let total_storage = storage::return_total_storage();

        let networks = sys
            .networks()
            .iter()
            .map(|(name, _networkdata)| name.to_string())
            .collect::<Vec<_>>();

        let os = sys.long_os_version().context("Failed to get OS version")?;
        let linux_version = sys.os_version().context("Failed to get short OS version")?;
        let kernel = sys
            .kernel_version()
            .context("Failed to get kernel version")?;
        let serverhostname = sys.host_name().context("Failed to get server hostname")?;
        // potential injection. Don't run eval on returned JSON

        let client = Client::new();

        let response = client
            .get("https://ifconfig.me")
            .send()
            .await
            .context("Failed to send request to get public IP")?;

        let ip = response
            .text()
            .await
            .context("Failed to get public IP address as string")?;

        Ok(Specs {
            cpus,
            memory: total_memory,
            disk: total_storage,
            networks,
            os,
            linux_version,
            kernel,
            serverhostname,
            csc_connected: true,
            csc_version: format!(
                "{}.{}.{}",
                pkg_version_major!(),
                pkg_version_minor!(),
                pkg_version_patch!()
            ),
            ip,
        })
    }
}
