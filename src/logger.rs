use log::{LevelFilter, error};
use log4rs::{
    append::rolling_file::RollingFileAppender,
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use log4rs::append::rolling_file::policy::compound::{
    CompoundPolicy,
    trigger::size::SizeTrigger,
    roll::fixed_window::FixedWindowRoller,
};
use std::{fs, path::Path};

const DEFAULT_LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S.%3f)} [{t}] {l} - {m}\n";
const CONSOLE_LOG_PATTERN: &str = "{d(%H:%M:%S)} {l} {m}\n";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const LOG_DIR: &str = "logs";
const LOG_FILE_PATH: &str = "logs/memory_monitor.log";
const MAX_BACKUP_FILES: u32 = 5;

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    cleanup_old_logs()?;
    ensure_log_directory()?;

    let file_appender = create_file_appender()?;
    let console_appender = create_console_appender();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(Root::builder()
            .appender("file")
            .appender("console")
            .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;
    Ok(())
}

fn create_file_appender() -> Result<RollingFileAppender, Box<dyn std::error::Error>> {
    let fixed_window_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&format!("{}.{{}}.gz", LOG_FILE_PATH), MAX_BACKUP_FILES)?;
    
    let size_trigger = SizeTrigger::new(MAX_LOG_SIZE);
    let compound_policy = CompoundPolicy::new(
        Box::new(size_trigger),
        Box::new(fixed_window_roller),
    );

    Ok(RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(DEFAULT_LOG_PATTERN)))
        .append(true)
        .build(LOG_FILE_PATH, Box::new(compound_policy))?)
}

fn create_console_appender() -> ConsoleAppender {
    ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(CONSOLE_LOG_PATTERN)))
        .build()
}

fn ensure_log_directory() -> Result<(), std::io::Error> {
    if !Path::new(LOG_DIR).exists() {
        fs::create_dir_all(LOG_DIR)?;
    }
    Ok(())
}

fn cleanup_old_logs() -> Result<(), std::io::Error> {
    if Path::new(LOG_DIR).exists() {
        for entry in fs::read_dir(LOG_DIR)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "gz" {
                    if let Err(e) = fs::remove_file(&path) {
                        error!("Failed to remove old log file {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_logger_setup() {
        assert!(setup_logger().is_ok());
        info!("Test log message");
        assert!(Path::new(LOG_FILE_PATH).exists());
    }

    #[test]
    fn test_log_rotation() {
        if let Ok(()) = setup_logger() {
            // write more than 10MB to trigger log rotation
            for i in 0..10000 {
                info!("Test log message {}", i);
                if i % 1000 == 0 {
                    thread::sleep(Duration::from_millis(10));
                }
            }
            // check if backup files are created
            assert!(Path::new(&format!("{}.1.gz", LOG_FILE_PATH)).exists());
        }
    }
} 