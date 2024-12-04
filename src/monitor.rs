use sysinfo::{
    System, SystemExt, ProcessExt, CpuExt, DiskExt, 
    ComponentExt, NetworkExt, PidExt
};
use log::{warn, error};
use std::time::SystemTime;
use std::collections::VecDeque;
use crate::config::{MEMORY_WARNING_THRESHOLD, MEMORY_CRITICAL_THRESHOLD};
use serde::Serialize;

const HISTORY_SIZE: usize = 100; // 保存最近100个数据点

#[derive(Debug)]
pub struct MetricsHistory {
    cpu_history: VecDeque<(SystemTime, f32)>,
    memory_history: VecDeque<(SystemTime, f64)>,
}

impl MetricsHistory {
    pub fn new() -> Self {
        Self {
            cpu_history: VecDeque::with_capacity(HISTORY_SIZE),
            memory_history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn add_metrics(&mut self, metrics: &DetailedMetrics) {
        let now = SystemTime::now();
        
        self.cpu_history.push_back((now, metrics.basic.cpu_usage));
        if self.cpu_history.len() > HISTORY_SIZE {
            self.cpu_history.pop_front();
        }

        let memory_usage = calculate_memory_percentage(
            metrics.basic.used_memory,
            metrics.basic.total_memory
        );
        self.memory_history.push_back((now, memory_usage));
        if self.memory_history.len() > HISTORY_SIZE {
            self.memory_history.pop_front();
        }
    }

    pub fn get_cpu_data(&self) -> Vec<(f64, f64)> {
        self.cpu_history
            .iter()
            .enumerate()
            .map(|(i, (_, usage))| (i as f64, *usage as f64))
            .collect()
    }

    pub fn get_memory_data(&self) -> Vec<(f64, f64)> {
        self.memory_history
            .iter()
            .enumerate()
            .map(|(i, (_, usage))| (i as f64, *usage))
            .collect()
    }
}

#[derive(Debug)]
enum MemoryUsageState {
    Critical(f64),
    Warning(f64),
    Normal,
}

impl From<f64> for MemoryUsageState {
    fn from(usage: f64) -> Self {
        match usage {
            u if u >= MEMORY_CRITICAL_THRESHOLD => Self::Critical(u),
            u if u >= MEMORY_WARNING_THRESHOLD => Self::Warning(u),
            _ => Self::Normal,
        }
    }
}

pub fn check_memory_usage(usage_percentage: f64) {
    match MemoryUsageState::from(usage_percentage) {
        MemoryUsageState::Critical(usage) => {
            error!(
                "Memory usage rate reached dangerous level: {:.1}%", 
                usage
            );
        }
        MemoryUsageState::Warning(usage) => {
            warn!(
                "Memory usage rate is high: {:.1}%", 
                usage
            );
        }
        MemoryUsageState::Normal => {
            // Memory usage rate is normal
        }
    }
}

pub fn get_top_processes(sys: &System, num_processes: usize) -> Vec<(&sysinfo::Pid, &sysinfo::Process)> {
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    processes.into_iter().take(num_processes).collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub load_average: LoadAvgWrapper,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadAvgWrapper {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

impl From<sysinfo::LoadAvg> for LoadAvgWrapper {
    fn from(load: sysinfo::LoadAvg) -> Self {
        Self {
            one: load.one,
            five: load.five,
            fifteen: load.fifteen,
        }
    }
}

pub fn get_system_metrics(sys: &mut System) -> SystemMetrics {
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let load_average = sys.load_average().into();

    SystemMetrics {
        cpu_usage,
        total_memory,
        used_memory,
        available_memory,
        load_average,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailedMetrics {
    pub basic: SystemMetrics,
    pub network: NetworkMetrics,
    pub processes: Vec<ProcessMetrics>,
    pub temperatures: Vec<Temperature>,
    pub disks: Vec<DiskMetrics>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub connections: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Temperature {
    pub label: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskMetrics {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

pub fn get_detailed_metrics(sys: &mut System) -> DetailedMetrics {
    // 先刷新所有数据
    sys.refresh_all();
    
    // 再获取各项指标
    let basic = get_system_metrics(sys);
    let network = get_network_metrics(sys);
    let processes = get_process_metrics(sys);
    let temperatures = get_temperature_metrics(sys);
    let disks = get_disk_metrics(sys);

    DetailedMetrics {
        basic,
        network,
        processes,
        temperatures,
        disks,
        timestamp: SystemTime::now(),
    }
}

fn get_disk_metrics(sys: &mut System) -> Vec<DiskMetrics> {
    sys.refresh_disks();  // 刷新磁盘数据
    sys.disks().iter().map(|disk| {
        #[cfg(target_os = "linux")]
        let (read_bytes, write_bytes) = {
            use std::fs::File;
            use std::io::Read;
            let mut buffer = String::new();
            if let Ok(mut file) = File::open("/proc/diskstats") {
                if file.read_to_string(&mut buffer).is_ok() {
                    // Parse disk stats
                    // TODO: 实现具体的解析逻辑
                    (0, 0)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        };

        #[cfg(not(target_os = "linux"))]
        let (read_bytes, write_bytes) = (0, 0);

        DiskMetrics {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            read_bytes,
            write_bytes,
        }
    }).collect()
}

fn get_process_metrics(sys: &mut System) -> Vec<ProcessMetrics> {
    sys.refresh_processes();

    sys.processes()
        .iter()
        .map(|(_, process)| ProcessMetrics {
            pid: process.pid().as_u32(),
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        })
        .collect()
}

fn calculate_memory_percentage(used: u64, total: u64) -> f64 {
    (used as f64 / total as f64) * 100.0
}

fn get_network_metrics(sys: &mut System) -> NetworkMetrics {
    sys.refresh_networks();
    
    let mut rx_bytes = 0;
    let mut tx_bytes = 0;
    let mut connections = 0;

    for (_interface_name, data) in sys.networks() {
        rx_bytes += data.total_received();
        tx_bytes += data.total_transmitted();
        connections += 1;
    }

    NetworkMetrics {
        rx_bytes,
        tx_bytes,
        connections,
    }
}

fn get_temperature_metrics(sys: &mut System) -> Vec<Temperature> {
    sys.components()
        .iter()
        .map(|component| Temperature {
            label: component.label().to_string(),
            value: component.temperature(),
        })
        .collect()
} 