use miniobserve_common::{LogRecord, Result};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    sync::Mutex,
};
use uuid::Uuid;

pub struct Wal {
    dir: PathBuf,
    active_segment: Mutex<PathBuf>,
    committed_file: PathBuf,
    max_segment_bytes: u64,
    lock: Mutex<()>,
}

impl Wal {
    pub async fn new(dir: impl AsRef<Path>, max_segment_bytes: u64) -> Result<Self> {
        fs::create_dir_all(&dir).await?;

        let active_segment = dir.as_ref().join(new_segment_name());
        let committed_file = dir.as_ref().join("committed.ids");

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&active_segment)
            .await?;

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&committed_file)
            .await?;

        Ok(Self {
            dir: dir.as_ref().to_path_buf(),
            active_segment: Mutex::new(active_segment),
            committed_file,
            max_segment_bytes,
            lock: Mutex::new(()),
        })
    }

    pub async fn append(&self, record: &LogRecord) -> Result<()> {
        let _guard = self.lock.lock().await;

        let path = self.active_segment.lock().await.clone();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await?;

        let line = serde_json::to_string(record)?;

        file.write_all(line.as_bytes()).await?;
        file.write_all(b"\n").await?;
        file.flush().await?;

        self.rotate_if_needed(&path).await?;

        Ok(())
    }

    async fn rotate_if_needed(&self, current_path: &Path) -> Result<()> {
        let metadata = fs::metadata(current_path).await?;

        if metadata.len() < self.max_segment_bytes {
            return Ok(());
        }

        let new_path = self.dir.join(new_segment_name());

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&new_path)
            .await?;

        let mut active = self.active_segment.lock().await;
        *active = new_path;

        Ok(())
    }

    pub async fn mark_committed(&self, ids: &[Uuid]) -> Result<()> {
        let _guard = self.lock.lock().await;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.committed_file)
            .await?;

        for id in ids {
            file.write_all(id.to_string().as_bytes()).await?;
            file.write_all(b"\n").await?;
        }

        file.flush().await?;

        Ok(())
    }

    pub async fn replay_uncommitted(&self) -> Result<Vec<LogRecord>> {
        let committed = self.load_committed_ids().await?;
        let mut records = Vec::new();

        let mut entries = fs::read_dir(&self.dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if !is_segment_file(&path) {
                continue;
            }

            let content = fs::read_to_string(&path).await.unwrap_or_default();

            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                let record: LogRecord = serde_json::from_str(line)?;

                if !committed.contains(&record.id) {
                    records.push(record);
                }
            }
        }

        Ok(records)
    }

    async fn load_committed_ids(&self) -> Result<HashSet<Uuid>> {
        let content = fs::read_to_string(&self.committed_file)
            .await
            .unwrap_or_default();

        Ok(content
            .lines()
            .filter_map(|line| Uuid::parse_str(line).ok())
            .collect())
    }
}

fn new_segment_name() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("segment-{millis}.wal")
}

fn is_segment_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with("segment-") && name.ends_with(".wal"))
        .unwrap_or(false)
}
pub async fn cleanup_committed_segments(&self) -> Result<()> {
    let committed = self.load_committed_ids().await?;
    let active = self.active_segment.lock().await.clone();

    let mut entries = fs::read_dir(&self.dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if !is_segment_file(&path) {
            continue;
        }

        if path == active {
            continue;
        }

        let content = fs::read_to_string(&path).await.unwrap_or_default();

        let mut all_committed = true;
        let mut has_records = false;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            has_records = true;

            let record: LogRecord = serde_json::from_str(line)?;

            if !committed.contains(&record.id) {
                all_committed = false;
                break;
            }
        }

        if has_records && all_committed {
            fs::remove_file(&path).await?;
        }
    }

    Ok(())
}