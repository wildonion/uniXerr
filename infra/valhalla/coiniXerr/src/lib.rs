







use std::collections::HashMap;
use std::env;
use dotenv::dotenv;




pub fn get_env_vars() -> HashMap<String, String>{

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                    env vars setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    dotenv().expect("⚠️ .env file not found");
    let mut vars: HashMap<String, String> = HashMap::new();
    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("DB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("DB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let secret_key = env::var("SECRET_KEY").expect("⚠️ no secret key variable set");
    let wallet_secret_key = env::var("GENERATE_WALLET_ADDRESS_SECRET_KEY").expect("⚠️ no secret key variable set for wallet address");
    let merkle_root_secret_key = env::var("MERKLE_ROOT_SECRET_KEY").expect("⚠️ no secret key variable set for merkle root");
    let genesis_secret_key = env::var("GENESIS_SECRET_KEY").expect("⚠️ no secret key variable set for genesis");
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ please set buffer size in .env");
    let max_block_size = env::var("MAX_BLOCK_SIZE").expect("⚠️ please set block size in .env");
    let max_epoch = env::var("MAX_EPOCH").expect("⚠️ please set max epoch in .env");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let rpc_addr = env::var("RPC_ADDR").expect("⚠️ no rpc addr variable set");
    let websocket_addr = env::var("WEBSOCKET_ADDR").expect("⚠️ no websocket addr variable set");
    let tcp_addr = env::var("TCP_ADDR").expect("⚠️ no tcp addr variable set");
    let udp_addr = env::var("UDP_ADDR").expect("⚠️ no udp addr variable set");
    let swarm_addr = env::var("SWARM_ADDR").expect("⚠️ no swarm addr variable set");


    vars.insert("DB_HOST".to_string(), db_host);
    vars.insert("DB_PORT".to_string(), db_port);
    vars.insert("DB_USERNAME".to_string(), db_username);
    vars.insert("DB_PASSWORD".to_string(), db_password);
    vars.insert("DB_ENGINE".to_string(), db_engine);
    vars.insert("DB_NAME".to_string(), db_name);
    vars.insert("ENVIRONMENT".to_string(), environment);
    vars.insert("SECRET_KEY".to_string(), secret_key);
    vars.insert("GENERATE_WALLET_ADDRESS_SECRET_KEY".to_string(), wallet_secret_key);
    vars.insert("MERKLE_ROOT_SECRET_KEY".to_string(), merkle_root_secret_key);
    vars.insert("GENESIS_SECRET_KEY".to_string(), genesis_secret_key);
    vars.insert("RPC_ADDR".to_string(), rpc_addr);
    vars.insert("WEBSOCKET_ADDR".to_string(), websocket_addr);
    vars.insert("TCP_ADDR".to_string(), tcp_addr);
    vars.insert("UDP_ADDR".to_string(), udp_addr);
    vars.insert("HOST".to_string(), host);
    vars.insert("BUFFER_SIZE".to_string(), buffer_size);
    vars.insert("MAX_BLOCK_SIZE".to_string(), max_block_size);
    vars.insert("MAX_EPOCH".to_string(), max_epoch);
    vars.insert("SWARM_ADDR".to_string(), swarm_addr);

    vars

}