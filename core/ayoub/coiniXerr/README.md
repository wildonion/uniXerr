


# 💰 coiniXerr

An Actor, p2p, Sharded TCP and RPC Based Design Pattern Runtime, Blockchain and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract to Mint NFT and FT for Digital Assests inside uniXerr Protocol.

### 📌 WIP 

* implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr with compilation commands in `app.sh` 

* implementing cap'n proto serialization RPC server and client, ZMQ streaming (coiniXerr nodes and the walleXerr must communicate with each other through the RPC and ZMQ with cap'n proto as the serialization protocol)

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors

* building coiniXerr engines

* use `tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP socket
