export CARGO_HOME=/home/runner/chengine/cargo_stash
export RUST_BACKTRACE=1
cargo test -- --test-threads 4 &&
# cargo test -- --nocapture
cargo run --release
