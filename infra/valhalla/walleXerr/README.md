

# 💳 walleXerr

coiniXerr Wallet Front-end 

## 📇 Notes

* to run in dev mode: ```trunk serve```

* all walleXerr codes is written in Rust which will be compiled to `wasm` to load it inside `js`

* users can send transactions from this wallet to the coiniXerr nodes for mining processes through RPC channels with cap'n proto as the serialization protocol

* realtime streaming over RPC cap'n proto for stock monitoring

* the `ttype` must be `0x00` which is a regular transaction.

* all transactions need to be signed with the sender's private key.

* use public-key (asymmetric) digital signature encryption to generate a keypair (hash of the public key as the wallet address and private key) for tx signing process.

* generated private key MUST be converted into ascii chars (secret phrase) to login with it.

* we can't have async I/O stream, sockets and std libs inside the wasm.
