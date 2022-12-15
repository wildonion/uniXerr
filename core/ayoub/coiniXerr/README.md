


# 💰 coiniXerr

An Actor, p2p, Sharded ZMQ Pub/Sub and RPC Based Design Pattern Runtime, Blockchain and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract to Mint NFT and FT for Digital Assests inside uniXerr Protocol.


### 📇 Notes

* Actors' events can be broadcasted to the whole network through the ZMQ p2p pub/sub with the cap'n proto as the serialization protocol.

* walleXerr communicates with coiniXerr network through the RPC stream with the cap'n proto as the serialization protocol.

* A transaction can also be sent through a TCP stream from a TCP client.

* use `tcp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP Protocol.

* use `udp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the UDP Protocol.

* use `rpc_tx_emulator` method to send fake transactions to the coiniXerr nodes through the RPC Protocol.


### 📌 WIP 

* complete `rpc.server.rs`, `zmq.p2p.rs` and `tcp.p2p.rs` streaming 

* broadcast other node state to the network on TCP stream in `tcp.p2p.rs`

* complete `rpc_tx_emulator` method and other `StorageModel` ORM trait methods

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors