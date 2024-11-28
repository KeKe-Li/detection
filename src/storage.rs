use rusqlite::{Connection, Result};
use crate::monitor::SystemMetrics;

pub struct MetricsStorage {
    conn: Connection,
}

impl MetricsStorage {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("metrics.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS metrics (
                timestamp INTEGER PRIMARY KEY,
                cpu_usage REAL,
                memory_used INTEGER,
                memory_total INTEGER
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn store_metrics(&self, metrics: &SystemMetrics) -> Result<()> {
        self.conn.execute(
            "INSERT INTO metrics VALUES (?1, ?2, ?3, ?4)",
            [
                chrono::Utc::now().timestamp(),
                (metrics.cpu_usage * 100.0) as i64,
                metrics.used_memory as i64,
                metrics.total_memory as i64,
            ],
        )?;
        Ok(())
    }
} 