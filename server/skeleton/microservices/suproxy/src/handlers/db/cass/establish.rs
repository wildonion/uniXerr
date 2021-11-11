



use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::session::new_lz4 as uniXerrClusterSession;
use cdrs::cluster::{ClusterTcpConfig, TcpConnectionPool, NodeTcpConfigBuilder};
use cdrs::load_balancing::RoundRobinSync;
use cdrs::query::*;
use std::env;
use dotenv::dotenv;
use std::sync::Arc;
pub type CassSession = Session<RoundRobinSync<TcpConnectionPool<StaticPasswordAuthenticator>>>;



//-- we've used cdrs without tokio version due to confliction between actix version 0.12 and cdrs tokio version  
//-- we have to clone the session every time we want to pass it between threads or function calls cause we put it inside the Arc in order to be shareable
//-- we've used Box to save a pointer on stack from the allocated heap for the trait object, also we've used Box<dyn Trait> 
//-- cause we don't know what type of object Error trait is implemented for, for example it might be for uniXerrClusterSession object.
//-- the successful return type of this function is an Arc<CassSession> for sharing the ownership of session between tokio threads.
//-- due to having no mutatation of the session at runtime inside a thread we didn't put the CassSession type inside a Mutex.
//-- cassandra => multiple cluster (datacenter or VPS) <-has-> nodes (multiple instances of cassandra db server) <-has-> partition replicas <-has-> rows
//-- three replicas in cassandra means there are three copies of each partition(contains rows) in each node(cassandra db server)
pub async fn connection() -> Result<Arc<CassSession>, Box<dyn std::error::Error>>{

    
    
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("CASS_HOST").expect("⚠️ please set cassandra host in .env");
    let username = env::var("CASSANDRA_USER").expect("⚠️ please set cassandra username in .env");
    let password = env::var("CASSANDRA_PASSWORD").expect("⚠️ please set cassandra password in .env");
    let authenticator = StaticPasswordAuthenticator::new(username, password);
    let cass_node1_port = env::var("CASS_NODE1_PORT").expect("⚠️ please set cassandra node1 port in .env");
    let cass_node2_port = env::var("CASS_NODE2_PORT").expect("⚠️ please set cassandra node2 port in .env");
    let cass_node3_port = env::var("CASS_NODE3_PORT").expect("⚠️ please set cassandra node3 port in .env");
    

    let node1_addr: &'static str = helerium::string_to_static_str(format!("{}:{}", host, cass_node1_port));
    let node2_addr: &'static str = helerium::string_to_static_str(format!("{}:{}", host, cass_node2_port));
    let node3_addr: &'static str = helerium::string_to_static_str(format!("{}:{}", host, cass_node3_port));


    let node1 = NodeTcpConfigBuilder::new(node1_addr, authenticator.clone())
        .max_size(5) 
        .min_idle(Some(4))
        .max_lifetime(Some(std::time::Duration::from_secs(60)))
        .idle_timeout(Some(std::time::Duration::from_secs(60)))
        .build();

    let node2 = NodeTcpConfigBuilder::new(node2_addr, authenticator.clone())
        .max_size(5) 
        .min_idle(Some(4))
        .max_lifetime(Some(std::time::Duration::from_secs(60)))
        .idle_timeout(Some(std::time::Duration::from_secs(60)))
        .build();

    let node3 = NodeTcpConfigBuilder::new(node3_addr, authenticator.clone())
        .max_size(5) 
        .min_idle(Some(4))
        .max_lifetime(Some(std::time::Duration::from_secs(60)))
        .idle_timeout(Some(std::time::Duration::from_secs(60)))
        .build();


    let load_balancer = RoundRobinSync::new();
    let cluster_config = ClusterTcpConfig(vec![node1
                                                                                    //   node2, 
                                                                                    //   node3
                                                                                      ]); //-- putting node servers inside the(this) cluster(datacenter)
    let session = uniXerrClusterSession(&cluster_config, load_balancer).expect("⚠️ can't create session"); //-- uniXerrClusterSession returns an error Result so we can call expect() on it 
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS uniXerr WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 3};";
    session.query(create_ks).expect("⚠️ uniXerr keyspace create error");
    Ok(Arc::new(session)) //-- putting the(this) cluster(datacenter) and its nodes balanced with RoundRobin inside a compressed lz4 session


}