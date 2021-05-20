#!/bin/sh
CARGO_TARGET_DIR=../target
cargo build --release --target wasm32-unknown-unknown
hc dna pack .. -o ../protocol.love.dna
hc app pack .. -o ../protocol.love.happ
