use std::sync::Arc;

use arrow::array::{ArrayRef, Int32Array};
use arrow::compute::filter;
use arrow::compute::kernels::cmp::eq;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;

pub fn filter_status_500(batch: &RecordBatch) -> RecordBatch {
    let status_array = batch
        .column(2)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap();

    let status_500_array = Int32Array::from(vec![500; status_array.len()]);

    let mask = eq(status_array, &status_500_array).unwrap();

    // println!("Filter mask for status = 500: {:?}", mask);

    let filtered_columns: Vec<ArrayRef> = batch
        .columns()
        .iter()
        .map(|column| filter(column.as_ref(), &mask).unwrap())
        .collect();

    // println!("Filtered batch with status = 500: {:?}", filtered_columns);
    RecordBatch::try_new(batch.schema(), filtered_columns).unwrap()
}

pub fn project_latency(batch: &RecordBatch) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "latency",
        DataType::Int32,
        false,
    )]));

    let latency_column = batch.column(1).clone();

    RecordBatch::try_new(schema, vec![latency_column]).unwrap()
}

pub fn aggregate_avg_latency(batches: &[RecordBatch]) -> f64 {
    let mut total_latency = 0;
    let mut total_count = 0;

    for batch in batches {
        let latency_array = batch
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();

        for i in 0..latency_array.len() {
            total_latency += latency_array.value(i);
            total_count += 1;
        }
    }

    if total_count == 0 {
        return 0.0;
    }

    total_latency as f64 / total_count as f64
}