




//// futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by using .await on it also try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
//// StreamExt is a trait for streaming utf8 bytes data - RemoteHandle is a handler for future objects which are returned by the remote_handle() method
use futures::{StreamExt, FutureExt}; 
use std::env;
use uuid::Uuid;
use dotenv::dotenv;
use std::sync::Arc;
use log::{info, error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use crate::hooopoeq::{Account, Topic};
use lapin::{
    Channel,
    Queue,
    options::*,
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, 
    Connection,
    ConnectionProperties,
    Result as LopinResult,
};
use utils::*; //// since utils is a lib thus we can use all its functions and modules by using it in here




#[path="pub.sub.rs"]
pub mod hooopoeq;









#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits; Box is an smart pointer which has valid lifetime for what's inside of it, we're putting the error part of the Result inside the Box since we have no idea about the size of the error or the type that caused this error happened at compile time thus we have to take a reference to it but without defining a specific lifetime
    






    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                  env vars setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈     
    
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    dotenv().expect("⚠️ .env file not found");
    let ampq_addr = env::var("AMQP_ADDR").expect("⚠️ no ampq address variable set");
    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("DB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("DB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");










    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                   hoopoe mq setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈      
    ////
    ////         publisher/subscriber app (rust or js code) 
    ////                      |
    ////                       ---- tcp socket
    ////                                       |
    ////                              rpc broker channels
    ////                                       |
    ////                                        --------- exchange
    ////                                                     |
    ////                             routing key ------- |binding| ------- routing key
    ////                                                     |
    ////                                             jobq queue buffer
    ////                                                     |
    ////                                                      --------- worker threadpool 
    ////
    //// ➔ publishers (rust or js code) which is connected to the mq broker can publish messages to a channel 
    ////    from there (inside the broker channels) messages will be buffered inside a specific queue.
    //// ➔ subscribers (rust or js code) want to subscribe to a specific message in which they must talk to a channel
    ////    then the channel will talk to the broker to get the message from a specific queue.
    //// ➔ rabbitmq uses queues instead of topics means that we can get all messages from a specific queues 
    ////    instead of subscribing to a specific topic by doing this all consumers can subscribe to a specific queue.  
    //// ➔ there might be multiple channels each of which are able to talk to a specific queue to get the buffered message from there.

    let sample_account_id = Uuid::new_v4().to_string();
    let mut account = Account::new(
                                    &ampq_addr,
                                    2, 
                                    sample_account_id
                                ).await;
    

                                




                                
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////         making queues, publish and subscribe
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    account //// making the hoop queue for publishing and subscribing process
        .make_queue("hoop")
        .await;
        
    account //// the publisher could be another app written in another lang
        .publish(10, "", "hoop") //// publishing 10 times on the passed in queue
        .await;

    account //// the subscriber could be another app written in another lang
        .subscribe("hoop") //// subscribing to the hoop queue
        .await;









    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                 app storage setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    let app_storage = db!{
        db_name,
        db_engine,
        db_host,
        db_port,
        db_username,
        db_password
    };









    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                     celery setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    // use celery for producing and consuming async tasks with a distributed message queues (the one that being used inside the rabbitmq)
    // ... 






    


    Ok(())



}