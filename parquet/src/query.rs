use arrow::array::{
    Int32Array,
    StringArray,
};

use arrow::record_batch::RecordBatch;

pub fn query_error_latency(
    batches: &[RecordBatch],
) {

    let mut total_latency = 0;
    let mut total_count = 0;

    println!();
    println!("Error services:");

    for batch in batches {

        let service_array = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        let latency_array = batch
            .column(1)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();

        let status_array = batch
            .column(2)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap();

        for i in 0..batch.num_rows() {

            if status_array.value(i) == 500 {

                println!(
                    "{} -> latency {}",
                    service_array.value(i),
                    latency_array.value(i),
                );

                total_latency +=
                    latency_array.value(i);

                total_count += 1;
            }
        }
    }

    println!();

    let avg =
        total_latency as f64
        / total_count as f64;

    println!(
        "Average error latency: {:.2}",
        avg
    );
}