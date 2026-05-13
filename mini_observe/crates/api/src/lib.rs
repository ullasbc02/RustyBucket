use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
// use miniobserve_common::{IngestLogRequest, MiniObserveError, QueryRequest};
use miniobserve_ingest::IngestHandle;
use miniobserve_query::QueryEngine;
use serde_json::json;
use miniobserve_common::{IngestLogRequest, IngestLogsRequest, MiniObserveError, QueryRequest};
#[derive(Clone)]
pub struct AppState {
    pub ingest: IngestHandle,
    pub query: QueryEngine,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/logs", post(ingest_log))
        .route("/api/query", post(query_logs))
        .route("/api/logs/bulk", post(ingest_logs_bulk))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

async fn ingest_log(
    State(state): State<AppState>,
    Json(payload): Json<IngestLogRequest>,
) -> Result<impl IntoResponse, ApiError> {
    state.ingest.ingest_log(payload).await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "accepted"
        })),
    ))
}

async fn query_logs(
    State(state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state.query.execute_sql(&payload.sql).await?;

    Ok(Json(json!({
        "result": result
    })))
}
async fn ingest_logs_bulk(
    State(state): State<AppState>,
    Json(payload): Json<IngestLogsRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let count = payload.records.len();

    for record in payload.records {
        state.ingest.ingest_log(record).await?;
    }

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "accepted",
            "records": count
        })),
    ))
}
struct ApiError(MiniObserveError);

impl From<MiniObserveError> for ApiError {
    fn from(err: MiniObserveError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            MiniObserveError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            MiniObserveError::Query(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.0.to_string()
        }));

        (status, body).into_response()
    }
}