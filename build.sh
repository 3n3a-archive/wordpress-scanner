#!/bin/bash

# builds and compresses for prod

# build
cargo build --release

# show how much
ls -lha target/release/wordpress-scanner

# compresss
sleep 1
upx -9 target/release/wordpress-scanner

# show how much
ls -lha target/release/wordpress-scanner