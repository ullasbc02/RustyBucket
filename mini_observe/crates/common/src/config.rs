use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub wal_dir: PathBuf,
    pub parquet_dir: PathBuf,
    pub batch_size: usize,
    pub flush_interval_secs: u64,
    pub channel_capacity: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            wal_dir: PathBuf::from("data/wal"),
            parquet_dir: PathBuf::from("data/parquet"),
            batch_size: 1000,
            flush_interval_secs: 5,
            channel_capacity: 10_000,
        }
    }
}