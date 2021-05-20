#!/bin/sh
TRYORAMA_LOG_LEVEL=info
RUST_BACKTRACE=1
TRYORAMA_HOLOCHAIN_PATH="holochain"
ts-node tryorama/index.ts
