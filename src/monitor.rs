use sysinfo::{System, SystemExt, ProcessExt, CpuExt};
use log::{warn, error};
use crate::config::{MEMORY_WARNING_THRESHOLD, MEMORY_CRITICAL_THRESHOLD};

#[derive(Debug)]
enum MemoryUsageState {
    Critical(f64),
    Warning(f64),
    Normal(f64),
}

impl From<f64> for MemoryUsageState {
    fn from(usage: f64) -> Self {
        match usage {
            u if u >= MEMORY_CRITICAL_THRESHOLD => Self::Critical(u),
            u if u >= MEMORY_WARNING_THRESHOLD => Self::Warning(u),
            u => Self::Normal(u),
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
        MemoryUsageState::Normal(_) => {
            // Memory usage rate is normal
        }
    }
}

pub fn get_top_processes(sys: &System, num_processes: usize) -> Vec<(&sysinfo::Pid, &sysinfo::Process)> {
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    processes.into_iter().take(num_processes).collect()
}

pub fn get_system_metrics(sys: &System) -> SystemMetrics {
    SystemMetrics {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        available_memory: sys.available_memory(),
        load_average: sys.load_average(),
    }
}
#[derive(Debug)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,
    pub load_average: sysinfo::LoadAvg,
}

#[derive(Debug)]
pub struct DetailedMetrics {
    pub basic: SystemMetrics,
    pub network: NetworkMetrics,
    pub processes: Vec<ProcessMetrics>,
    pub temperatures: Vec<Temperature>,
}

#[derive(Debug)]
pub struct NetworkMetrics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub connections: usize,
}

#[derive(Debug)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Debug)]
pub struct Temperature {
    pub label: String,
    pub value: f32,
} 