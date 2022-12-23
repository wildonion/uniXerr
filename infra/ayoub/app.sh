#!/bin/bash
sudo chown -R root:root . && sudo chmod -R 777 .
echo "What App Do You Want To Build?"
read App
if [[ $App == "conse" ]]; then
    cargo build --bin conse --release
    sudo rm conse
    sudo cp target/release/conse ./conse
    sudo pm2 delete conse
    sudo pm2 start conse --name conse
elif [[ $App == "coiniXerr" ]]; then
    cargo build --bin coiniXerr --release
    sudo rm coiniXerr
    sudo cp target/release/coiniXerr ./coiniXerr
    sudo pm2 delete coiniXerr
    sudo pm2 start coiniXerr --name coiniXerr
else
    echo "Invalid App Name!"
fi