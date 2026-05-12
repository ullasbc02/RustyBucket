use std::fs::File;
use std::sync::Arc;

use arrow::record_batch::RecordBatch;

use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub fn read_parquet(
    path: &str,
) -> Vec<RecordBatch> {

    let file = File::open(path)
        .unwrap();

    let builder =
        ParquetRecordBatchReaderBuilder::try_new(file)
            .unwrap();

    let reader = builder
        .build()
        .unwrap();

    let mut batches = Vec::new();

    for batch in reader {

        batches.push(
            batch.unwrap()
        );
    }

    batches
}