


# 💰 coiniXerr

An Actor, p2p, Sharded TCP and RPC Based Design Pattern Runtime, Blockchain and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract to Mint NFT and FT for Digital Assests inside uniXerr Protocol.

### 📌 WIP 

* complete `rpc.server.rs`, `zmq.p2p.rs` and `tcp.p2p.rs` streaming 

* broadcast other node state to the network on TCP stream in `tcp.p2p.rs`

* complete `rpc_tx_emulator` method and other `StorageModel` ORM trait methods

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors

* use `tcp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP Protocol

* use `udp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the UDP Protocol

* use `rpc_tx_emulator` method to send fake transactions to the coiniXerr nodes through the RPC Protocol
