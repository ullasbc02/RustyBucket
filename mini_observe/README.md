# mini_observe — Overview and Data Flow

This README documents the end-to-end data pipeline used by the mini_observe project: how logs are ingested, durably stored, and queried.

## High-level components

- `crates/api` — HTTP API (Axum) exposing endpoints: `/health`, `/api/logs`, `/api/logs/bulk`, `/api/query`.
- `crates/ingest` — Ingestion pipeline: converts requests to `LogRecord`, appends to WAL, and enqueues records for batching.
- `crates/wal` — Write-Ahead Log (WAL) for durability and crash recovery.
- `crates/storage` — Converts `LogRecord` batches to Arrow `RecordBatch` and writes partitioned Parquet files.
- `crates/query` — Query engine (DataFusion) that reads Parquet files and executes SQL queries.

## Ingest flow (POST `/api/logs/bulk`) — Stage-by-stage

1. Client POSTs JSON to `/api/logs/bulk`. Axum handler in `crates/api` deserializes to `IngestLogsRequest`.

2. For each `IngestLogRequest` the API calls `IngestHandle::ingest_log()` in `crates/ingest`.

   - `ingest_log()` converts `IngestLogRequest` → `LogRecord` (adds a new `Uuid`).
   - It immediately calls `Wal::append(&record)` (in `crates/wal`) which:
     - serializes the `LogRecord` as a JSON line and appends it to `segment-active.wal` (fsync/flush ensures durability).
   - After WAL append, `ingest_log()` sends the `LogRecord` into an async MPSC channel for background processing and returns `202 Accepted` to the client.

3. Background batch worker (spawned by `start_ingestion_pipeline`) listens on the channel and accumulates records in-memory.

   - The worker flushes the batch when either:
     - `batch.len() >= batch_size` (configured in `AppConfig`), or
     - a periodic `flush_interval` timer fires (e.g. 5s).

4. When flushing, the worker calls `ParquetStorage::write_logs(&records)` in `crates/storage`:

   - `records_to_batch()` transposes `Vec<LogRecord>` into an Arrow `RecordBatch` with the schema:
     - `id: Utf8`,
     - `timestamp: Timestamp(ms, "UTC")`,
     - `org_id: Utf8`,
     - `stream_name: Utf8`,
     - `service: Utf8`,
     - `level: Utf8`,
     - `message: Utf8`,
     - `latency_ms: Int64 (nullable)`,
     - `trace_id: Utf8 (nullable)`,
     - `span_id: Utf8 (nullable)`.

   - The storage code computes a partition path using the first record's `org_id`, `stream_name`, and `timestamp` (year/month/day/hour), creates the directory tree, and writes a Parquet file (Snappy compression) named like `batch-{uuid}.parquet`.

5. If the Parquet write succeeds, the batch worker calls `Wal::mark_committed(&ids)` to append committed UUIDs to `committed.ids`.

   - If the write fails, records are requeued in-memory so the worker can retry on the next flush.

6. On server startup, `Wal::replay_uncommitted()` is used to read `segment-active.wal` and return only those records whose UUIDs are not present in `committed.ids`, allowing the ingestion pipeline to re-enqueue and persist records that were not flushed before the crash.

## Query flow (POST `/api/query`) — How data is read back

1. Client POSTs `{"sql":"..."}` to `/api/query`. The API handler calls `QueryEngine::execute_sql()` in `crates/query`.

2. `QueryEngine` creates a `datafusion::SessionContext`, then registers the Parquet dataset:

   ```rust
   ctx.register_parquet("logs", parquet_dir, ParquetReadOptions::default()).await?;
   ```

   DataFusion uses Parquet metadata and the directory layout to discover files. Because files are partitioned by `org_id`, `stream_name`, `year`, `month`, `day`, `hour`, DataFusion can skip irrelevant partitions when the query filters by those columns.

3. The SQL is parsed and planned (`ctx.sql(sql)`), then executed. DataFusion reads the needed Parquet files, decodes columns into Arrow arrays, and executes the query plan (filter, projection, aggregation, etc.).

4. Results are returned as a list of Arrow `RecordBatch` objects. The API uses Arrow's `pretty_format_batches()` to format the result table before returning it to the client.

## Durability and Crash Recovery

- Every record is appended to the WAL before being enqueued for processing. The WAL contains each record as a JSON line in `segment-active.wal`.
- After a successful Parquet write, the worker writes the record IDs to `committed.ids`.
- On restart, the server calls `Wal::replay_uncommitted()` which returns only non-committed `LogRecord`s for reprocessing.

## Important files to inspect

- API: [crates/api/src/lib.rs](crates/api/src/lib.rs)
- Ingest: [crates/ingest/src/lib.rs](crates/ingest/src/lib.rs)
- WAL: [crates/wal/src/lib.rs](crates/wal/src/lib.rs)
- Storage: [crates/storage/src/lib.rs](crates/storage/src/lib.rs)
- Query engine: [crates/query/src/lib.rs](crates/query/src/lib.rs)

## Tips

- Use the WAL files for debugging: `segment-active.wal` contains JSON lines for every ingested record; `committed.ids` lists UUIDs that have been persisted.
- Parquet files are written under the configured `parquet_dir` in partitioned folders — use `parquet-tools` or `arrow` utilities to inspect.

---

## Running and Testing the Flow

### Prerequisites

- Rust toolchain (1.70+)
- `cargo` available in your PATH
- `curl` for testing HTTP endpoints

### Build the project

From the `mini_observe` directory:

```bash
cargo build -p miniobserve-server
```

This compiles all crates and produces the server binary.

### Run the server

```bash
cargo run -p miniobserve-server
```

The server will start on `http://localhost:3000` and listen for API requests. You should see output like:

```
Server running on 0.0.0.0:3000
```

The server creates `data/wal` and `data/parquet` directories in your working directory to store WAL segments and Parquet files.

### Test the ingest flow

In a separate terminal, ingest a batch of log records:

```bash
curl -X POST http://localhost:3000/api/logs/bulk \
  -H "Content-Type: application/json" \
  -d '{
    "records": [
      {
        "timestamp": "2026-05-12T10:00:00Z",
        "org_id": "default",
        "stream_name": "logs",
        "service": "payments",
        "level": "ERROR",
        "message": "payment failed",
        "latency_ms": 320,
        "trace_id": "trace-1",
        "span_id": "span-1"
      },
      {
        "timestamp": "2026-05-12T10:01:00Z",
        "org_id": "default",
        "stream_name": "logs",
        "service": "auth",
        "level": "INFO",
        "message": "login success",
        "latency_ms": 80,
        "trace_id": "trace-2",
        "span_id": "span-2"
      }
    ]
  }'
```

Expected response: `202 Accepted` — records are enqueued and will be persisted asynchronously.

### Verify WAL and Parquet files

After ingesting records, check that data was written:

```bash
# List WAL directory
ls -la data/wal/

# List Parquet directory structure
find data/parquet/ -type f -name "*.parquet"
```

You should see:
- `data/wal/segment-*.wal` files containing JSON lines
- `data/wal/committed.ids` with UUIDs of persisted records
- `data/parquet/org_id=default/stream_name=logs/year=2026/month=05/day=12/hour=10/batch-*.parquet` files

### Test the query flow

Query the ingested logs using SQL:

```bash
curl -X POST http://localhost:3000/api/query \
  -H "Content-Type: application/json" \
  -d '{
    "sql": "SELECT service, level, message FROM logs WHERE level = '\''ERROR'\''"
  }'
```

This should return the ERROR log from the payments service.

More complex queries:

```bash
# Count logs by service
curl -X POST http://localhost:3000/api/query \
  -H "Content-Type: application/json" \
  -d '{
    "sql": "SELECT service, COUNT(*) as count FROM logs GROUP BY service"
  }'

# Filter by timestamp range
curl -X POST http://localhost:3000/api/query \
  -H "Content-Type: application/json" \
  -d '{
    "sql": "SELECT * FROM logs WHERE timestamp >= '\''2026-05-12T10:00:00Z'\'' AND timestamp < '\''2026-05-12T11:00:00Z'\''"
  }'
```

### Test crash recovery

To verify the WAL-based recovery flow:

1. Start the server: `cargo run -p miniobserve-server`
2. Ingest some records using the bulk ingest command above
3. Kill the server (Ctrl+C) **before** the flush interval (5 seconds) completes
4. Check `data/wal/` to see uncommitted segments
5. Restart the server: `cargo run -p miniobserve-server`
6. The server will replay uncommitted records from the WAL on startup
7. Ingest more records and verify all records (old and new) are persisted

### Clean slate

To reset and start fresh:

```bash
rm -rf data/wal data/parquet
```

Then restart the server.

---

File created in this workspace: `mini_observe/README.md`
