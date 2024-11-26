use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    style::{Color, SetForegroundColor, ResetColor},
};
use sysinfo::{System, SystemExt, DiskExt};
use crate::utils::format_bytes;
use crate::monitor::SystemMetrics;
use crate::config::{MEMORY_WARNING_THRESHOLD, MEMORY_CRITICAL_THRESHOLD};

#[derive(Debug)]
enum UsageColor {
    Critical,
    Warning,
    Normal,
}

impl From<f64> for UsageColor {
    fn from(percentage: f64) -> Self {
        match percentage {
            p if p >= MEMORY_CRITICAL_THRESHOLD => Self::Critical,
            p if p >= MEMORY_WARNING_THRESHOLD => Self::Warning,
            _ => Self::Normal,
        }
    }
}

impl From<UsageColor> for Color {
    fn from(usage_color: UsageColor) -> Self {
        match usage_color {
            UsageColor::Critical => Color::Red,
            UsageColor::Warning => Color::Yellow,
            UsageColor::Normal => Color::Green,
        }
    }
}

pub fn print_memory_bar(percentage: f64, width: usize) {
    let filled_width = ((percentage / 100.0) * width as f64) as usize;
    let bar: String = format!(
        "[{}{}] {:.1}%",
        "=".repeat(filled_width),
        " ".repeat(width - filled_width),
        percentage
    );
    
    let color: Color = UsageColor::from(percentage).into();
    
    let mut handle = stdout();
    let _ = handle.execute(SetForegroundColor(color));
    println!("Usage: {}", bar);
    let _ = handle.execute(ResetColor);
}

pub fn print_disk_info(sys: &System) {
    println!("\nDisk Usage:");
    println!("Device Name      Total      Used     Available  Usage");
    println!("--------------------------------------------------------");
    
    for disk in sys.disks() {
        let total = disk.total_space();
        let used = total - disk.available_space();
        let available = disk.available_space();
        let usage_percent = (used as f64 / total as f64) * 100.0;
        
        println!("{:<15} {:>8} {:>8} {:>8} {:>7.1}%",
            disk.name().to_string_lossy(),
            format_bytes(total),
            format_bytes(used),
            format_bytes(available),
            usage_percent
        );
    }
}

pub fn print_cpu_usage(metrics: &SystemMetrics) {
    println!("\nCPU Usage:");
    print_memory_bar(metrics.cpu_usage as f64, 50);
}

pub fn print_system_load(metrics: &SystemMetrics) {
    println!("\nSystem Load:");
    println!("1 min: {:.2}", metrics.load_average.one);
    println!("5 min: {:.2}", metrics.load_average.five);
    println!("15 min: {:.2}", metrics.load_average.fifteen);
} 