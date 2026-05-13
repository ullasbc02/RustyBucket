use miniobserve_common::{AppConfig, IngestLogRequest, LogRecord, MiniObserveError, Result};
use miniobserve_storage::ParquetStorage;
use miniobserve_wal::Wal;
use std::sync::Arc;
use tokio::{
    sync::mpsc,
    time::{self, Duration},
};
use tracing::{error, info};

#[derive(Clone)]
pub struct IngestHandle {
    sender: mpsc::Sender<LogRecord>,
    wal: Arc<Wal>,
}

impl IngestHandle {
    pub async fn ingest_log(&self, req: IngestLogRequest) -> Result<()> {
        let record: LogRecord = req.into();

        self.wal.append(&record).await?;

        self.sender
            .send(record)
            .await
            .map_err(|_| MiniObserveError::Internal("ingestion channel closed".to_string()))?;

        Ok(())
    }
}

pub async fn start_ingestion_pipeline(config: AppConfig) -> Result<IngestHandle> {
    let wal = Arc::new(Wal::new(&config.wal_dir, config.wal_segment_max_bytes).await?);
    let storage = Arc::new(ParquetStorage::new(&config.parquet_dir));

    let (sender, receiver) = mpsc::channel(config.channel_capacity);

    let recovered = wal.replay_uncommitted().await?;
    if !recovered.is_empty() {
        info!("recovered {} records from WAL", recovered.len());

        for record in recovered {
            sender
                .send(record)
                .await
                .map_err(|_| MiniObserveError::Internal("failed to enqueue recovered WAL record".to_string()))?;
        }
    }

    tokio::spawn(batch_worker(
        receiver,
        storage,
        wal.clone(),
        config.batch_size,
        Duration::from_secs(config.flush_interval_secs),
    ));

    Ok(IngestHandle { sender, wal })
}

async fn batch_worker(
    mut receiver: mpsc::Receiver<LogRecord>,
    storage: Arc<ParquetStorage>,
    wal: Arc<Wal>,
    batch_size: usize,
    flush_interval: Duration,
) {
    let mut batch = Vec::with_capacity(batch_size);
    let mut ticker = time::interval(flush_interval);

    loop {
        tokio::select! {
            maybe_record = receiver.recv() => {
                match maybe_record {
                    Some(record) => {
                        batch.push(record);

                        if batch.len() >= batch_size {
                            flush_batch(&mut batch, &storage, &wal).await;
                        }
                    }
                    None => {
                        if !batch.is_empty() {
                            flush_batch(&mut batch, &storage, &wal).await;
                        }
                        break;
                    }
                }
            }

            _ = ticker.tick() => {
                if !batch.is_empty() {
                    flush_batch(&mut batch, &storage, &wal).await;
                }
            }
        }
    }
}

async fn flush_batch(
    batch: &mut Vec<LogRecord>,
    storage: &ParquetStorage,
    wal: &Wal,
) {
    let records = std::mem::take(batch);

    match storage.write_logs(&records) {
        Ok(path) => {
            info!("flushed {} records to {:?}", records.len(), path);

            let ids: Vec<_> = records.iter().map(|r| r.id).collect();

            if let Err(err) = wal.mark_committed(&ids).await {
                error!("failed to mark WAL records committed: {:?}", err);
            }
        }
        Err(err) => {
            error!("failed to write parquet batch: {:?}", err);

            for record in records {
                batch.push(record);
            }
        }
    }
}