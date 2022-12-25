


# 💰 coiniXerr

An Actor, p2p Pub/Sub, Sharded TCP and RPC Based Design Pattern Runtime, Zero Proof of Knowledge Blockchain for uniXerr Cryptocurrency Coin.

> The whole coiniXerr node can be executed from the kernel using **BPF** technology which is balzingly fast! 💥 


### 📇 Notes

* Actors' events can be broadcasted to the whole network through the libp2p pub/sub.

* walleXerr communicates with coiniXerr network through the RPC stream with the cap'n proto as the serialization protocol.

* A transaction can also be sent through a TCP stream (bootstrapped with tokio) from a TCP client

* use `tcp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP Protocol.

* use `udp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the UDP Protocol.

* use `rpc_tx_emulator` method to send fake transactions to the coiniXerr nodes through the RPC Protocol.

### 📌 WIP 

* complete `actor.rs`, `rpc.server.rs` and `tcp.server.rs` streaming TODOs

* complete `rpc_tx_emulator` method and other `StorageModel` ORM trait methods

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus, transaction and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors

* refer to [NEAR Rules](https://github.com/wildonion/smarties/blob/main/contracts/near/NEAR.rules) for implementing unique scaling mechanism like `nightshade sharding`
