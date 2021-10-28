









mod constants;
mod schemas;
mod libs;
mod consensus;
mod utils;
mod apis;
use tokio::net::{TcpListener, TcpStream}; //-- async tcp listener and stream
use tokio::io::{AsyncReadExt, AsyncWriteExt}; //-- read from the input and write to the output - AsyncReadExt and AsyncWriteExt are traits which are implemented for an object of type TcpStream and based on orphan rule we must use them here to use the read() and write() method implemented for the object of TcpStream
use tokio::sync::mpsc; //-- to share values between multiple async tasks spawned by the tokio spawner
use listenfd::ListenFd;
use std::env;
use std::sync::{Mutex, Arc};
use dotenv::dotenv;
use actix::{*, prelude::*}; //-- loading actix actors and handlers for threads communication using their address and defined events 
use actix_web::{App, HttpServer, middleware};
use actix_session::CookieSession;
use futures::{executor, join};
use apis::wallet::routes as coin_routes;
use crate::libs::actors::{Miner, Ping};
use crate::libs::scheduler;
use crate::schemas::{Transaction, Chain, RuntimeInfo, MetaData};








// #[tokio::main]
#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the actix_web as a runtime on top of tokio to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    


    


    


    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let buffer_size = env::var("BUFFER_SIZE").expect("⚠️ please set buffer size in .env").parse::<usize>().unwrap();
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let coiniXerr_tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let blockchain = Chain::default(); //-- start the network by building a genesis block and a default transaction from the coiniXerr network wallet to the wildonion wallet
    let listener = TcpListener::bind(format!("{}:{}", host, coiniXerr_tcp_port)).await.unwrap();
    let pool = scheduler::ThreadPool::new(10);
    let run_time_info = RuntimeInfo::new();
    //-- we can't borrow data inside Arc as mutable cause run_time_info object has an add() method in which new runtime info will be appended to a hash map which has &mut self as its first argument
    //-- we have to put the run_time_info object inside a Mutex to share and protect it between multiple threads and mutate by blocking the current thread when calling the lock() method, prevent from being in a dead lock situation
    let arc_mutual_exclusion_run_time_info = Arc::new(Mutex::new(run_time_info)); //-- Mutex will block the current thread when calling the lock() method
    //-- since the receiver must be mutable we can't clone the receiver using Arc cause the value inside Arc can't be mutate; to solve this we have to put the receiver inside Mutex then clone it using Arc
    //-- trait Send is not implemented for the receiver thus in total, we can't have this type : Arc<Mutex<receiver>> to share the receiver for starting miners' actors
    let (tx, mut rx) = mpsc::channel::<(TcpStream, Arc<Mutex<RuntimeInfo>>)>(buffer_size); //-- mpsc channel to send the incoming stream and run time info object to multiple threads through the channel for each incoming connection from the socket 










    




    /////// ==============================================================================================================================   
    ///////                         starting miners' actors for incoming transactions' bytes through a tcp stream 
    /////// ==============================================================================================================================
    while let Ok((stream, addr)) = listener.accept().await{
        println!("-> connection stablished from miner [{}]", addr);
        let cloned_mutex_run_time_info_obj = Arc::clone(&arc_mutual_exclusion_run_time_info); //-- cloning mutex runtime info object using Arc cause the mentioned struct doesn't implement the trait Clone since we can't implement for it; Clone is not implemented for Addr<Miner>
        let tx = tx.clone(); //-- each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the semaphore or mutex lock
        pool.execute(move || { //-- executing pool of threads for handling synchronous tasks using a messaging channel protocol called mpsc job queue channel in which its sender will send the job or task or message to the channel and the receiver inside a thread will wait until a job becomes available to down side of the channel finally the current thread must be blocked for the mutex (contains a message like a job) lock - every job or task has its own sender but only one receiver can be waited for job for mutex lock at a time inside a thread 
            tokio::spawn(async move { //-- spawning an async task (of socket process) inside a thread pool which will use a thread to start a miner actor in the background - a thread will be choosed to receive the task or job using the down side of the mpsc channel (receiver) to acquire the mutex for the lock operation
                tx.send((stream, cloned_mutex_run_time_info_obj)).await.unwrap(); //-- sending the stream and the cloned mutex runtime info through the mpsc channel 
            }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
        });
    }
    while let Some((mut stream, run_time_info)) = rx.recv().await{ //-- waiting for the stream and the cloned mutex runtime info to become available to the down side of channel (receiver) to start a miner actor for every incoming connection - stream must be mutable for reading and writing from and to socket
        tokio::spawn(async move { //-- this is an async task related to starting a miner actor which is going to be solved in the background on a single (without having to work on them in parallel) thread using green thread pool of tokio runtime and message passing channels like mpsc job queue channel protocol
            let mut transaction_buffer_bytes = [0 as u8; 1024];
            while match stream.read(&mut transaction_buffer_bytes).await{ //-- keep socket always open
                Ok(size) if size == 0 => false, //-- socket closed
                Ok(size) => {
                    stream.write(&transaction_buffer_bytes[0..size]).await.unwrap();
                    let miner = Miner::create(|ctx| {
                        // now we can get an address of the first actor and create the second actor
                        let addr = ctx.address();
                        let addr2 = Miner {
                            transaction: serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[0..size]).unwrap(), //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a Transaction object using serde_json::from_slice
                            name: String::from("Miner 2"),
                            recipient: addr.recipient(),
                        }
                        .start();
                        // let's start pings
                        addr2.do_send(Ping { id: 10 });
                        // now we can finally create first actor
                        let miner = Miner {
                            transaction: serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[0..size]).unwrap(), //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a Transaction object using serde_json::from_slice
                            name: String::from("Miner 1"),
                            recipient: addr2.recipient(),
                        };
                        miner
                    });
                    run_time_info.lock().unwrap().add(
                        MetaData{
                            address: stream.peer_addr().unwrap(),
                            buffer: transaction_buffer_bytes[0..size].to_owned(), //-- to_owned() creates owned data from borrowed data, usually by cloning - &[u8] to Vec<u8>
                            actor: miner,
                        }
                    );
                    true
                },
                Err(e) => {
                    println!("-> terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown().await.unwrap(); //-- shuts down the output stream
                    // stream.shutdown(Shutdown::Both).await.unwrap(); //-- both the reading and the writing portions of the TcpStream should be shut down
                    false
                }
            } {} //-- it'll return true on its Ok() arm and false on its Err arm
        }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
    }
    

    








    

    
    /////// ========================================================
    ///////                 actix HTTP web server
    /////// ========================================================
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




}
