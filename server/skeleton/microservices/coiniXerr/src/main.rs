








#[macro_use]
mod constants;
mod schemas;
mod libs;
mod consensus;
mod utils;
mod apis;
use tokio::net::{TcpListener, TcpStream}; //-- async tcp listener and stream
use tokio::io::{AsyncReadExt, AsyncWriteExt}; //-- read from the input and write to the output - AsyncReadExt and AsyncWriteExt are traits which are implemented for an object of type TcpStream and based on orphan rule we must use them here to use the read() and write() method implemented for the object of TcpStream
use tokio::sync::mpsc; //-- to share values between multiple async tasks spawned by the tokio spawner which is based on green threads so shared state can be change only one at a time inside a thread 
use listenfd::ListenFd;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::{env, slice, mem};
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
    


    


    


    // NOTE - we can't borrow data inside Arc as mutable if we have a an object in which one of its method has &mut self as its first argument and needs to mutate a field like run_time_info add() method in which the info_dict field will be update 
    // NOTE - to solve above issue we have to put that object inside a Mutex to share and protect it between multiple threads and mutate or acquire the mutex by blocking the current thread when calling the lock() method, prevent from being in a dead lock, race condition and shared state situations
    let run_time_info = RuntimeInfo::new();
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(run_time_info)); //-- we can clone the run_time_info withou using Arc cause Clone trait is implemented for RuntimeInfo -> MetaData -> Miner actor
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let buffer_size = env::var("BUFFER_SIZE").expect("⚠️ please set buffer size in .env").parse::<usize>().unwrap();
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let coiniXerr_tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let blockchain = Chain::default(); //-- start the network by building a genesis block and a default transaction with 100 coins from the coiniXerr network wallet to the wildonion wallet
    let listener = TcpListener::bind(format!("{}:{}", host, coiniXerr_tcp_port)).await.unwrap();
    let pool = scheduler::ThreadPool::new(10);
    let (tx, mut rx) = mpsc::channel::<(TcpStream, Uuid, Arc<Mutex<RuntimeInfo>>)>(buffer_size); //-- mpsc channel to send the incoming stream, the generated uuid of the runtime info object and the runtime info object itself to multiple threads through the channel for each incoming connection from the socket 


    
    
    
    
    
    





    
    
    
    
    /////// ==============================================================================================================================   
    ///////                         starting miners' actors for incoming transactions' bytes through a tcp stream 
    /////// ==============================================================================================================================
    while let Ok((stream, addr)) = listener.accept().await{
        println!("-> connection stablished from miner [{}]", addr);
        let cloned_mutex_runtime_info_object = Arc::clone(&arc_mutex_runtime_info_object); //-- cloning (making a deep copy) runtime info object to prevent from moving in every iteration between threads
        let tx = tx.clone(); //-- we're using mpsc channel to send data between tokio tasks and each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning (making a deep copy) the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the mutex lock
        pool.execute(move || { //-- executing pool of threads for scheduling synchronous tasks using a messaging channel protocol called mpsc job queue channel in which its sender will send the job or task or message coming from the process constantly to the channel and the receiver inside an available thread (a none blocked thread) will wait until a job becomes available to down side of the channel finally the current thread must be blocked for the mutex (contains a message like a job) lock - every job or task has its own sender but only one receiver can be waited at a time inside a thread for mutex lock 
            tokio::spawn(async move { //-- spawning an async task (of socket process) inside a thread pool which will use a thread to start a miner actor in the background - a thread will be choosed to receive the task or job using the down side of the mpsc channel (receiver) to acquire the mutex for the lock operation
                // ----------------------------------------------------------------------
                //                  STARTING MINER ACTOR FOR THIS STREAM
                // ----------------------------------------------------------------------
                let miner = Miner{ //-- every peer is a miner
                    id: Uuid::new_v4(),
                    transaction: None, //-- signed and mined transaction - none when we're initializing a miner
                    recipient: None, //-- address of another miner - none when we're initializing a miner
                    rewards: None, //-- reward coins after mining transactions - none when we're initializing a miner
                };
                let miner_addr = miner.clone().start(); //-- cloning (making a deep copy) the miner actor will prevent the object from moving - trait Clone is implemented for Miner actor struct
                // ----------------------------------------------------------------------
                //                           SAVING RUNTIME INFO
                // ----------------------------------------------------------------------
                let meta_data_uuid = {
                    cloned_mutex_runtime_info_object.lock().unwrap().add(
                        MetaData{
                            address: stream.peer_addr().unwrap(),
                            actor: miner.clone(), //-- cloning (making a deep copy) the miner actor will prevent the object from moving
                        }
                    )
                };
                tx.send((stream, meta_data_uuid, cloned_mutex_runtime_info_object)).await.unwrap(); //-- sending the stream, the cloned runtime info and metadata uuid through the mpsc channel 
            }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
        });
    }
    while let Some((mut stream, generated_uuid, cloned_mutex_runtime_info_object)) = rx.recv().await{ //-- waiting for the stream, the generated uuid of the runtime info object and the runtime info object itself to become available to the down side of channel (receiver) to change the started miner actor for every incoming connection - stream must be mutable for reading and writing from and to socket
        tokio::spawn(async move { //-- this is an async task related to updating a miner actor on every incoming message from the sender which is going to be solved in the background on a single (without having to work on them in parallel) thread using green thread pool of tokio runtime and message passing channels like mpsc job queue channel protocol
            let mut transaction_buffer_bytes = [0 as u8; 1024];
            while match stream.read(&mut transaction_buffer_bytes).await{ //-- streaming over the incoming bytes from the socket - reading is the input and writing is the output
                Ok(size) if size == 0 => false, //-- socket closed
                Ok(size) => {
                    // ----------------------------------------------------------------------
                    //                              MINING PROCESS
                    // ----------------------------------------------------------------------
                    // TODO - limit transaction inside a block by calculating the size of the block after adding an incoming transaction from the auth microservice
                    // TODO - if the size of the current block was equal to 4 mb then we have to build another block for mining its transaction
                    // TODO - do the mining and consensus process here then send back the mined transaction inside the response to where it's called
                    // TODO - add mined block to the coiniXerr chain
                    // blockchain.add(mined_block);
                    // ...
                    let deserialized_transaction = &mut serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[0..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a Transaction object using serde_json::from_slice
                    deserialized_transaction.signed = Some(chrono::Local::now().naive_local().timestamp()); // TODO - this should be update after a successful signed contract and mined process
                    let signed_transaction_bytes: &[u8] = unsafe { //-- encoding process of new transaction by building the &[u8] using raw parts of the struct - serializing a new transaction struct into &[u8] bytes
                        //-- converting a const raw pointer of an object and its length into the &[u8], the len argument is the number of elements, not the number of bytes
                        //-- the total size of the generated &[u8] is the number of elements (each one has 1 byte size) * mem::size_of::<Transaction>() and it must be smaller than isize::MAX
                        //-- here number of elements or the len for a struct is the size of the total struct which is mem::size_of::<Transaction>()
                        slice::from_raw_parts(deserialized_transaction as *const Transaction as *const u8, mem::size_of::<Transaction>())
                    };
                    // ----------------------------------------------------------------------
                    //               UPDATING MINER ACTOR WITH A SIGNED TRANSACTION
                    // ----------------------------------------------------------------------
                    for (id, md) in cloned_mutex_runtime_info_object.lock().unwrap().info_dict.iter_mut(){ //-- id and md are &mut Uuid and &mut MetaData respectively - we have to iterate over our info_dict mutably and borrowing the key and value in order to update the miner actor transaction of our matched meta_data id with the incoming uuid
                        if id == &generated_uuid{
                            let dese_tra = serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[0..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a Transaction object using serde_json::from_slice
                            md.update_miner_transaction(Some(dese_tra)); //-- update the miner actor with a signed transaction
                        }
                    }
                    // ----------------------------------------------------------------------
                    //               SENDING SIGNED TRANSACTION BACK TO THE USER
                    // ----------------------------------------------------------------------
                    stream.write(&signed_transaction_bytes).await.unwrap(); //-- sends the signed transaction back to the user
                    true
                },
                Err(e) => {
                    println!("-> terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown().await.unwrap(); //-- shuts down the output stream
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
