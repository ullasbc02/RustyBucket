use std::sync::Arc;

use arrow::array::{
    Int32Array,
    StringArray,
};

use arrow::datatypes::{
    DataType,
    Field,
    Schema,
};

use arrow::record_batch::RecordBatch;

pub fn create_batches() -> Vec<RecordBatch> {

    let schema = Arc::new(
        Schema::new(vec![
            Field::new(
                "service",
                DataType::Utf8,
                false,
            ),
            Field::new(
                "latency",
                DataType::Int32,
                false,
            ),
            Field::new(
                "status",
                DataType::Int32,
                false,
            ),
        ])
    );

    let batch1 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(
                StringArray::from(vec![
                    "auth",
                    "payments",
                    "search",
                ])
            ),
            Arc::new(
                Int32Array::from(vec![
                    100,
                    250,
                    120,
                ])
            ),
            Arc::new(
                Int32Array::from(vec![
                    200,
                    500,
                    200,
                ])
            ),
        ],
    )
    .unwrap();

    let batch2 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(
                StringArray::from(vec![
                    "checkout",
                    "payments",
                    "auth",
                ])
            ),
            Arc::new(
                Int32Array::from(vec![
                    450,
                    300,
                    90,
                ])
            ),
            Arc::new(
                Int32Array::from(vec![
                    500,
                    500,
                    200,
                ])
            ),
        ],
    )
    .unwrap();

    vec![batch1, batch2]
}