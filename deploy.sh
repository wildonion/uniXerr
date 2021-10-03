#!/bin/bash










# NOTE - building each microservice separately
# ...
# cargo build --bin auth --release --manifest-path server/skeleton/microservices/Cargo.toml
# cargo build --bin suproxy --release --manifest-path server/skeleton/microservices/Cargo.toml
# cargo build --bin psychoder --release --manifest-path server/skeleton/microservices/Cargo.toml
# cargo build --bin coiniXerr --release --manifest-path server/skeleton/microservices/Cargo.toml





cd microservices
diesel setup --migration-dir auth/migrations/
chown -R $USER:$USER . && cargo build --bins --release --manifest-path Cargo.toml && sudo cp .env $HONE/.env
sudo mv target/release/auth $HONE
sudo mv target/release/suproxy $HONE
sudo mv target/release/psychoder $HONE
sudo mv target/release/coiniXerr $HONE
cd $HOME
pm2 start auth
pm2 start suproxy
pm2 start psychoder
pm2 start coiniXerr
pm2 startup
