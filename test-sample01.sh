#!/bin/sh
percentage=${1-"60"}
cargo run --release -- frequency -f sample01.txt -n 2
