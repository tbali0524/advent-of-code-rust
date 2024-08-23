cargo check
cargo fmt
cargo clippy
cargo test
cargo doc --no-deps --open
cargo build --release
cargo run --release -- %*
