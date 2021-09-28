

An asynchronous 🪢 and multithreading🧵 CLI backend supports various deep learning algorithms and networks such as MLP, CNN, LSTM, VAE and Transformers


# Psychoder Development Setup

## Requirements

* **Install prerequisite packages on Linux:** ```sudo apt install openssl libssl-dev cmake libpq-dev```

* **Install _openssl_ using ```choco install openssl```, _cmake_ for _rdkafka_ lib using ```choco install cmake``` and _gcc_/_g++_ with _mingw_ using ```choco install mingw``` on Windows** 

* **Install `cargo-watch`:** ```cargo install cargo-watch```  

* **Build _cassandra_ and _kafka_ docker images(containers):** ```sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v && sudo docker-compose -f docker-compose.yml build && sudo docker-compose up -d --force-recreate --remove-orphans```

## Logging

* **Docker logs:** ```sudo docker images && sudo docker ps && sudo docker-compose -f docker-compose.yml logs -f```

* **Accessing _cassandra_ db - node1:** ```sudo docker-compose exec cassandra-node1 cqlsh --username=supapp --password=supapp```

* **Accessing _cassandra_ db - node2:** ```sudo docker-compose exec cassandra-node2 cqlsh --username=supapp --password=supapp```

* **Accessing _cassandra_ db - node3:** ```sudo docker-compose exec cassandra-node3 cqlsh --username=supapp --password=supapp```

* **Accessing _cassandra_ db bash - node1:** ```sudo docker-compose exec cassandra-node1 bash```

* **Accessing _cassandra_ db bash - node2:** ```sudo docker-compose exec cassandra-node2 bash```

* **Accessing _cassandra_ db bash - node3:** ```sudo docker-compose exec cassandra-node3 bash```

* **Accessing _kafka_ broker bash - node1:** ```sudo docker-compose exec kafka-node1 bash```

* **Accessing _kafka_ broker bash - node2:** ```sudo docker-compose exec kafka-node2 bash```

* **Accessing _kafka_ broker bash - node3:** ```sudo docker-compose exec kafka-node3 bash```

## Running Microservices Commands

* **Watch _psychoder_ service:** ```cargo watch -x run```

# Production Setup

* **Build _psychoder_ service:** ```cargo build --bin psychoder --release```

# Hints

* Remember to install _rustup_, _pm2_, _docker_ and _docker-compose_
* Instead of using _tokio_ socket with _mpsc_ job queue channel protocol for live event streaming between threads in our _UI_ apps we've used _kafka_ for heavy long time streaming with load balancing and data repications strategy
* Currently there are three cassandra nodes and three kafka brokers(nodes) inside our VPS or cluster(datacenter) built and ran with docker
* _cassandra_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of cassandra db server) <-has-> partition replicas <-has-> rows
* _kafka_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
* Three replicas in cassandra means there are three copies of each partition(contains rows) in each node(cassandra db server)
* Three replicas in kafka means there are three copies of each topics' partitions(buck of events) in each node(kafka broker)
* _kafka_ partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
* The default number of partitions in kafka for each topic is 10
