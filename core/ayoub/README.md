
🏗 Core Backend of the Ayoub PaaS, FaaS and coiniXerr Rafael Runtime with Flexible Design Pattern Supports Pay-As-You-Go Requests written in Hyper, Tokio and Riker.

# ☢️ Run in Dev Mode

> conse PaaS: ```cargo run --bin conse``` 

> coiniXerr: ```cargo run --bin coiniXerr```

> psychoder: ```cargo run --bin psychoder```

# ☣️ Build for Production

> conse PaaS: ```cargo build --bin conse --release```

> coiniXerr: ```cargo build --bin coiniXerr --release```

> psychoder: ```cargo build --bin psychoder --release```

# 🗒 Notes

* Give the permission to the root using ```sudo chown -R root:root . && sudo chmod -R 777 .``` command

* To update a user access level to dev first signup the user using `/auth/signup` API inside the `PaaS` then run the binary like so: `./app wildonion 0`

* To Run and Setup Ayoub: ```sudo chmod +x app.sh && ./app.sh```