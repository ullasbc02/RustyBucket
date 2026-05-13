use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct IngestLogsRequest {
    pub records: Vec<IngestLogRequest>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalEntry {
    pub record: LogRecord,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub org_id: String,
    pub stream_name: String,
    pub service: String,
    pub level: String,
    pub message: String,
    pub latency_ms: Option<i64>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestLogRequest {
    pub timestamp: DateTime<Utc>,
    pub org_id: String,
    pub stream_name: String,
    pub service: String,
    pub level: String,
    pub message: String,
    pub latency_ms: Option<i64>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

impl From<IngestLogRequest> for LogRecord {
    fn from(req: IngestLogRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: req.timestamp,
            org_id: req.org_id,
            stream_name: req.stream_name,
            service: req.service,
            level: req.level,
            message: req.message,
            latency_ms: req.latency_ms,
            trace_id: req.trace_id,
            span_id: req.span_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryRequest {
    pub sql: String,
}