# SKELETON Development Setup

## Requirements

* **Install prerequisite packages on Linux:** ```sudo apt install openssl libssl-dev cmake libpq-dev```

* **Install _openssl_ for _diesel_ using ```choco install openssl```, _cmake_ for _rdkafka_ lib using ```choco install cmake``` and _gcc_/_g++_ with _mingw_ using ```choco install mingw``` on Windows** 

* **Put _postgres_ lib and bin path into environment variable on Windows:** ```C:\Program Files\PostgreSQL\13\lib``` and ```C:\Program Files\PostgreSQL\13\bin```

* **Install _cargo_ bullshits:** ```cd microservices && cargo install diesel_cli --no-default-features --features postgres && cargo install systemfd cargo-watch```  

* **Build _cassandra_, _postgres_ and _adminer_ docker images(containers):** ```sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v && sudo docker-compose -f docker-compose.yml build && sudo docker-compose up -d --force-recreate --remove-orphans```

## Logging

* **Docker logs:** ```sudo docker images && sudo docker ps && sudo docker-compose -f docker-compose.yml logs -f```

* **Export skeleton _postgres_ db:** ```sudo docker-compose exec postgres /bin/bash -c "PGPASSWORD=skeleton pg_dump --username skeleton --dbname skeleton" > skeleton.sql```

* **Import skeleton _postgres_ db:** ```sudo docker-compose exec -it postgres /bin/bash -c "PGPASSWORD=skeleton psql --username skeleton --dbname skeleton < skeleton.sql```

* **Accessing skeleton _postgres_ db:** ```sudo docker-compose exec postgres psql --username=skeleton --dbname=skeleton```

* **Accessing skeleton _postgres_ db bash:** ```sudo docker-compose exec postgres bash```

* **Accessing _cassandra_ db - node1:** ```sudo docker-compose exec cassandra-node1 cqlsh --username=skeleton --password=skeleton```

* **Accessing _cassandra_ db - node2:** ```sudo docker-compose exec cassandra-node2 cqlsh --username=skeleton --password=skeleton```

* **Accessing _cassandra_ db - node3:** ```sudo docker-compose exec cassandra-node3 cqlsh --username=skeleton --password=skeleton```

* **Accessing _cassandra_ db bash - node1:** ```sudo docker-compose exec cassandra-node1 bash```

* **Accessing _cassandra_ db bash - node2:** ```sudo docker-compose exec cassandra-node2 bash```

* **Accessing _cassandra_ db bash - node3:** ```sudo docker-compose exec cassandra-node3 bash```

* **Accessing _kafka_ broker bash - node1:** ```sudo docker-compose exec kafka-node1 bash```

* **Accessing _kafka_ broker bash - node2:** ```sudo docker-compose exec kafka-node2 bash```

* **Accessing _kafka_ broker bash - node3:** ```sudo docker-compose exec kafka-node3 bash```

## Updating `auth` Microservice API Acess Level

* **Updating access level to admin access:** ```cd cargo microservices/auth/ && cargo run <USERNAME> <ACCESS_LEVEL>```
    * **eg - change access level of user _wildonion_ to admin level:** ```cd microservices/auth/ && cargo run wildonion 2```

## Setup Postgres DB and User

```
CREATE DATABASE skeleton;
CREATE USER skeleton WITH ENCRYPTED PASSWORD 'skeleton';
GRANT ALL PRIVILEGES ON DATABASE skeleton TO skeleton;
ALTER USER skeleton WITH SUPERUSER;
```

## Running Microservices Commands

* **Run _auth_ microservice using one the following commands:** 
    * ```systemfd --no-pid -s http::7366 -- cargo watch -C microservices/auth -x run```
    * ```cargo watch -C microservices/auth -x run```

* **Run _suproxy_ microservice:**
    * ```systemfd --no-pid -s http::7368 -- cargo watch -C microservices/suproxy -x run```
    * ```cargo watch -C microservices/suproxy -x run```

# SKELETON Production Setup

* **Build & run each microservice:** ```sudo chmod +x deploy.sh && ./deploy.sh```

# SKELETON Postgres Database Setup

* **Generate _migrations_ folder, create skeleton postgres db, `diesel.toml` file on first run or run existing migrations into the database:** 

    * ```cd microservices && diesel setup --migration-dir microservices/auth/migrations/```

* **Generate SQL files for your table operations:** ```diesel migration generate SQL-OPERATION_TABLE-NAME```

    * **eg - create users table for _auth_ microservice:** ```diesel migration generate create_users --migration-dir microservices/auth/migrations/```

* **Migrate tables into postgres db and generate(update) `schema.rs` file inside _src_ folder:** ```diesel migration run```

    * **eg - migrate all SQL files of operations of _auth_ microservice into the database:** ```diesel migration run --migration-dir microservices/auth/migrations/```

* **Check diesel migrations errors:** ```diesel migration list```

    * **eg - check migrations errors for _auth_ microservice:** ```diesel migration list --migration-dir microservices/auth/migrations/```

# Hints

* Remember to install _rustup_, _pm2_, _docker_ and _docker-compose_
* Instead of using _tokio_ socket with _mpsc_ job queue channel protocol for live event streaming between threads in our _UI_ apps we've used _kafka_ for heavy long time streaming with load balancing and data repications strategy
* Run `deploy.sh` script only once in your server
* For security reasons you must forward all microservices port to something else using nginx or traefik on your VPS
* Currently there are three cassandra nodes inside our VPS or cluster(datacenter) built and ran with docker
* _cassandra_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of cassandra db server) <-has-> partition replicas <-has-> rows
* _kafka_ => multiple cluster(datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
* Three replicas in cassandra means there are three copies of each partition(contains rows) in each node(cassandra db server)
* Three replicas in kafka means there are three copies of each topics' partitions(buck of events) in each node(kafka broker)
* _kafka_ partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
* The default number of partitions in kafka for each topic is 10
* _actix_ actors communicate with each other through their address of type `Addr` object using some defined events   
* Down migration command for each table is: ```diesel migration down```
* Adminer page address is: **http://SERVER-IP:3257**
* _auth_ microservice API address is: **http://SERVER-IP:7366/skeleton/api/auth**
* _suproxy_ load balancer API address is: **ws://SERVER-IP:7368**
* In order to generate the `schema.rs` in _src_ folder the ```diesel migration run``` command must have a successful result
* You can also create sql files(`up.sql` and `down.sql`) for your table in each migrations folder by hand then run the ```diesel setup``` command to migrate them all into the db at once
