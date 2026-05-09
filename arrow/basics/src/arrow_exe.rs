use std::collections::HashMap;
use std::sync::Arc;

use arrow::compute::{
    sum,
    filter,
};
use arrow::compute::kernels::cmp::eq;

use arrow::array::{
    Int32Array,
    StringArray,
};

use arrow::array::Array;

use arrow::datatypes::{
    DataType,
    Field,
    Schema,
};

use arrow::record_batch::RecordBatch;
pub fn main(){
    println!();
    println!("===============================");
    println!("===============================");
    println!("Running Arrow Basics executable...");
    
    println!("Creating Arrow arrays...");

    // -----------------------------
    // Step 1: Create column arrays
    // -----------------------------

    let service_array = StringArray::from(vec![
        "auth",
        "payments",
        "search",
    ]);

    let latency_array = Int32Array::from(vec![
        100,
        250,
        120,
    ]);

    let status_array = Int32Array::from(vec![
        200,
        500,
        200,
    ]);

    println!("Arrays created");

    // -----------------------------
    // Step 2: Create schema
    // -----------------------------

    let schema = Schema::new(vec![
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
    ]);

    println!("Schema created");

    // -----------------------------
    // Step 3: Create RecordBatch
    // -----------------------------

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(service_array),
            Arc::new(latency_array),
            Arc::new(status_array),
        ],
    )
    .unwrap();

    println!("RecordBatch created");

    // -----------------------------
    // Step 4: Inspect batch
    // -----------------------------

    println!();
    println!("===== BATCH INFO =====");

    println!("{:?}", batch.schema());

    println!("Rows: {}", batch.num_rows());
    println!("Columns: {}", batch.num_columns());

    // -----------------------------
    // Step 5: Compute avg latency
    // -----------------------------

    let latency_array = batch
        .column(1)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap();

    // let mut total = 0;

    // for i in 0..latency_array.len() {

    //     total += latency_array.value(i);
    // }
    let total = sum(latency_array).unwrap();

    let avg =
        total as f64
        / latency_array.len() as f64;

    println!();
    println!("Average latency: {}", avg);

    // -----------------------------
    // Step 6: Filter errors
    // -----------------------------

    let service_array = batch
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>() // returns generic array, we need to downcast to the actual type
        .unwrap();

    let status_array = batch
        .column(2)
        .as_any()
        .downcast_ref::<Int32Array>()
        .unwrap();

    println!();
    println!("Error services:");

    // for i in 0..status_array.len() {

    //     if status_array.value(i) == 500 {

    //         println!(
    //             "{}",
    //             service_array.value(i)
    //         );
    //     }
    // }
    let mask = eq(status_array, &Int32Array::from(vec![500; status_array.len()])).unwrap(); // creates a boolean array where true indicates status == 500 [false, true, false]
    
    let filtered_services = filter(service_array, &mask).unwrap(); // filters the service array using the mask, returns ["payments"]
    
    println!("{:?}", filtered_services);
    // -----------------------------
    // Step 7: Count services
    // -----------------------------

    let mut counts = HashMap::new();

    for i in 0..service_array.len() {

        let service =
            service_array.value(i);

        *counts.entry(service)
            .or_insert(0) += 1;
    }

    println!();
    println!("Service counts:");

    println!("{:?}", counts);
    
    
}