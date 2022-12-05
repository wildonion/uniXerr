


# 💰 coiniXerr

An Actor, p2p, Sharded TCP, UDP and RPC Based Design Pattern Runtime, Blockchain and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract to Mint NFT and FT for Digital Assests inside uniXerr Protocol.

### 📌 WIP 

* implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr 

* implementing cap'n proto and JSON serialization RPC server and client, ZMQ streaming (coiniXerr nodes must communicate with each other and the walleXerr through the RPC protocol with cap'n proto and JSON as the serialization protocols)

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors

* building coiniXerr engines

* use `tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP socket
