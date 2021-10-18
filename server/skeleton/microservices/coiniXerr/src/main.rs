


mod constants;
mod schemas;
mod peer;
mod consensus;
mod utils;
mod apis;
use crate::schemas::block::{Transaction, Block, Chain};
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;
use std::time::SystemTime;
use actix_web::{App, HttpServer, middleware};
use actix_session::CookieSession;
use futures::{executor, join};
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use apis::wallet::routes as coin_routes;
use uuid::Uuid;






// #[tokio::main]
#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the actix_web as a runtime on top of tokio to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    


    
    
    /////////////// ===================================================================== ///////////////
    /////////////// NOTE - INITIALIZING THE coiniXerr NETWORK BY CREATING A GENESIS BLOCK
    /////////////// ===================================================================== ///////////////
    // let blockchain = Chain::new(uuid::new_v4(), "main".to_string(), vec![Block::default()]) //-- creating another branch or fork
    let blockchain = Chain::default();
    // blockchain.add(mined_block);





    
    
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env::set_var("RUST_LOG", "librdkafka=trace,rdkafka::client=debug");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let kafka_host = env::var("KAFKA_HOST").expect("⚠️ please set kafka host in .env");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let node1_port = env::var("KAFKA_NODE1_PORT").expect("⚠️ please set kafka node1 port in .env"); //-- broker 1
    let node2_port = env::var("KAFKA_NODE2_PORT").expect("⚠️ please set kafka node2 port in .env"); //-- broker 2
    let node3_port = env::var("KAFKA_NODE3_PORT").expect("⚠️ please set kafka node3 port in .env"); //-- broker 3
    










    /////// ========================================== run peer actor ==========================================
    peer::actor::run().await; //-- running peer actor
    /////// ====================================================================================================
  
    



    



    

    /////// ========================================== kafka producer server ==========================================
    let broker1 = format!("{}:{}", kafka_host, node1_port);
    let broker2 = format!("{}:{}", kafka_host, node2_port);
    let broker3 = format!("{}:{}", kafka_host, node3_port);
    let brokers = format!("{},{},{}", broker1, broker2, broker3);
    // NOTE - kafka => multiple cluster (datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
    // NOTE - three replicas in kafka means there are three copies of each topics' partitions (buck of events) in each node (kafka broker)
    // NOTE - kafka partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
    // NOTE - the default number of partitions in kafka for each topic is 10.
    let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("⚠️ producer creation error");
    let producer = producer.clone(); //-- we're clonning the producer cause we want to move it between tokio::spawn() threads thus according to rust ownership we have to take a reference to the producer using clone() cause trait Copy is not imeplemented for that
    tokio::spawn(async move{ //-- tokio is based on event loop - tokio::spawn() takes a task of type future and shares it between multiple threads using its job queue channel protocol, so every type in the task must be Send + Sync and cloneable
        let mut i = 0_usize;
        let heavy_number = 2;
        let async_block = async move { //-- this is a future object which is bounded to Future trait and it's not like the nodejs event loop
            heavy_number + 3 * 274
        };
        let blocked_thread = executor::block_on(async_block); //-- block_on will run a future to completion on the current thread and block the thread (execution gets stuck there) until the given future has completed (release the mutex) 
        // let suspend_execution = async_block.await; //-- .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
        // let joined_futures = join!(async_block); //-- we can only use join!() inside an async function or block - join!() complte multiple futures at the same time
        // let joined_tokio = tokio::join!(async_block); //-- join!() is like .await but can wait for multiple futures concurrently, completing multiple futures at the same time  
        loop {
            let transaction_event = Transaction::default();
            let topic = transaction_event.id.to_string(); //-- every transaction is a topic
            let transaction_event_json = serde_json::to_string_pretty(&transaction_event).expect("⚠️ failed to serialize transaction event"); //-- serializing the struct into json
            let transaction_data: Transaction = serde_json::from_str(&transaction_event_json).expect("⚠️ failed to deserialize transaction json"); //-- deserializing the json into struct
            let key = &i.to_string(); //-- setting the key for this event
            let devlivery_status = producer.send_result( //-- we're using FutureRecord for sending the message or the event asynchoronously to all consumers cause send_result() method takes a FutureRecord to send a message
            FutureRecord::to(&topic)
                        .key(key)
                        .payload(&transaction_event_json) //-- we can send serde json inside the payload
                        .headers(OwnedHeaders::new().add("wo_header_key", "wo_header_value"))
                        .timestamp(
                            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
                                Ok(n) =>  n.as_secs() as i64,
                                Err(_) => { panic!("SystemTime before UNIX EPOCH!") },
                            }
                        )
            );
            println!("[+] Delivery status for Player data with imei {} inside iteration key {} received", topic, i);
            match devlivery_status{ //-- devlivery_status is a Result of delivery future and in order to solve it we have to await on it 
                Ok(delivery) => {
                    let solved_delivery = delivery.await.unwrap().unwrap();
                    info!("[+] Delivery solved {:?}", solved_delivery);
                },
                Err(e) => {
                    error!("[!] Delivery error {:?}", e);
                }
            }

            i += 1;
        }
    });
    /////// ===========================================================================================================






    
    /////// ============================ actix HTTP web server ============================
    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                .wrap(middleware::Logger::default())
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
                .configure(coin_routes)
        });
    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", host, coiniXerr_http_port))?
        }
    };
    server.run().await
    /////// ===============================================================================




}
