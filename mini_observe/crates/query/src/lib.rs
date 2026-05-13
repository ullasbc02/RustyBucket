use arrow::util::pretty::pretty_format_batches;
use datafusion::prelude::*;
use miniobserve_common::{MiniObserveError, Result};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct QueryEngine {
    parquet_dir: PathBuf,
}

impl QueryEngine {
    pub fn new(parquet_dir: impl AsRef<Path>) -> Self {
        Self {
            parquet_dir: parquet_dir.as_ref().to_path_buf(),
        }
    }

    pub async fn execute_sql(&self, sql: &str) -> Result<String> {
        let ctx = SessionContext::new();

        ctx.register_parquet(
            "logs",
            self.parquet_dir.to_string_lossy().as_ref(),
            ParquetReadOptions::default(),
        )
        .await
        .map_err(|e| MiniObserveError::Query(e.to_string()))?;

        let df = ctx
            .sql(sql)
            .await
            .map_err(|e| MiniObserveError::Query(e.to_string()))?;

        let batches = df
            .collect()
            .await
            .map_err(|e| MiniObserveError::Query(e.to_string()))?;

        let table = pretty_format_batches(&batches[..])
            .map_err(|e| MiniObserveError::Query(e.to_string()))?;

        Ok(table.to_string())
    }
}