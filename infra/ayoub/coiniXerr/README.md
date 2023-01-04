


# 💰 coiniXerr

An Actor, p2p Pub/Sub, Sharded TCP and RPC Based Design Pattern Runtime, Blockchain with Zero Proof of Knowledge, Proof of Healing and Raft Consensus Mechanisms for uniXerr Cryptocurrency Coin.

> The whole coiniXerr node can be executed from the kernel using **BPF** technology which is balzingly fast! 💥 

### 📇 Notes

* parachain and validator actors' events can be broadcasted to the whole network through the libp2p pub/sub.

* walleXerr communicates with coiniXerr network through the RPC stream with the cap'n proto as the serialization protocol.

* a transaction can also be sent through a TCP stream (bootstrapped with tokio) from a TCP client

* use `tcp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP Protocol.

* use `udp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the UDP Protocol.

* use `rpc_tx_emulator` method to send fake transactions to the coiniXerr nodes through the RPC Protocol.

* coiniXerr node can use the `user_data` macro to fetch all the information of a specific user from the **Conse** server for the authentication process. 

### 📌 WIP 

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh` also refer to [NEAR Rules](https://github.com/wildonion/smarties/blob/main/contracts/near/NEAR.rules) for implementing unique scaling mechanism like `nightshade sharding`.

* `StorageModel` ORM trait methods, Rafael runtime and its log, onion actors, utils macros, reset slot and consensus algorithms

* gitbook from wiki