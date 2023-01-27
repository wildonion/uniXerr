


# 💰 coiniXerr

An Actor, Gossipsub P2P Pub/Sub, Sharded TCP, UDP and RPC Based Design Pattern Runtime, Blockchain with Zero Proof of Knowledge, Proof of Healing and Raft Consensus Mechanisms for uniXerr Cryptocurrency Coin.

### 🍟 Features

* Sharding based machanism

* Actor based validators and parachains

* Transaction Server over `tokio` **TCP**, **UDP** and **RPC**

* Transaction **mempool** channel based on `tokio` **MPSC** job queue channel 

* P2P **kademlia** for peer finding and **gossipsub** for pub/sub pattern

* **Cap'n Proto** as the serialization method in **RPC** communication

* Supports **Zero Proof of Knowledge** and **Raft** consensus mechanisms

* A new consensus mechanisms called **Proof of Healing**

* Can be compiled to a linux kernel executable using **BPF** 💥 technology

* **parachain** based blockchain supports acutions to reset the **slot**

* A **FaaS** runtime called **Rafael** to take control of the node state, balance the traffic and load [STEM](https://github.com/wildonion/stem) `.wasm` file for **AI** logics inside the node   

### 📇 Notes

* parachain and validator actors' events can be broadcasted to the whole network through the libp2p pub/sub.

* walleXerr communicates with coiniXerr network through the RPC stream with the cap'n proto as the serialization protocol.

* a transaction can also be sent through a TCP and UDP stream (bootstrapped with tokio) from a TCP or UDP client for verifying and block mining process.

* only TCP, UDP and RPC protocols are responsible for transaction and block verifying and mining process. 

* use `tcp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP Protocol.

* use `udp_tx_emulator` method to send fake transactions to the coiniXerr nodes through the UDP Protocol.

* use `rpc_tx_emulator` method to send fake transactions to the coiniXerr nodes through the RPC Protocol.

* coiniXerr node can use the `user_data` macro to fetch all the information of a specific user from the **Conse** server for the authentication process. 