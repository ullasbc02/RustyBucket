use datafusion::prelude::*;
use datafusion::dataframe::DataFrameWriteOptions;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();

    // 1. Register JSON logs
    ctx.register_json("raw_logs", "data/logs.json", JsonReadOptions::default())
        .await?;

    println!("Raw JSON logs:");
    ctx.sql("SELECT * FROM raw_logs").await?.show().await?;

    // 2. Convert JSON logs to Parquet
    let df = ctx.sql("SELECT * FROM raw_logs").await?;
    df.write_parquet("data/logs_parquet", DataFrameWriteOptions::new(), None)
        .await?;

    println!("\nJSON converted to Parquet.");

    // 3. Register Parquet as query table
    ctx.register_parquet("logs", "data/logs_parquet", ParquetReadOptions::default())
        .await?;

    // 4. Observability queries
    let queries = vec![
        (
            "Average latency for payments",
            "
            SELECT AVG(latency_ms) AS avg_latency
            FROM logs
            WHERE service = 'payments'
            ",
        ),
        (
            "Error count by service",
            "
            SELECT service, COUNT(*) AS error_count
            FROM logs
            WHERE level = 'ERROR'
            GROUP BY service
            ",
        ),
        (
            "P95 latency by service",
            "
            SELECT service, approx_percentile_cont(latency_ms, 0.95) AS p95_latency
            FROM logs
            GROUP BY service
            ",
        ),
        (
            "Show query plan",
            "
            EXPLAIN
            SELECT AVG(latency_ms)
            FROM logs
            WHERE service = 'payments'
            ",
        ),
    ];

    for (name, sql) in queries {
        println!("\n{name}");
        let df = ctx.sql(sql).await?;
        df.show().await?;
    }

    Ok(())
}