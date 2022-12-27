

# 💳 walleXerr

coiniXerr Wallet Front-end 

## Notes

* to run in dev mode: ```trunk serve```

* all walleXerr codes is written in Rust which will be compiled to `wasm` to load it inside `js`

* users can send transactions from this wallet to the coiniXerr nodes for mining processes through RPC with cap'n proto as the serialization protocol

* the `ttype` must be `0x00` which is a regular transaction.