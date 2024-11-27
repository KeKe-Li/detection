use log::LevelFilter;
use log4rs::{
    append::rolling_file::RollingFileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use log4rs::append::rolling_file::policy::compound::{
    CompoundPolicy,
    trigger::size::SizeTrigger,
    roll::fixed_window::FixedWindowRoller,
};
use std::path::PathBuf;

const DEFAULT_LOG_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} - {l} - {m}\n";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB

pub fn setup_logger(log_path: Option<PathBuf>, level: Option<LevelFilter>) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = log_path.unwrap_or_else(|| PathBuf::from("memory_monitor.log"));
    let log_level = level.unwrap_or(LevelFilter::Info);
    
    let window_size = 3; 
    let fixed_window_roller = FixedWindowRoller::builder()
        .build(&format!("{}.{{}}.gz", log_file.display()), window_size)?;
    
    let size_trigger = SizeTrigger::new(MAX_LOG_SIZE);
    let compound_policy = CompoundPolicy::new(
        Box::new(size_trigger),
        Box::new(fixed_window_roller),
    );

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(DEFAULT_LOG_PATTERN)))
        .build(log_file, Box::new(compound_policy))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(log_level)
        )?;

    log4rs::init_config(config)?;
    Ok(())
} 