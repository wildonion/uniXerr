






use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use cdrs_tokio::cluster::{NodeTcpConfigBuilder, NodeTcpConfig};
use std::env;
use dotenv::dotenv;


pub fn builder(port: String) -> NodeTcpConfig{

    
    
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let username = env::var("CASSANDRA_USER").expect("⚠️ please set cassandra username in .env");
    let password = env::var("CASSANDRA_PASSWORD").expect("⚠️ please set cassandra password in .env");
    let authenticator = StaticPasswordAuthenticatorProvider::new(username, password);
    
    

    //-- the type Arc<T> provides shared ownership of a value of type T, allocated in the heap
    NodeTcpConfigBuilder::new(format!("{}:{}", host, port), std::sync::Arc::new(authenticator))
        .max_size(5) 
        .min_idle(Some(4))
        .max_lifetime(Some(std::time::Duration::from_secs(60)))
        .idle_timeout(Some(std::time::Duration::from_secs(60)))
        .build() //-- building node1 using r2d2 - build() method returns NodeTcpConfig, thus the return type of builder() function will be this type  

    
    
}
    

