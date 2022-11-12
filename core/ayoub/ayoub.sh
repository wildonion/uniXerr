#!/bin/bash
sudo chown -R root:root . && sudo chmod -R 777 .
cargo build --bin ayoub --release
sudo rm app
sudo cp target/release/ayoub ./app
sudo pm2 delete ayoub
sudo pm2 start app --name ayoub