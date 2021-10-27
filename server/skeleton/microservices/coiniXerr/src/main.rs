


mod constants;
mod schemas;
mod peer;
mod consensus;
mod utils;
mod apis;
use crate::schemas::{Transaction, Chain, RuntimeInfo};
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;
use std::time::SystemTime;
use std::net::{TcpStream, TcpListener};
use actix_web::{App, HttpServer, middleware};
use actix_session::CookieSession;
use futures::{executor, join};
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use apis::wallet::routes as coin_routes;
use utils::jobq;
use std::sync::{Mutex, Arc};
use tokio::sync::mpsc;









// #[tokio::main]
#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the actix_web as a runtime on top of tokio to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    


    


    


    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env::set_var("RUST_LOG", "librdkafka=trace,rdkafka::client=debug");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let buffer_size = env::var("BUFFER_SIZE").expect("⚠️ please set buffer size in .env").parse::<usize>().unwrap();
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let coiniXerr_tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let kafka_host = env::var("KAFKA_HOST").expect("⚠️ please set kafka host in .env");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let node1_port = env::var("KAFKA_NODE1_PORT").expect("⚠️ please set kafka node1 port in .env"); //-- broker 1
    let node2_port = env::var("KAFKA_NODE2_PORT").expect("⚠️ please set kafka node2 port in .env"); //-- broker 2
    let node3_port = env::var("KAFKA_NODE3_PORT").expect("⚠️ please set kafka node3 port in .env"); //-- broker 3
    let broker1 = format!("{}:{}", kafka_host, node1_port);
    let broker2 = format!("{}:{}", kafka_host, node2_port);
    let broker3 = format!("{}:{}", kafka_host, node3_port);
    let brokers = format!("{},{},{}", broker1, broker2, broker3);
    let producer: &FutureProducer = &ClientConfig::new().set("bootstrap.servers", brokers).set("message.timeout.ms", "5000").create().expect("⚠️ producer creation error");
    let producer = producer.clone(); //-- we're clonning the producer cause we want to move it between tokio::spawn() green threads; since the Copy trait is not implement for the producer thus to borrow it we have to clone it
    let blockchain = Chain::default(); //-- start the network by building a genesis block and a default transaction from the coiniXerr network wallet to the wildonion wallet
    let listener = TcpListener::bind(format!("{}:{}", host, coiniXerr_tcp_port)).unwrap();
    let pool = jobq::ThreadPool::new(10);
    let run_time_info = RuntimeInfo::new();
    //-- we can't borrow data inside Arc as mutable cause run_time_info object has an add() method in which new runtime info will be appended to a hash map which has &mut self as its first argument
    //-- we have to put the run_time_info object inside a Mutex to share and protect it between multiple threads and mutate by blocking the current thread when calling the lock() method, prevent from being in a dead lock situation
    let arc_mutual_exclusion_run_time_info = Arc::new(Mutex::new(run_time_info)); //-- Mutex will block the current thread when calling the lock() method
    let (tx, mut rx) = mpsc::channel::<(TcpStream, Arc<Mutex<RuntimeInfo>>)>(buffer_size); //-- mpsc channel to send the incoming stream and run time info object for each incoming connection from the socket 
    















    /////// ==============================================================================================================================   
    ///////                         starting miners' actors using a tcp streamer for incoming transactions' bytes 
    /////// ==============================================================================================================================

    /* 
                  --------------------------------------------------------------------------------------------------------------
                / --------------------------------------------------------------------------------------------------------------
                | solving all incoming tasks of a process simultaneously inside the thread pool created for 
                | that process by sending each task into a free thread (one thread for each incoming task)
                | is done by using message passing channels like job queue channel protocol.
                | an example of above design pattern is actor which is a multithread task scheduler which communicates with
                | each other through their address (Addr object) and defined events (Messages).
                |
                |
                |
                | tokio::spawn() will spawn an async task (of type future) in the background (don’t need to await on them) 
                | so we can solve multiple tasks or multiple processes concurrently and simultaneously inside a single thread 
                | in the background of the app without making a thread pool for each process or task, since tokio::spawn() 
                | itself uses multiprocessing and green thread - threads that are scheduled by a runtime library or 
                | VM instead of natively by the underlying OS) concepts in its runtime for solving tasks. 
                \ --------------------------------------------------------------------------------------------------------------
                  --------------------------------------------------------------------------------------------------------------

                NOTE - actix actors are used for sending messages and events through their address (Addr object) instead of blocking the local thread for mutex acquisition using mpsc channel
                NOTE - all actix actors are on top of tokio in which every future task like actors communication events and messages will be handled by mpsc job queue channel and multithreading patterns
                NOTE - mpsc channel can be used to communicate between threads while we're using a thread pool to mutate a data structure by locking on the data (Mutex<T>) and blocking the local thread to acquire the mutex and prevent other thread from mutating and locking it at a same time to avoid being in dead lock situation
                NOTE - the sender of mpsc channel can be owned by multiple threads but the receiver can only be owned by only one thread at a time, that's because it's called multi producer and single consumer  
                NOTE - we have to clone the receiver for passing between multiple threads and for mutating what's in it we have to put it inside a Mutex to insure that only one thread can change the content of the receiver at a time
                NOTE - mutex acquisition is done by waiting on the receiver until a job or task becomes available to down side of the channel then locking on the receiver to acquire the mutex which will block the threads waiting for the lock to become available
                NOTE - multiple producers or workers own the receiver (Ac<T>) and single consumer or worker get the job at a time from the receiver (Mutex<T>)
                NOTE - we can share sender of the mpsc job queue channel between multiple threads by getting a clone from it but this is not the same for the receiver
                NOTE - we'll spawn some threads inside a pool like four threads for every process like socket connection to schedule and solve all its incoming tasks 
                NOTE - tasks or jobs of a process can be a massive computational data or a bytes of a file from evey connection
                NOTE - tasks or jobs of a process can be solved simultaneously inside one of the opened threads using a queue channel like mpsc
                NOTE - tasks or jobs of a process can be sent to multiple threads using sender of mpsc channel and only one thread can solve it at a time using the receiver of mpsc job queue channel
                NOTE - if a thread was busy another thread will be spawned to handle new task or job coming from the process
                NOTE - task scheduler is done through threads communication based on message passing channels like mpsc job queue channel or actors to avoid dead lock, shared state and race condition 
                NOTE - we can send a computation result inside the tokio::spawn() through mpsc job queue channel and let the task inside tokio::spawn() be run in the background
                NOTE - we can save each tokio::spawn() inside a variable which of type JoinHandle to await on them later on to block their running background task to get the computation result of their async task
    
    */

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => { //-- we have a success connection
                let cloned_mutex_run_time_info_obj = Arc::clone(&arc_mutual_exclusion_run_time_info); //-- cloning run_time_info struct using Arc cause the mentioned struct doesn't implement the trait Clone since we can't implement for it; Clone is not implemented for Addr<Miner>
                let tx = tx.clone(); //-- each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the mutex lock
                pool.execute(move || {
                    tokio::spawn(async move { //-- starting a miner actor in the background
                        tx.send((stream, cloned_mutex_run_time_info_obj)).await.unwrap(); //-- sending the stream and the cloned mutex runtime info through the mpsc channel 
                        crate::peer::node::start_miner_actor(rx).await; //-- starting a miner actor for every incoming connection; this is an async task which is going to be solved in the background on a single thread using green thread pool of tokio runtime and message passing channels like mpsc job queue channel protocol
                    }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
                });
            }, 
            Err(e) => {
                println!("-> stream error! - {}", e);
            }
        }
    }


  
    

    





    

    /////// ========================================== kafka producer server ==========================================
    // NOTE - kafka => multiple cluster (datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
    // NOTE - three replicas in kafka means there are three copies of each topics' partitions (buck of events) in each node (kafka broker)
    // NOTE - kafka partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
    // NOTE - the default number of partitions in kafka for each topic is 10
    // NOTE - tokio::spawn() can run many tasks concurrently in the background on a single thread without having to work on them in parallel
    // NOTE - we can spawn tasks of our process using tokio::spawn() inside a thread pool cause tokio can run its tasks concurrently on a single thread
    // NOTE - tokio::spawn() is an asynchronous multithreaded and event loop based task spawner and scheduler which takes a green thread based task of type future of a process and shares it between its threads using its job queue channel protocol so every type in the task must be Send + Sync + 'static and cloneable
    tokio::spawn(async move {
        let mut i = 0_usize;
        let heavy_number = 2;
        let async_block = async move { //-- this is a future object which is bounded to Future trait and it's not like the nodejs event loop
            heavy_number + 3 * 274
        };
        let blocked_thread_future = executor::block_on(async_block); //-- block_on will run a future to completion on the current thread and block the thread (execution gets stuck there) until the given future has completed (release the mutex) 
        // let suspend_execution_future = async_block.await; //-- we can only use .await inside an async function or block - .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
        // let joined_futures = join!(async_block); //-- we can only use join!() inside an async function or block - join!() complte multiple futures at the same time
        // let joined_tokio_futures = tokio::join!(async_block); //-- join!() is like .await but can wait for multiple futures concurrently, completing multiple futures at the same time
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
    }); //-- awaiting on tokio::spawn() will block the current task which is running in the background like creating a thread and immediately joining it
    /////// ===========================================================================================================















    
    /////// ============================ actix HTTP web server ============================
    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                .data(Arc::clone(&arc_mutual_exclusion_run_time_info)) //-- clone the arc_mutual_exclusion_run_time_info to move it between actix routes and threads
                .data(blockchain.clone()) //-- clone the blockchain to move it between actix routes and threads
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
