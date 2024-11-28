use clap::Parser;

/// System resource monitoring tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Config file path
    #[clap(short, long, default_value = "config.json")]
    pub config: String,

    /// Log level (debug, info, warn, error)
    #[clap(short, long)]
    pub log_level: Option<String>,

    /// Update interval in seconds
    #[clap(short, long)]
    pub interval: Option<u64>,
} 