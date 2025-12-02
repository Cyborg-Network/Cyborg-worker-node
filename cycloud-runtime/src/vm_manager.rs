use std::process::Command;
use std::path::{PathBuf};
use std::fs;
use std::io::Write;

use sysinfo::System;

use crate::TaskStatus;

const VM_NAME: &str = "cycloud";
const RESERVED_MEMORY_BYTES: u64 = 500 * 1024 * 1024;
const VM_IMAGE_PATH: &str = "/var/lib/libvirt/images/cycloud_base.qcow2";

pub struct VmManager {
    vm_image_path: PathBuf,
}

impl VmManager {
    pub fn new() -> Self {

        Self { vm_image_path: PathBuf::from(VM_IMAGE_PATH) }
    }

    fn get_host_cpu_count(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let output = Command::new("nproc")
            .output()?;

        if !output.status.success() {
            return Err("Failed to get CPU count".into());
        }

        let cpu_count = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<usize>()?;

        Ok(cpu_count)
    }

    fn calculate_optimal_vcpus(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let host_cpus = self.get_host_cpu_count()?;

        let vcpus = match host_cpus {
            1..=2 => 1,
            3..=4 => host_cpus - 1,
            5..=8 => host_cpus - 2,
            9..=16 => host_cpus - 2,
            _ => ((host_cpus as f32 * 0.875) as usize).max(host_cpus - 4),
        };

        println!("Host CPUs: {}, allocating {} vCPUs to VM", host_cpus, vcpus);
        Ok(vcpus as u32)
    }

    fn get_available_memory_mb(&self) -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();

        sys.available_memory()
    }

    fn calculate_optimal_mem(&self) -> u64 {
        self.get_available_memory_mb() - RESERVED_MEMORY_BYTES / 1024 / 1024
    }

    fn get_available_space_gb(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let parent = self.vm_image_path
            .parent()
            .ok_or("Invalid image path")?;
        
        let output = Command::new("df")
            .arg("-BG")
            .arg("--output=avail")
            .arg(parent)
            .output()?;

        if !output.status.success() {
            return Err("Failed to get disk space".into());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let available_gb = output_str
            .lines()
            .nth(1)
            .and_then(|line| line.trim().trim_end_matches('G').parse::<u64>().ok())
            .ok_or("Failed to parse available space")?;

        Ok(available_gb)
    }

    fn calculate_optimal_size(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let available = self.get_available_space_gb()?;
        
        if available <= 10 {
            return Err("Not enough disk space (need at least 10GB free)".into());
        }

        let optimal_size = available - 10;
        println!("Available space: {}GB, allocating {}GB to VM (10GB overhead)", available, optimal_size);
        
        Ok(optimal_size)
    }

    pub fn ensure_vm_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.vm_image_path.exists() {
            println!("VM image already exists at: {}", self.vm_image_path.display());
            return Ok(());
        }

        println!("Creating new Ubuntu Jammy VM image...");

        if let Some(parent) = self.vm_image_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let cloud_img_url = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img";
        let temp_img = format!("{}.tmp", self.vm_image_path.display());

        println!("Downloading Ubuntu Jammy cloud image...");
        let output = Command::new("wget")
            .arg("-O")
            .arg(&temp_img)
            .arg(cloud_img_url)
            .output()?;

        if !output.status.success() {
            fs::remove_file(&temp_img).ok();
            return Err(format!(
                "Failed to download image: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }

        println!("Converting and resizing image...");
        let output = Command::new("qemu-img")
            .arg("convert")
            .arg("-f")
            .arg("qcow2")
            .arg("-O")
            .arg("qcow2")
            .arg(&temp_img)
            .arg(&self.vm_image_path)
            .output()?;

        if !output.status.success() {
            fs::remove_file(&temp_img).ok();
            return Err(format!(
                "Failed to convert image: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }
        
        let optimal_size = self.calculate_optimal_size()?;

        let output = Command::new("qemu-img")
            .arg("resize")
            .arg(&self.vm_image_path)
            .arg(format!("{}G", optimal_size))
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to resize image: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }

        fs::remove_file(&temp_img).ok();

        println!("VM image created successfully at: {}", self.vm_image_path.display());
        Ok(())
    }

    fn generate_vm_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let optimal_vcpus = self.calculate_optimal_vcpus()?;

        Ok(format!(
            r#"<domain type='kvm'>
  <name>{}</name>
  <memory unit='MiB'>{}</memory>
  <vcpu placement='static'>{}</vcpu>
  <os>
    <type arch='x86_64' machine='pc'>hvm</type>
    <boot dev='hd'/>
  </os>
  <devices>
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2'/>
      <source file='{}'/>
      <target dev='vda' bus='virtio'/>
    </disk>
    <interface type='network'>
      <source network='default'/>
      <model type='virtio'/>
    </interface>
    <console type='pty'>
      <target type='serial' port='0'/>
    </console>
    <graphics type='vnc' port='-1' autoport='yes'/>
  </devices>
</domain>"#,
            VM_NAME,
            self.calculate_optimal_mem(),
            optimal_vcpus,
            self.vm_image_path.display()
        ))
    }

    pub fn setup_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let xml_config = self.generate_vm_xml()?;
        let xml_path = format!("/tmp/{}_config.xml", VM_NAME);
        
        let mut file = fs::File::create(&xml_path)?;
        file.write_all(xml_config.as_bytes())?;

        let output = Command::new("virsh")
            .arg("define")
            .arg(&xml_path)
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to define VM: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let output = Command::new("virsh")
            .arg("start")
            .arg(VM_NAME)
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to start VM: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        std::thread::sleep(std::time::Duration::from_secs(10));

        fs::remove_file(&xml_path).ok();

        Ok(())
    }

    pub fn stop_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("virsh")
            .arg("shutdown")
            .arg(VM_NAME)
            .output()?;

        if !output.status.success() {
            let output = Command::new("virsh")
                .arg("destroy")
                .arg(VM_NAME)
                .output()?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to stop VM: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
                .into());
            }
        }

        Ok(())
    }

    pub fn restart_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_impl()?;
        
        std::thread::sleep(std::time::Duration::from_secs(3));

        let output = Command::new("virsh")
            .arg("start")
            .arg(VM_NAME)
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to restart VM: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(())
    }

    pub fn cleanup_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("virsh")
            .arg("destroy")
            .arg(VM_NAME)
            .output()
            .ok();

        let output = Command::new("virsh")
            .arg("undefine")
            .arg(VM_NAME)
            .arg("--remove-all-storage")
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Failed to cleanup VM: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(())
    }

    pub fn status_impl(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        let output = Command::new("virsh")
            .arg("domstate")
            .arg(VM_NAME)
            .output()?;

        if !output.status.success() {
            // VM doesn't exist - could be cleaned or never created
            return Ok(TaskStatus::Stopped);
        }

        let state = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
        
        match state.as_str() {
            "running" => Ok(TaskStatus::Running),
            "shut off" | "shutoff" => Ok(TaskStatus::Stopped),
            "paused" | "suspended" => Ok(TaskStatus::Stopped),
            "in shutdown" | "shutdown" => Ok(TaskStatus::Stopped),
            "crashed" | "dying" => Ok(TaskStatus::Broken),
            "pmsuspended" => Ok(TaskStatus::Stopped),
            "idle" | "blocked" => Ok(TaskStatus::Running),
            _ => {
                eprintln!("Unknown VM state: {}", state);
                Ok(TaskStatus::Broken)
            }
        }
    }
}
