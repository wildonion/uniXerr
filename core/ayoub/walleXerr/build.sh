#!/bin/bash
set -e
cargo install wasm-pack --force
sudo npm i wasm-opt -g
wasm-pack build --target bundler --out-dir bundlerPKG
wasm-pack build --target web --out-dir webPKG
wasm-pack build --target nodejs --out-dir nodePKG
wasm-opt -Oz bundlerPKG/walleXerr_bg.wasm -o bundlerPKG/walleXerr_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz webPKG/walleXerr_bg.wasm -o webPKG/walleXerr_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz nodePKG/walleXerr_bg.wasm -o nodePKG/walleXerr_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz ../../../target/wasm32-unknown-unknown/release/walleXerr.wasm -o walleXerr.wasm



curl https://wasmtime.dev/install.sh -sSf | bash
rustup target add wasm32-wasi
cargo build --bin walleXerr --target wasm32-wasi --release
sudo cp ../../../target/wasm32-wasi/release/walleXerr.wasm ./walleXerr.wasm
wasmtime walleXerr.wasm