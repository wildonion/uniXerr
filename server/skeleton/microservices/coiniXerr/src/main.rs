





/////// ==========--------------==========--------------==========--------------==========--------------==========--------------
///////                                      coiniXerr node design pattern explained
/////// ==========--------------==========--------------==========--------------==========--------------==========--------------
// a transaction of a transfer event might be send either using the http api or through
// a tcp stream to the coiniXerr server handled each one in parallel by a multithreading based scheduler; 
// an actor will be started on successful connection from every peer only in tcp mode. 
// once the transaction has received asynchronously and simultaneously they must be signed in order to send them 
// through the mpsc job queue channel to down side of the channel for mining process 
// and relatively for all users to have a successful transfer. they can only be signed 
// as long as the receiver of the transaction channel is waiting for the new transaction 
// and if the receiver was not able to receive caused by a sudden shutdown, dropped sender 
// (caused by joining the thread contains sender to stop the task from being processed in background) and 
// timeout or deadline issue that transaction will not be signed and the transfer process won't be 
// a successful event. of course if the transaction is not signed means there will be no mining process 
// cause the receiver is not waiting to receive anything from the sender to put them in a block for mining.
/////// ==========--------------==========--------------==========--------------==========--------------==========--------------






/////// ==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------
// NOTE - we can save each tokio::spawn() inside a vector of type JoinHandle (like OS threads) to await on them one by one later on to block their running background task to get the computation result of their async task or we can send their computation results through the mpsc job queue channel between other tasks             
// NOTE - tokio::spawn() is an asynchronous multithreaded (green threads) and event loop based task spawner and scheduler which takes an async task of type future of a process and shares it between its threads using its job queue channel protocol so every type in the task must be Send + Sync + 'static and cloneable
// NOTE - we can't borrow data inside Arc as mutable if we have a an object in which one of its method has &mut self as its first argument and needs to mutate a field like run_time_info add() method in which the info_dict field will be updated 
// NOTE - to solve above issue we have to put that object inside a Mutex (&mut) to share its ownership (Arc) and protect it between multiple threads and mutating or mutex acquisition is done by blocking the current thread when calling the lock() method, prevent from being in a dead lock and shared state situations
// NOTE - & is used to take a reference or borrow the ownership; for Copy trait bounded type this means coping the type by borrowing its ownership and &mut is a mutable pointer to that for changing it
// NOTE - Arc (we use Arc if the type wasn't bounded to Clone trait) is used to share the ownership of data and move it between threads safely with Mutex; we can change and mutate the data inside Arc by locking the local or current thread at runtime to acquire the mutex 
/////// ==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------








#[macro_use]
mod constants;
mod schemas;
mod actors;
mod engine;
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
use actix::{*, prelude::*}; //-- loading actix actors and handlers for validator actor's threads communication using their address and defined events 
use actix_web::{App, HttpServer, middleware, web};
use actix_session::CookieSession;
use apis::wallet::routes as coin_routes;
use crate::actors::{parathread::{Parachain, Communicate}, peer::{Validator, Contract}};
use crate::utils::scheduler;
use crate::schemas::{Transaction, RuntimeInfo, MetaData};
use crate::engine::contract::token::CRC20; //-- based on orphan rule we must use CRC20 here to use the mint() and other methods implemented for the validator actor













// #[tokio::main]
#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the actix_web as an event loop based runtime under the hood on top of tokio to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    


    











    let run_time_info = RuntimeInfo::new();
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(run_time_info)); //-- we can clone the run_time_info without using Arc cause Clone trait is implemented for RuntimeInfo -> MetaData -> Validator actor
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let buffer_size = env::var("BUFFER_SIZE").expect("⚠️ please set buffer size in .env").parse::<usize>().unwrap();
    let max_block_size = env::var("BUFFER_SIZE").expect("⚠️ please set block size in .env").parse::<usize>().unwrap();
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let coiniXerr_tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let listener = TcpListener::bind(format!("{}:{}", host, coiniXerr_tcp_port)).await.unwrap();
    let pool = scheduler::ThreadPool::new(10); //-- 10 threads per incoming stream to handle all its incoming transactions concurrently and simultaneously
    let (stream_sender, mut stream_receiver) = mpsc::channel::<(TcpStream, Uuid, Arc<Mutex<RuntimeInfo>>, Arc<Mutex<Addr<Validator>>>)>(buffer_size); //-- mpsc channel to send the incoming stream, the generated uuid of the runtime info object and the runtime info object itself to multiple threads through the channel for each incoming connection from the socket
    let (transaction_sender, mut transaction_receiver) = mpsc::channel::<Arc<Mutex<Transaction>>>(buffer_size); //-- transaction mempool channel - mpsc channel to send all transactions of all peers' stream to down side of the channel asynchronously for mining process
    println!("-> {} - server is up", chrono::Local::now().naive_local());
    
    





















    /////// ==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////      starting coiniXerr default parachain network by adding the genesis block to it and initializing the first block 
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    // TODO - update the slot field based on auctions and biddings
    // TODO - send message from first_parachain_addr to second_parachain_addr
    // ...
    println!("-> {} - starting default parachain", chrono::Local::now().naive_local());
    let parachain = Parachain{slot: None, blockchain: None, another_parachain: None, current_block: None};
    let first_parachain_addr = parachain.clone().start(); //-- building a new parachain actor - cloning (making a deep copy of) the parachain actor will prevent the object from moving and loosing ownership; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership - trait Clone is implemented for Parachain actor struct
    
    
    println!("-> {} - starting second parachain", chrono::Local::now().naive_local());
    let second_parachain = Parachain{slot: None, blockchain: None, another_parachain: None, current_block: None};
    let second_parachain_addr = second_parachain.clone().start(); //-- building a new parachain actor - cloning (making a deep copy of) the parachain actor will prevent the object from moving and loosing ownership; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership - trait Clone is implemented for Parachain actor struct
        

    let mut current_block = parachain.clone().current_block.unwrap(); //-- cloning (making a deep copy of) the parachain actor will prevent the object from moving and loosing ownership - we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    
    
    
    
    









    
    
    
    /////// ==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////                starting validators' actors for incoming regular transactions' bytes through a tcp stream 
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    while let Ok((stream, addr)) = listener.accept().await{ //-- await suspends the accept() function execution to solve the future but allows other code blocks to run  
        println!("-> {} - connection stablished from {}", chrono::Local::now().naive_local(), addr);
        let cloned_arc_mutex_runtime_info_object = Arc::clone(&arc_mutex_runtime_info_object); //-- cloning (making a deep copy of) runtime info object to prevent from moving in every iteration between threads
        let stream_sender = stream_sender.clone(); //-- we're using mpsc channel to send data between tokio tasks and each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning (making a deep copy of) the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the mutex lock
        // ----------------------------------------------------------------------
        //                 STARTING VALIDATOR ACTOR FOR THIS STREAM
        // ----------------------------------------------------------------------
        println!("-> {} - starting validator actor for this peer", chrono::Local::now().naive_local());
        let validator = Validator{ //-- every peer is a validator
            id: Uuid::new_v4(),
            addr, //-- socket address of this validator
            recent_transaction: None, //-- signed transaction - none when we're initializing a validator
        };
        let validator_addr = validator.clone().start(); //-- cloning (making a deep copy of) the validator actor will prevent the object from moving in every iteration - trait Clone is implemented for Validator actor struct
        // ----------------------------------------------------------------------
        //                  SAVING RUNTIME INFO FOR THIS STREAM
        // ----------------------------------------------------------------------
        println!("-> {} - saving runtime info", chrono::Local::now().naive_local());
        let meta_data_uuid = {
                cloned_arc_mutex_runtime_info_object.lock().unwrap().add( //-- locking on runtime info object (mutex) must be done in order to prevent other threads from mutating it at the same time 
                MetaData{
                    address: addr,
                    actor: validator.clone(), //-- cloning (making a deep copy of) the validator actor will prevent the object from moving in every iteration
                }
            )
        };
        pool.execute(move || { //-- parallel transactions handler - executing pool of threads for scheduling synchronous tasks spawned with tokio::spawn() using a messaging channel protocol called mpsc job queue channel in which its sender will send the job or task or message coming from the process constantly to the channel and the receiver inside an available thread (a none blocked thread) will wait until a job becomes available to down side of the channel finally the current thread must be blocked for the mutex (contains a message like a job) lock - mpsc definition : every job or task has its own sender but only one receiver can be waited at a time inside a thread for mutex lock 
            tokio::spawn(async move { //-- spawning an async task (of socket process) inside a thread pool which will use a thread to start a validator actor in the background - a thread will be choosed to receive the task or job using the down side of the mpsc channel (receiver) to acquire the mutex for the lock operation
                let arc_mutex_validator_addr = Arc::new(Mutex::new(validator_addr)); //-- creating an Arc object which is inside a Mutex to share and mutate data between threads cause Validator actor addr object doesn't implement Clone trait and the object inside Arc is not mutable thus we have to put the validator_addr object inside a mutex to be updatable between threads
                let cloned_arc_mutex_validator_addr = Arc::clone(&arc_mutex_validator_addr); //-- we're borrowing the ownership of the Arc-ed and Mutex-ed validator_addr object to move it between threads without loosing the ownership 
                println!("-> {} - sending stream setups through the channel", chrono::Local::now().naive_local());
                stream_sender.send((stream, meta_data_uuid, cloned_arc_mutex_runtime_info_object, cloned_arc_mutex_validator_addr)).await.unwrap(); //-- sending the stream, the cloned runtime info and metadata uuid through the mpsc channel 
            }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
        });
    }
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------















    /////// ==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////                                 waiting to receive stream and other setups asynchronously 
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    while let Some((mut stream, generated_uuid, cloned_arc_mutex_runtime_info_object, cloned_arc_mutex_validator_addr)) = stream_receiver.recv().await{ //-- waiting for the stream, the generated uuid of the runtime info object and the runtime info object itself to become available to the down side of channel (receiver) to change the started validator actor for every incoming connection - stream must be mutable for reading and writing from and to socket
        println!("-> {} - receiving the stream setups", chrono::Local::now().naive_local());
        let transaction_sender = transaction_sender.clone(); //-- cloning transaction_sender to send signed transaction through the channel to the receiver for mining process
        tokio::spawn(async move { //-- this is an async task related to updating a validator actor on every incoming message from the sender which is going to be solved in the background on a single (without having to work on them in parallel) thread using green thread pool of tokio runtime and message passing channels like mpsc job queue channel protocol
            let mut transaction_buffer_bytes = vec![0 as u8; buffer_size]; //-- using [0 as u8; buffer_size] gives us the error of `attempt to use a non-constant value in a constant` cause [u8] doesn't implement the Sized trait
            while match stream.read(&mut transaction_buffer_bytes).await{ //-- streaming over the incoming bytes from the socket - reading is the input and writing is the output
                Ok(size) if size == 0 => false, //-- socket closed
                Ok(size) => {
                    let deserialized_transaction_union = Transaction::new(&transaction_buffer_bytes[0..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes into the Transaction struct object using TransactionMem union
                    let deserialized_transaction_serde = &mut serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[0..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a Transaction object using serde_json::from_slice
                    // TODO - if the downside of the mpsc job queue channel was available the transaction will be signed and sent through the channel to be pushed inside a block for mining process
                    // ...
                    println!("-> {} - signing incoming transaction", chrono::Local::now().naive_local());
                    // ----------------------------------------------------------------------
                    //              SIGNING THE INCOMING TRANSACTION WITH SERVER TIME
                    // ----------------------------------------------------------------------
                    deserialized_transaction_serde.signed = Some(chrono::Local::now().naive_local().timestamp()); //-- signing the incoming transaction with server time
                    // ----------------------------------------------------------------------
                    //               SENDING SIGNED TRANSACTION BACK TO THE PEER
                    // ----------------------------------------------------------------------
                    // NOTE - encoding or serializing process is converting struct object into utf8 bytes
                    // NOTE - decoding or deserializing process is converting utf8 bytes into the struct object
                    let signed_transaction_serialized_into_bytes: &[u8] = unsafe { //-- encoding process of new transaction by building the &[u8] using raw parts of the struct - serializing a new transaction struct into &[u8] bytes
                        //-- converting a const raw pointer of an object and its length into the &[u8], the len argument is the number of elements, not the number of bytes
                        //-- the total size of the generated &[u8] is the number of elements (each one has 1 byte size) * mem::size_of::<Transaction>() and it must be smaller than isize::MAX
                        //-- here number of elements or the len for a struct is the size of the total struct which is mem::size_of::<Transaction>()
                        slice::from_raw_parts(deserialized_transaction_serde as *const Transaction as *const u8, mem::size_of::<Transaction>()) //-- it'll form a slice from the pointer to the struct and the total size of the struct which is the number of elements inside the constructed &[u8] array; means number of elements in constructing a &[u8] from a struct is the total size of the struct allocated in the memory
                    };
                    println!("-> {} - sending signed transaction back to the peer", chrono::Local::now().naive_local());
                    stream.write(&signed_transaction_serialized_into_bytes).await.unwrap(); //-- sending the signed transaction back to the peer
                    // ----------------------------------------------------------------------
                    //       UPDATING VALIDATOR ACTOR WITH THE LATEST SIGNED TRANSACTION
                    // ----------------------------------------------------------------------
                    println!("-> {} - updating validator actor with the recent signed transaction", chrono::Local::now().naive_local());
                    for (id, md) in cloned_arc_mutex_runtime_info_object.lock().unwrap().info_dict.iter_mut(){ //-- id and md are &mut Uuid and &mut MetaData respectively - we have to iterate over our info_dict mutably and borrowing the key and value in order to update the validator actor transaction of our matched meta_data id with the incoming uuid
                        if id == &generated_uuid{
                            let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&signed_transaction_serialized_into_bytes).unwrap(); //-- deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                            md.update_validator_transaction(Some(signed_transaction_deserialized_from_bytes)); //-- update the validator actor with a recent signed transaction
                        }
                    }
                    // ---------------------------------------------------------------------------------------
                    //      SENDING SIGNED TRANSACTION TO DOWN SIDE OF THE CHANNEL FOR CONSENSUS PROCESS
                    // ---------------------------------------------------------------------------------------
                    println!("-> {} - sending signed transaction to down side of the channel for consensus process", chrono::Local::now().naive_local());
                    let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&signed_transaction_serialized_into_bytes).unwrap(); //-- deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                    let arc_mutex_transaction = Arc::new(Mutex::new(signed_transaction_deserialized_from_bytes)); //-- putting the signed_transaction_deserialized_from_bytes inside a Mutex to borrow it as mutable inside Arc by locking the current thread 
                    let cloned_arc_mutex_transaction = Arc::clone(&arc_mutex_transaction); //-- cloning the arc_mutex_transaction to send it through the mpsc job queue channel 
                    transaction_sender.send(cloned_arc_mutex_transaction).await.unwrap(); //-- sending signed transaction through the mpsc job queue channel asynchronously for mining process
                    true
                },
                Err(e) => {
                    println!("-> {} - terminating connection with validator {}", chrono::Local::now().naive_local(), stream.peer_addr().unwrap());
                    stream.shutdown().await.unwrap(); //-- shuts down the output stream
                    false
                }
            } {} //-- it'll return true on its Ok() arm and false on its Err arm
        }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
    }
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------














    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////     waiting to receive signed transactions asynchronously from the sender to push them inside the current block - this buffer zone is the transaction mempool channel
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------==========--------------
    while let Some(transaction) = transaction_receiver.recv().await{ //-- waiting for each transaction to become available to the down side of channel (receiver) for mining process cause sending is done asynchronously 
        println!("-> {} - receiving new transaction to push inside the current block", chrono::Local::now().naive_local());
        let mutex_transaction = transaction.lock().unwrap().clone();
        println!("-> {} - new transaction : {:#?}", chrono::Local::now().naive_local(), mutex_transaction);
        // ----------------------------------------------------------------------
        //                            TRANSACTION TYPES
        // ----------------------------------------------------------------------
        if mutex_transaction.ttype == 0x00{
            // TODO - regular transaction
            // TODO - send issue contract message to validator actor
            todo!();
        } else if mutex_transaction.ttype == 0xFF{
            // TODO - issuing CRC21 smart contract
            // TODO - send issue contract message to validator actor
            let ttype = 0xFF;
            todo!();
        } else if mutex_transaction.ttype == 0x02{
            // TODO - issuing CRC20 smart contract
            // TODO - send issue contract message to validator actor
            let ttype = 0x02;
            todo!();
        } else if mutex_transaction.ttype == 0x03{
            // TODO - issuing CRC22 smart contract
            // TODO - send issue contract message to validator actor
            let ttype = 0x03;
            todo!();
        }
        // ----------------------------------------------------------------------
        //                             CONSENSUS PROCESS
        // ----------------------------------------------------------------------
        while std::mem::size_of_val(&current_block) <= max_block_size{ //-- returns the dynamically-known size of the pointed-to value in bytes by passing a reference or pointer to the value to this method - push incoming transaction into the current_block until the current block size is smaller than the max_block_size
            current_block.push_transaction(mutex_transaction.clone()); //-- cloning transaction object in every iteration to prevent from moving and loosing ownership - adding pending transaction from the mempool channel into the current block for validating that block
            if std::mem::size_of_val(&current_block) > max_block_size{
                // TODO - calculate the block and merkle_root hash
                todo!();
                println!("-> {} - shaping a new block to add transactions", chrono::Local::now().naive_local());
                let (prev, last) = {
                    let mut rev_iter = parachain.blockchain.clone().unwrap().blocks.iter().rev(); //-- cloning (making a deep copy of) the blockchain of the parachain actor will prevent the object from moving and loosing ownership - we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    (rev_iter.next().unwrap().to_owned(), rev_iter.next().unwrap().to_owned()) //-- converting &Block to Block by using to_owned() method in which cloning process will be used 
                };
                current_block = parachain.blockchain.clone().unwrap().build_raw_block(&prev); //-- passing the previous block by borrowing it - cloning (making a deep copy of) the blockchain of the parachain actor will prevent the object from moving and loosing ownership; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
            }
        }
        if let (Some(merkle_root), Some(block_hash)) = (current_block.clone().merkle_root, current_block.clone().hash){ //-- checking the block's hash and merkle_root hash for transactions finality
            println!("-> {} - validating process has been started for block [{}]", chrono::Local::now().naive_local(), current_block.id);
            current_block.is_valid = true;
            println!("-> {} - adding the created block to the chain", chrono::Local::now().naive_local());
            parachain.blockchain.clone().unwrap().add(current_block.clone()); //-- adding the cloned of current block to the coiniXerr parachain blockchain - cloning must be done to prevent current_block and the blockchain parachain from moving in every iteration transaction_receiver loop; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
        }
    }
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------

    







    





    
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    ///////                                                 actix HTTP web server
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                .service(
            web::scope("/coiniXerr")
                        .data(transaction_sender.clone()) //-- clone the transaction_sender to movie it between actix routes and threads 
                        .data(parachain.blockchain.clone().unwrap()) //-- clone the blockchain of the parachain to move it between actix routes and threads
                        .configure(coin_routes)
                    )
                .wrap(middleware::Logger::default())
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
        });
    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", host, coiniXerr_http_port))?
        }
    };
    server.run().await
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------





}
