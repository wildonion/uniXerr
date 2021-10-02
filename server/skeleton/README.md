# uniXerr Skeleton Development Setup

## Requirements

* **Install prerequisite packages on Linux:** ```sudo apt install openssl libssl-dev cmake libpq-dev```

* **Install _openssl_ for _diesel_ using ```choco install openssl```, _cmake_ for _rdkafka_ lib using ```choco install cmake``` and _gcc_/_g++_ with _mingw_ using ```choco install mingw``` on Windows** 

* **Put _postgres_ lib and bin path into environment variable on Windows:** ```C:\Program Files\PostgreSQL\13\lib``` and ```C:\Program Files\PostgreSQL\13\bin```

* **Install _cargo_ bullshits:** ```cd microservices && cargo install diesel_cli --no-default-features --features postgres && cargo install systemfd cargo-watch```  

## Updating `auth` Microservice API Acess Level

* **Updating access level to admin access:** ```cd cargo microservices/auth/ && cargo run <USERNAME> <ACCESS_LEVEL>```
    * **eg - change access level of user _wildonion_ to admin level:** ```cd microservices/auth/ && cargo run wildonion 2```

## Running Microservices Commands

* **Run _auth_ microservice using one the following commands:** 
    * ```systemfd --no-pid -s http::7366 -- cargo watch -C microservices/auth -x run```
    * ```cargo watch -C microservices/auth -x run```

* **Run _suproxy_ load balancer using one the following commands:**
    * ```systemfd --no-pid -s http::7368 -- cargo watch -C microservices/suproxy -x run```
    * ```cargo watch -C microservices/suproxy -x run```

* **Run _coiniXerr_ network:**
    * ```cargo watch -C microservices/coiniXerr -x run```

# uniXerr Skeleton Production Setup

## Setup Postgres DB and User

```
CREATE DATABASE uniXerr;
CREATE USER uniXerr WITH ENCRYPTED PASSWORD 'uniXerr';
GRANT ALL PRIVILEGES ON DATABASE uniXerr TO uniXerr;
ALTER USER uniXerr WITH SUPERUSER;
```

* **Build & run each microservice:** ```sudo chmod +x deploy.sh && ./deploy.sh```

# uniXerr Skeleton Postgres Database Setup

* **Generate _migrations_ folder, create uniXerr postgres db, `diesel.toml` file on first run or run existing migrations into the database:** 

    * ```cd microservices && diesel setup --migration-dir microservices/auth/migrations/```

* **Generate SQL files for your table operations:** ```diesel migration generate SQL-OPERATION_TABLE-NAME```

    * **eg - create users table for _auth_ microservice:** ```diesel migration generate create_users --migration-dir microservices/auth/migrations/```

* **Migrate tables into postgres db and generate(update) `schema.rs` file inside _src_ folder:** ```diesel migration run```

    * **eg - migrate all SQL files of operations of _auth_ microservice into the database:** ```diesel migration run --migration-dir microservices/auth/migrations/```

* **Check diesel migrations errors:** ```diesel migration list```

    * **eg - check migrations errors for _auth_ microservice:** ```diesel migration list --migration-dir microservices/auth/migrations/```

# Hints

* Remember to install _rustup_, _pm2_, _postgres_, _cassandra_ and _kafka_
* Instead of using _tokio_ socket with _mpsc_ job queue channel protocol for live event streaming between threads in our _UI_ apps we've used _kafka_ for heavy long time streaming with load balancing and data repications strategy
* Run `deploy.sh` script only once in your server
* For security reasons you must forward all microservices port to something else using nginx or traefik on your VPS
* _cassandra_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of cassandra db server) <-has-> partition replicas <-has-> rows
* _kafka_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
* Three replicas in cassandra means there are three copies of each partition(contains rows) in each node(cassandra db server)
* Three replicas in kafka means there are three copies of each topics' partitions(buck of events) in each node(kafka broker)
* _kafka_ partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
* The default number of partitions in kafka for each topic is 10
* _actix_ actors communicate with each other through their address of type `Addr` object using some defined events   
* Down migration command for each table is: ```diesel migration down```
* Adminer page address is: **http://SERVER-IP:3257**
* _auth_ microservice API address is: **http://SERVER-IP:7366/uniXerr/api/auth**
* _suproxy_ load balancer API address is: **ws://SERVER-IP:7368**
* In order to generate the `schema.rs` in _src_ folder the ```diesel migration run``` command must have a successful result
* You can also create sql files(`up.sql` and `down.sql`) for your table in each migrations folder by hand then run the ```diesel setup``` command to migrate them all into the db at once
