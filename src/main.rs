use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use sysinfo::{System, SystemExt};
use log::info;

use memory_monitor::{
    logger,
    monitor,
    display,
    config::UPDATE_INTERVAL,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger(None, None)?;
    let mut sys = System::new_all();
    
    info!("Start monitoring system resources...");    
    loop {
        stdout().execute(Clear(ClearType::All))?.execute(MoveTo(0, 0))?;
        sys.refresh_all();
        
        let metrics = monitor::get_system_metrics(&sys);
        
        println!("System Resource Monitor (Updates every {} seconds)\n", UPDATE_INTERVAL);
        display::print_cpu_usage(&metrics);
        display::print_memory_bar(metrics.used_memory as f64 / metrics.total_memory as f64 * 100.0, 50);
        display::print_disk_info(&sys);
        display::print_system_load(&metrics);
        
        stdout().flush()?;
        thread::sleep(Duration::from_secs(UPDATE_INTERVAL));
    }
} 