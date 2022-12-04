



## Setup 

> ```sudo chown -R root:root . && sudo chmod 777 -R .```

> ```sudo cp  /home/uniXerr/.near-credentials/ /root/```

### Contract State Initialization

```console
near call reserve.conse.testnet new '{"owner_id": "reserve.conse.testnet"}' --accountId reserve.conse.testnet
near call donate.coinixerr.testnet new '{"owner_id": "donate.coinixerr.testnet"}' --accountId donate.coinixerr.testnet
```

### Delete/Create Sub-accounts

```console
near delete reserve.conse.testnet conse.testnet
near delete donate.coinixerr.testnet coinixerr.testnet
near create-account reserve.conse.testnet --masterAccount conse.testnet --initialBalance 25
near create-account donate.coinixerr.testnet --masterAccount coinixerr.testnet --initialBalance 25
```

### Deploy on Master Accounts

> We can only have one smart contract per each account.

```console
NEAR_ENV=testnet near deploy --wasmFile reserve/out/conse.wasm --accountId reserve.conse.testnet
NEAR_ENV=testnet near deploy --wasmFile donation/out/coinixerr.wasm --accountId donate.coinixerr.testnet
```