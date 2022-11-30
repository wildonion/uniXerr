



## Setup 

> ```sudo chown -R root:root . && sudo chmod 777 -R .```

> ```sudo cp  /home/uniXerr/.near-credentials/ /root/```

### Contract State Initialization

```console
near call reserve.conse.testnet new '{"owner_id": "reserve.conse.testnet"}' --accountId reserve.conse.testnet
near call donate.hoopoe.testnet new '{"owner_id": "donate.hoopoe.testnet"}' --accountId donate.hoopoe.testnet
```

### Delete/Create Sub-accounts

```console
near delete reserve.conse.testnet conse.testnet
near delete donate.hoopoe.testnet hoopoe.testnet
near create-account reserve.conse.testnet --masterAccount conse.testnet --initialBalance 25
near create-account donate.hoopoe.testnet --masterAccount hoopoe.testnet --initialBalance 25
```

### Deploy on Master Accounts

> We can only have one smart contract per each account.

```console
NEAR_ENV=testnet near deploy --wasmFile reserve/out/conse.wasm --accountId conse.testnet
NEAR_ENV=testnet near deploy --wasmFile donation/out/hoopoe.wasm --accountId hoopoe.testnet
```