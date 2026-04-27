# Build and Run

## Prerequisites
- Install Rust (includes `cargo`): https://www.rust-lang.org/tools/install
- Verify installation:
    ```bash
    rustc --version
    cargo --version
    ```

## Create a New Rust Project (Optional)
If you haven't already created a Rust project, you can do so with:
```bash
cargo new my_rust_project
cd my_rust_project
```

## Build the Program
From the project root (where `Cargo.toml` is located), run:
```bash
cargo build
```

## Run the Program
```bash
cargo run
```

## Build and Run Optimized (Release)
```bash
cargo run --release
```

## Run Tests (Optional)
```bash
cargo test
```