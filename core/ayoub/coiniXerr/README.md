


# 💰 coiniXerr

An Actor, Sharded TCP, UDP and RPC Based Design Pattern Runtime, Blockchain and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract to Mint NFT and FT for Digital Assests inside uniXerr Protocol.

### 📌 WIP 

* implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr 

* implementing cap'n proto serialization RPC server and client, RMQ streaming with celery (coiniXerr nodes must communicate with each other through the RPC protocol with cap'n proto as the serialization protocol)

* implementing JSON-RPC server protocol to communicate with walleXerr

* implementing p2p on UDP protocol for coiniXerr nodes 

* HAProxy, dockerizing, k8s-ing and ci/cd in `app.sh`

* coiniXerr engines, reset slot, mempool channel, runtime log, consensus and block validation process, transaction signature, block and merkle root and wallet address using Argon2 based on `XChaCha20Poly1305` end-to-end encryption

* implementing Rafael runtime and onion actors

* building coiniXerr engines

* use `tx_emulator` method to send fake transactions to the coiniXerr nodes through the TCP socket
