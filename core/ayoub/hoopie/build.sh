#!/bin/bash
set -e
cargo install wasm-pack --force
sudo npm i wasm-opt -g
wasm-pack build --target bundler --out-dir bundlerPKG
wasm-pack build --target web --out-dir webPKG
wasm-pack build --target nodejs --out-dir nodePKG
wasm-opt -Oz bundlerPKG/hoopie_bg.wasm -o bundlerPKG/hoopie_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz webPKG/hoopie_bg.wasm -o webPKG/hoopie_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz nodePKG/hoopie_bg.wasm -o nodePKG/hoopie_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz ../../../target/wasm32-unknown-unknown/release/hoopie.wasm -o hoopie.wasm



curl https://wasmtime.dev/install.sh -sSf | bash
rustup target add wasm32-wasi
cargo build --bin hoopie --target wasm32-wasi --release
sudo cp ../../../target/wasm32-wasi/release/hoopie.wasm ./hoopie.wasm
wasmtime hoopie.wasm