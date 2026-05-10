use crate::data::create_log_batches;
use crate::operators::{
    aggregate_avg_latency,
    filter_status_500,
    project_latency,
};

pub fn run_query() {
    let batches = create_log_batches();

    println!("Input batches: {}", batches.len());

    println!();
    println!("===== SCAN OUTPUT =====");
    for batch in &batches {
        println!("{:?}", batch);
    }

    let filtered_batches: Vec<_> = batches
        .iter()
        .map(filter_status_500)
        .collect();

    println!();
    println!("===== AFTER FILTER status = 500 =====");
    for batch in &filtered_batches {
        println!("{:?}", batch);
    }

    let projected_batches: Vec<_> = filtered_batches
        .iter()
        .map(project_latency)
        .collect();

    println!();
    println!("===== AFTER PROJECTION latency only =====");
    for batch in &projected_batches {
        println!("{:?}", batch);
    }

    let avg_latency = aggregate_avg_latency(&projected_batches);

    println!();
    println!("===== FINAL RESULT =====");
    println!("Average latency where status = 500: {:.2}", avg_latency);
}