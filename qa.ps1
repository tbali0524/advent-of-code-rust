# run QA checks and all the puzzles
cargo check
cargo fmt
cargo clippy
cargo doc --no-deps --open
cargo test
cargo run --release
