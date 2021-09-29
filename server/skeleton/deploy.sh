#!/bin/bash




sudo docker stop $(sudo docker ps -a -q) && sudo docker-compose down -v && sudo docker system prune -af --volumes
sudo docker-compose -f docker-compose.yml build --no-cache && sudo docker-compose up -d --force-recreate --remove-orphans
sudo docker save $(sudo docker images -a -q) | gzip > $HOME/skeleton.tar.gz && sudo docker load -i $HOME/skeleton.tar.gz



# NOTE - building each microservice separately
# ...
# cargo build --bin auth --release --manifest-path microservices/Cargo.toml
# cargo build --bin suproxy --release --manifest-path microservices/Cargo.toml
# cargo build --bin psychoder --release --manifest-path microservices/Cargo.toml
# cargo build --bin neura --release --manifest-path microservices/Cargo.toml
# cargo build --bin musiem --release --manifest-path microservices/Cargo.toml
# cargo build --bin fearless --release --manifest-path microservices/Cargo.toml
# cargo build --bin coiniXerr --release --manifest-path microservices/Cargo.toml
# cargo build --bin wolfia --release --manifest-path microservices/Cargo.toml





cd microservices
diesel setup --migration-dir auth/migrations/
chown -R $USER:$USER . && cargo build --bins --release --manifest-path Cargo.toml && sudo cp .env $HONE/.env
sudo mv target/release/auth $HONE
sudo mv target/release/suproxy $HONE
sudo mv target/release/psychoder $HONE
sudo mv target/release/neura $HONE
sudo mv target/release/musiem $HONE
sudo mv target/release/fearless $HONE
sudo mv target/release/coiniXerr $HONE
sudo mv target/release/wolfia $HONE
cd $HOME
pm2 start auth
pm2 start suproxy
pm2 start psychoder
pm2 start neura
pm2 start musiem
pm2 start fearless
pm2 start coiniXerr
pm2 start wolfia 
pm2 startup
