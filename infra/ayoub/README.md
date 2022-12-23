# ☢️ Run in Dev Mode

> conse PaaS: ```cargo run --bin conse``` 

> coiniXerr: ```cargo run --bin coiniXerr```

> waleXerr ui: ```cargo run --bin walleXerr```

## 🧪 Test Conse Server

```cargo test --bin conse```

# ☣️ Build for Production

> conse PaaS: ```cargo run --bin conse --release``` 

> coiniXerr: ```cargo run --bin coiniXerr --release```

> waleXerr ui: ```cargo run --bin walleXerr --release```

# 📇 Notes

* Install protobuf for `libp2p`: ```sudo apt install -y protobuf-compiler```

* Remember that `.env` file and the compiled app must be in the same palce. 

* To see all binaries: ```cargo run --bin```

* Give the permission to the root using ```sudo chown -R root:root . && sudo chmod -R 777 .``` command.

* To update a user access level to dev first signup the user using `/auth/signup` API inside the `PaaS` then run the binary like so: `./app wildonion 0`

* To Run and Setup Ayoub: ```sudo chmod +x app.sh && ./app.sh```