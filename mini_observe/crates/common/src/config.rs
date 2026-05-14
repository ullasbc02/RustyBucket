use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub wal_dir: PathBuf,
    pub wal_segment_max_bytes: u64,
    pub parquet_dir: PathBuf,
    pub batch_size: usize,
    pub flush_interval_secs: u64,
    pub channel_capacity: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            wal_dir: PathBuf::from("data/wal"),
            wal_segment_max_bytes: 10 * 1024 * 1024,
            parquet_dir: PathBuf::from("data/parquet"),
            batch_size: 1000,
            flush_interval_secs: 5,
            channel_capacity: 10_000,
        }
    }
}