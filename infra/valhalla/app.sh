#!/bin/bash
wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2_amd64.deb
sudo chown -R root:root . && sudo chmod -R 777 .
sudo apt update -y && curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
sudo apt-get install -y nodejs && npm install pm2@latest -g
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | sudo apt-key add -
sudo apt-get install gnupg
wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | sudo apt-key add -
echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu bionic/mongodb-org/6.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list
sudo apt-get update -y && sudo apt-get install -y mongodb-org
sudo mkdir -p /data/db && sudo chown -R $USER /data/db
echo "[?] What App Do You Want To Build?"
read App    
if [[ $App == "coiniXerr" ]]; then
    echo "☕ Building coiniXerr"
    sudo apt -y install build-essential zlib1g-dev llvm-12-dev libclang-12-dev linux-headers-$(uname -r) libelf-dev libudev-dev
    sudo apt update && sudo apt-get -y install wget software-properties-common lsb-release linux-headers-generic pkg-config
    sudo wget https://apt.llvm.org/llvm.sh && sudo chmod +x sudo llvm.sh && ./llvm.sh 13 && rm -f ./llvm.sh
    sudo apt install -y capnproto && sudo apt install -y protobuf-compiler
    cargo install cargo-bpf
    cargo build --bin coiniXerr --release
    sudo rm coiniXerr && sudo cp target/release/coiniXerr ./coiniXerr
    sudo pm2 delete coiniXerr && sudo pm2 start coiniXerr --name coiniXerr # run the compile coiniXerr node
    cargo bpf build # build into the .elf contains the BPF bytecode 
    cargo bpf load target/release/bpf-programs/coiniXerr/coiniXerr.elf # run the BPF from the linux kernel
elif [[ $App == "walleXerr" ]]; then
    echo "☕ Building walleXerr"
    rustup target add wasm32-unknown-unknown # install the compilation target for browser-based WebAssembly using lib.rs
    cargo install --locked trunk wasm-bindgen-cli
    cd walleXerr && trunk build --release
else
    echo "[!] Invalid App Name!"
fi