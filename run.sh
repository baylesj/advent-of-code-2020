#!/usr/bin/env sh

# Helper script for build + running.
cargo fmt && cargo build && cargo run $0 $1 $2 $3 $4 $5
