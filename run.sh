#!/usr/bin/env sh

# Helper script for build + running.
cargo fmt && cargo build && cargo run --release -- "$@"
