#!/usr/bin/env sh

# clean up previous coverage result
cargo cov clean

# test the code
cargo cov test

# open the coverage report
cargo cov report --open