#!/bin/sh
percentage=${1-"50"}
cargo run --release -- frequency -f sample-big.txt -n 3


