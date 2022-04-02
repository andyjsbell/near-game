#!/bin/bash
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "`dirname $0`"
cargo build --target wasm32-unknown-unknown --release
mkdir res
cp $TARGET/wasm32-unknown-unknown/release/near_game.wasm ./res/
#wasm-opt -Oz --output ./res/near_game.wasm ./res/near_game.wasm

