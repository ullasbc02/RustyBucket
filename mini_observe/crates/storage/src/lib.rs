use std::{fs::File, path::{Path, PathBuf}, sync::Arc};

use arrow::{
    array::{Int64Array, StringArray, TimestampMillisecondArray, Array},
    compute::cast,
    datatypes::{DataType, Field, Schema, TimeUnit},
    record_batch::RecordBatch,
};
use chrono::{Datelike, Timelike};
use miniobserve_common::{LogRecord, MiniObserveError, Result};
use parquet::{
    arrow::ArrowWriter,
    basic::Compression,
    file::properties::WriterProperties,
};
use uuid::Uuid;

pub struct ParquetStorage {
    base_dir: PathBuf,
}

impl ParquetStorage {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    pub fn write_logs(&self, records: &[LogRecord]) -> Result<PathBuf> {
        if records.is_empty() {
            return Err(MiniObserveError::Storage(
                "cannot write empty log batch".to_string(),
            ));
        }

        let first = &records[0];

        let partition_dir = self
            .base_dir
            .join(format!("org_id={}", first.org_id))
            .join(format!("stream_name={}", first.stream_name))
            .join(format!("year={}", first.timestamp.year()))
            .join(format!("month={:02}", first.timestamp.month()))
            .join(format!("day={:02}", first.timestamp.day()))
            .join(format!("hour={:02}", first.timestamp.hour()));

        std::fs::create_dir_all(&partition_dir)?;

        let file_path = partition_dir.join(format!("batch-{}.parquet", Uuid::new_v4()));

        let batch = records_to_batch(records)?;

        let file = File::create(&file_path)?;

        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))
            .map_err(|e| MiniObserveError::Storage(e.to_string()))?;

        writer
            .write(&batch)
            .map_err(|e| MiniObserveError::Storage(e.to_string()))?;

        writer
            .close()
            .map_err(|e| MiniObserveError::Storage(e.to_string()))?;

        Ok(file_path)
    }
}

fn records_to_batch(records: &[LogRecord]) -> Result<RecordBatch> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())),
            false,
        ),
        Field::new("org_id", DataType::Utf8, false),
        Field::new("stream_name", DataType::Utf8, false),
        Field::new("service", DataType::Utf8, false),
        Field::new("level", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
        Field::new("latency_ms", DataType::Int64, true),
        Field::new("trace_id", DataType::Utf8, true),
        Field::new("span_id", DataType::Utf8, true),
    ]));

    let ids: Vec<String> = records.iter().map(|r| r.id.to_string()).collect();

    let timestamps: Vec<i64> = records
        .iter()
        .map(|r| r.timestamp.timestamp_millis())
        .collect();

    let org_ids: Vec<&str> = records.iter().map(|r| r.org_id.as_str()).collect();
    let streams: Vec<&str> = records.iter().map(|r| r.stream_name.as_str()).collect();
    let services: Vec<&str> = records.iter().map(|r| r.service.as_str()).collect();
    let levels: Vec<&str> = records.iter().map(|r| r.level.as_str()).collect();
    let messages: Vec<&str> = records.iter().map(|r| r.message.as_str()).collect();

    let latencies: Vec<Option<i64>> = records.iter().map(|r| r.latency_ms).collect();

    let trace_ids: Vec<Option<&str>> = records
        .iter()
        .map(|r| r.trace_id.as_deref())
        .collect();

    let span_ids: Vec<Option<&str>> = records
        .iter()
        .map(|r| r.span_id.as_deref())
        .collect();

    // Create timestamp array and cast to UTC timezone to match schema
    let timestamps_no_tz: Arc<dyn Array> = Arc::new(TimestampMillisecondArray::from(timestamps));
    let ts_array = cast(
        &timestamps_no_tz,
        &DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())),
    )
    .map_err(|e| MiniObserveError::Storage(e.to_string()))?;

    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(StringArray::from(ids)),
            ts_array,
            Arc::new(StringArray::from(org_ids)),
            Arc::new(StringArray::from(streams)),
            Arc::new(StringArray::from(services)),
            Arc::new(StringArray::from(levels)),
            Arc::new(StringArray::from(messages)),
            Arc::new(Int64Array::from(latencies)),
            Arc::new(StringArray::from(trace_ids)),
            Arc::new(StringArray::from(span_ids)),
        ],
    )
    .map_err(|e| MiniObserveError::Storage(e.to_string()))
}