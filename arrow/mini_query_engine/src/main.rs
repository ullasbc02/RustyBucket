mod data;
mod engine;
mod operators;

fn main() {
    println!("Running Mini Arrow Query Engine");

    engine::run_query();
}