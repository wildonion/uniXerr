







use super::node; //-- super refers to the root directory of the establish.rs file - you can also use this syntax : use crate::handlers::db::cass::node;
use cdrs_tokio::cluster::session::Session;
use cdrs_tokio::cluster::session::new_lz4 as MUSIEMClusterSession;
use cdrs_tokio::cluster::{ClusterTcpConfig, TcpConnectionPool};
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::retry::DefaultRetryPolicy;
use cdrs_tokio::query::*;
use std::env;
use dotenv::dotenv;
use std::sync::Arc;
pub type CassSession = Session<RoundRobin<TcpConnectionPool>>;




//-- we've used Box to save a pointer on stack from the allocated heap for the trait object, also we've used Box<dyn Trait> 
//-- cause we don't know what type of object Error trait is implemented for, for example it might be for MUSIEMClusterSession object.
//-- the successful return type of this function is an Arc<CassSession> for sharing the ownership of session between tokio threads.
//-- due to having no mutatation of the session at runtime inside a thread we didn't put the CassSession type inside a Mutex.     
pub async fn connection() -> Result<Arc<CassSession>, Box<dyn std::error::Error>>{

    
    
    dotenv().expect("⚠️ .env file not found");
    let node1 = node::builder(env::var("CASS_NODE1_PORT").expect("⚠️ please set cassandra node1 port in .env")); //-- node server 1
    let node2 = node::builder(env::var("CASS_NODE2_PORT").expect("⚠️ please set cassandra node2 port in .env")); //-- node server 2
    let node3 = node::builder(env::var("CASS_NODE3_PORT").expect("⚠️ please set cassandra node3 port in .env")); //-- node server 3



    
    let load_balancer = RoundRobin::new();
    let cluster_config = ClusterTcpConfig(vec![node1, node2, node3]); //-- putting node servers inside the(this) cluster(datacenter)
    let policy = Box::new(DefaultRetryPolicy::default());
    

    
    let session = MUSIEMClusterSession(&cluster_config, load_balancer, policy).await.expect("⚠️ can't create session"); //-- MUSIEMClusterSession returns an error Result so we can call expect() on it 
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS musiem WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 3};";
    session.query(create_ks).await.expect("⚠️ musiem keyspace create error");
    Ok(Arc::new(session)) //-- putting the(this) cluster(datacenter) and its nodes balanced with RoundRobin inside a compressed lz4 session


}