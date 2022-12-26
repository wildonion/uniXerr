

> To Run and Setup each Service: ```sudo chmod +x app.sh && ./app.sh```


## 🧪 Test Conse Server

```cargo test --bin conse```

# 📇 Notes

* Install **capnproto**: ```sudo apt-get install capnproto```

* Install **protobuf** for `libp2p`: ```sudo apt install -y protobuf-compiler```

* Remember that `.env` file and the compiled apps must be in the same palce. 

* To see all binaries: ```cargo run --bin```

* Give the permission to the root using ```sudo chown -R root:root . && sudo chmod -R 777 .``` command.