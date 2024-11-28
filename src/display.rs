use sysinfo::{System, SystemExt, DiskExt, ProcessExt};
use tabled::{Table, Tabled};

pub fn print_memory_bar(usage_percentage: f64, _bar_width: usize) {
    let usage_bar = create_usage_bar(usage_percentage);
    println!("Memory Usage:\nUsage: [{}] {:.1}%", usage_bar, usage_percentage);
}

pub fn print_disk_info(sys: &System) {
    println!("Disk Usage:");
    println!("{:<15} {:<10} {:<10} {:<10} {:<10}", "Device Name", "Total", "Used", "Available", "Usage");
    println!("--------------------------------------------------------");
    for disk in sys.disks() {
        let total = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0); // GB
        let used = (disk.total_space() - disk.available_space()) as f64 / (1024.0 * 1024.0 * 1024.0); // GB
        let available = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0); // GB
        let usage_percentage = (used / total as f64) * 100.0;
        println!("{:<15} {:<10.2} GB {:<10.2} GB {:<10.2} GB {:<10.1}%", 
                 disk.mount_point().display(), total, used, available, usage_percentage);
    }
}

pub fn print_cpu_usage(cpu_usage: f32) {
    let usage_bar = create_usage_bar(cpu_usage as f64);
    println!("CPU Usage:\nUsage: [{}] {:.1}%", usage_bar, cpu_usage);
}

pub fn print_system_load(load_average: sysinfo::LoadAvg) {
    println!("System Load:");
    println!("1 min: {:.2}", load_average.one);
    println!("5 min: {:.2}", load_average.five);
    println!("15 min: {:.2}", load_average.fifteen);
}

#[derive(Tabled)]
struct ProcessInfo {
    pid: sysinfo::Pid,
    user: String,
    virt: String,
    res: String,
    cpu_usage: f32,
    mem_usage: f64,
    command: String,
    run_time: String,
}

pub fn print_processes(sys: &System) {
    let processes: Vec<_> = sys.processes().iter().collect();
    let mut process_info: Vec<ProcessInfo> = processes
        .iter()
        .map(|(&pid, process)| ProcessInfo {
            pid,
            user: process.user_id().map_or("Unknown".to_string(), |uid| uid.to_string()),
            virt: format_memory(process.virtual_memory()),
            res: format_memory(process.memory()),
            cpu_usage: process.cpu_usage(),
            mem_usage: (process.memory() as f64 / sys.total_memory() as f64) * 100.0,
            command: process.name().to_string(),
            run_time: format_run_time(process.run_time()),
        })
        .collect();

    // Sort by CPU usage
    process_info.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    // Print table
    let table = Table::new(process_info);
    println!("{}", table);
}


fn create_usage_bar(percentage: f64) -> String {
    let bar_width = 50; // Set the width of the bar chart
    let filled_length = (percentage / 100.0 * bar_width as f64).round() as usize;
    let empty_length = bar_width - filled_length;
    format!("{}{}", "=".repeat(filled_length), " ".repeat(empty_length))
}

fn format_memory(bytes: u64) -> String {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    format!("{:.1}G", gb)
}

fn format_run_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{}h{:02}m{:02}s", hours, minutes, seconds)
} 