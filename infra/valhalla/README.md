

> Run Valhalla: ```sudo chmod +x app.sh && ./app.sh```

## 📇 Notes

* Remember that `.env` file and the compiled apps must be in the same palce. 

* To see all binaries: ```cargo run --bin```

## 🚧 WIP 

* add `quic` protocol for P2P stack

* complete websocket pubsub protcol

* merkle root, parachain heartbeat and validator reward distribution

* first coiniXerr node compilation test

* complete RPC pub/sub streaming setup and `rcp_tx_emultator()` function

* setup noise protocol over ring encryption algorithms using [snow](https://crates.io/crates/snow) for tokio TCP, UDP and RPC server

* second coiniXerr node test with BPF (complete BPF setup in code and `cvm.rs`)

* https://github.com/wildonion/cs-concepts#-blogs-and-books

* `StorageModel` ORM trait methods, Rafael runtime and its log, onion actors, utils macros and methods and consensus algorithms

* walleXerr TODOs

* complete wiki
