
🏗 Core Backend of the Ayoub PaaS, FaaS and coiniXerr Rafael Runtime with Flexible Design Pattern Supports Pay-As-You-Go Requests written in Hyper, Tokio and Riker.

# ☢️ Run in Dev Mode

> ayoub PaaS: ```cargo run --bin ayoub``` 

> coiniXerr: ```cargo run --bin coiniXerr```

# ☣️ Build for Production

> ayoub PaaS: ```cargo build --bin ayoub --release```

> coiniXerr: ```cargo build --bin coiniXerr --release```

# 🗒 Notes

* Give the permission to the root using ```sudo chown -R root:root . && sudo chmod -R 777 .``` command

* To update a user access level to dev first signup the user using `/auth/signup` API inside the `PaaS` then run the binary like so: `./ayoub wildonion 0`

* To Run and Setup Ayoub: ```sudo chmod +x ayoub.sh && ./ayoub.sh```