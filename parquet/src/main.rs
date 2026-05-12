mod data;
mod parquet_writer;
mod parquet_reader;
mod query;

use data::create_batches;

use parquet_writer::write_parquet;
use parquet_reader::read_parquet;

use query::query_error_latency;

fn main() {

    println!();
    println!("===== CREATE ARROW BATCHES =====");

    let batches = create_batches();

    println!(
        "Created {} batches",
        batches.len()
    );

    println!();
    println!("===== WRITE PARQUET =====");

    write_parquet(
        "logs.parquet",
        &batches,
    );

    println!();
    println!("===== READ PARQUET =====");

    let read_batches =
        read_parquet("logs.parquet");

    println!(
        "Read {} batches",
        read_batches.len()
    );

    println!();
    println!("===== QUERY =====");

    query_error_latency(
        &read_batches
    );
}