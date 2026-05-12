use std::fs::File;
use std::sync::Arc;

use arrow::record_batch::RecordBatch;

use parquet::arrow::ArrowWriter;

pub fn write_parquet(
    path: &str,
    batches: &[RecordBatch],
) {

    let file = File::create(path)
        .unwrap();

    let schema = batches[0].schema();

    let mut writer = ArrowWriter::try_new(
        file,
        schema,
        None,
    )
    .unwrap();

    for batch in batches {

        writer.write(batch)
            .unwrap();
    }

    writer.close()
        .unwrap();

    println!(
        "Parquet file written: {}",
        path
    );
}