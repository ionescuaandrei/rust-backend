use std::process;

use sysinfo::{self, Disks};

pub async fn init() -> sysinfo::System {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_all();

    sys
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]

pub enum Kind {
    System,
    Process,
    Memory,
    Cpu,
    Disk,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct System {
    name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    uptime: u64,
}

impl System {
    pub fn generate() -> Self {
        // TODO: folosit sysinfo
        Self{
            name: sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            os_version: sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            uptime: sysinfo::System::uptime()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]

pub struct Process {
    pid: u32,
    name: String,
    memory: u64,
    cpu_usage: f32,
    run_time: u64,
}

impl Process {
    pub fn generate(sys: &mut sysinfo::System) -> Vec<Self> {
        // let mut ret = vec![];
        // for (pid, process) in sys.processes() {
        //     let pid = process.pid().as_u32();
        //     let name = process.name().to_string_lossy().into_owned();
        //     let memory = process.memory();
        //     let cpu_usage = process.cpu_usage();
        //     let run_time = process.run_time();
            
        //     let process = Process {
        //         pid,
        //         name,
        //         memory,
        //         cpu_usage,
        //         run_time,
        //     };
        //     ret.push(process);
        // }
        // ret

        sys.processes().iter().map(|(pid, process)| {
            let pid = pid.as_u32();
            let name = process.name().to_string_lossy().into_owned();
            let memory = process.memory();
            let cpu_usage = process.cpu_usage();
            let run_time = process.run_time();

            Process {
                pid,
                name,
                memory,
                cpu_usage,
                run_time,
            }
        }).collect()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Memory {
    used: u64,
    total: u64,
}

impl Memory {
    pub fn generate(sys: &mut sysinfo::System) -> Self {
        Self {
            used: sys.used_memory(),
            total: sys.total_memory(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoreMetrics {
    name: String,
    brand: String,
    usage: f32,
    frequency: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cpu {
    cpu_usage: f32,
    cores: Vec<CoreMetrics>,
}

impl Cpu {
    pub fn generate(sys: &mut sysinfo::System) -> Self {
        Self{
            cpu_usage: sys.global_cpu_usage(),
            cores: sys.cpus().iter().map(|cpu| CoreMetrics{
                name: cpu.name().into(),
                brand: cpu.brand().into(),
                usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            }).collect()
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Disk {
    name: String,
    available_space: u64,
    total_space: u64,
    is_removable: bool,
}

impl Disk {
    pub fn generate() -> Vec<Self> {
        Disks::new_with_refreshed_list().iter().map(|disk| Disk {
                name: disk.name().to_str().unwrap().to_string(),
                available_space: disk.available_space(),
                total_space: disk.total_space(),
                is_removable: disk.is_removable(),
        }).collect()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Summary {
    system: System,
    process: Vec<Process>,
    memory: Memory,
    cpu: Cpu,
    disk: Vec<Disk>,
}

impl Summary {
    pub fn generate(sys: &mut sysinfo::System) -> Self {
        Self {
            system: System::generate(),
            process: Process::generate(sys),
            memory: Memory::generate(sys),
            cpu: Cpu::generate(sys),
            disk: Disk::generate(),
        }
    }
}