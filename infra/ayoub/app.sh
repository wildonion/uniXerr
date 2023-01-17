#!/bin/bash
sudo chown -R root:root . && sudo chmod -R 777 .
echo "[?] What App Do You Want To Build?"
read App    
if [[ $App == "coiniXerr" ]]; then
    echo "[+] Installing Required Packages for BPF Compiling"
    sudo apt -y install build-essential zlib1g-dev llvm-12-dev libclang-12-dev linux-headers-$(uname -r) libelf-dev
    sudo apt update && sudo apt-get -y install wget software-properties-common lsb-release linux-headers-generic pkg-config
    sudo wget https://apt.llvm.org/llvm.sh && sudo chmod +x sudo llvm.sh && ./llvm.sh 13 && rm -f ./llvm.sh
    sudo apt install -y capnproto && sudo apt install -y protobuf-compiler
    cargo install cargo-bpf
    cargo build --bin coiniXerr --release
    sudo rm coiniXerr
    sudo cp target/release/coiniXerr ./coiniXerr
    sudo pm2 delete coiniXerr
    sudo pm2 start coiniXerr --name coiniXerr # run the compile coiniXerr node
    cargo bpf build # build into the .elf contains the BPF bytecode 
    cargo bpf load target/release/bpf-programs/coiniXerr/coiniXerr.elf # run the BPF from the linux kernel
elif [[ $App == "walleXerr" ]]; then
    echo "Building walleXerr"
    rustup target add wasm32-unknown-unknown # compilation target for browser-based WebAssembly
    cargo install --locked trunk wasm-bindgen-cli
    cd walleXerr && trunk build --release
else
    echo "[!] Invalid App Name!"
fi