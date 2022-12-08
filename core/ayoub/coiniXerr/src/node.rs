



/*







Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  




        ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
               coiniXerr node design pattern explained
        ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

        a coiniXerr node is a p2p tcp (using zmq) and rpc (capn'n proto) based both server and client at the same time.
        a transaction can be sent through a tcp using a zmq publisher or an rpc stream with cap'n proto serialization
        from the walleXerr or another node, each of which will be handled in parallel using tokio, an actor will be started on 
        successful connection from every peer. 
        once the transaction has received asynchronously and simultaneously they must be signed in order to send them through 
        the mpsc job queue channel to down side of the mempool channel for mining process and relatively for all users to have 
        a successful transfer. They can only be signed as long as the receiver of the transaction channel or the mempool is 
        waiting for the new transaction and if the receiver was not able to receive caused by a sudden shutdown, dropped sender 
        (caused by joining the thread contains sender to stop the task from being processed in background) and timeout or deadline 
        issue that transaction will not be signed and the transfer process won't be a successful event. 
        of course if the transaction is not signed means there will be no mining process cause the receiver is not waiting 
        to receive anything from the sender to put them in a block for mining.
        The main structure of the coiniXerr network is its parachains in which every parachain has its own blockchain stuffs and an
        special field called slot in which every 5 seconds a reset task will be scheduled to be executed on the selected parachain to 
        reset all its feilds by a successful auction process by the coiniXerr validators.
        It must be mentioned that each parachain and validator is an actor that can communicate with each other by sending message,  
        also there are channels related to parachain and validator actors used to broadcast new transaction income, update a specific validator 
        and parachain state, new validator and parachain joined and block mining process message events to related subscribers.
        All subscribers of the mentioned channels which are interested on specific topic must support the meesage event of the topic 
        in order to be able subscribe to that topic.
      





*/









// #![allow(unused)] //-- will let the unused vars be there - we have to put this on top of everything to affect the whole crate
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate

use std::fmt;
use is_type::Is;
use rayon::prelude::*;
use log::{info, error, LevelFilter};
use tokio::net::{TcpListener, TcpStream, UdpSocket}; //-- async tcp listener and stream
use tokio::io::{AsyncReadExt, AsyncWriteExt}; //-- read from the input and write to the output - AsyncReadExt and AsyncWriteExt are traits which are implemented for an object of type TcpStream and based on orphan rule we must use them here to use the read() and write() method asyncly which has been implemented for the object of TcpStream (these trait have been implemented for TcpStream structure)
use tokio::sync::mpsc; //-- to share values between multiple async tasks spawned by the tokio spawner which is based on green threads so shared state can be change only one at a time inside a thread 
use uuid::Uuid;
use std::{fmt::Write, num::ParseIntError};
use std::sync::{Arc, Mutex, mpsc as std_mpsc, mpsc::channel as heavy_mpsc}; //-- communication between threads is done using mpsc job queue channel and end of the channel can only be owned by one thread at the time to avoid being in deadlock and race condition situations, however the sender half can be cloned and through such cloning the conceptual sender part of a channel can be shared among threads which is how you do the multi-producer, single-consumer part
use std::time::{Instant, Duration};
use std::{env, thread::{self, JoinHandle}};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::net::SocketAddr; //-- these structures are not async; to be async in reading and writing from and to socket we must use tokio::net 
use std::collections::{HashMap, HashSet};
use dotenv::dotenv;
use riker::actors::*;
use riker::system::ActorSystem;
use riker_patterns::ask::*; //// used to ask any actor to give us the info about or update the state of its guarded type 
use crate::actors::{
                    parathread::{Parachain, Communicate as ParachainCommunicate, Cmd as ParachainCmd, UpdateParachainEvent, ParachainCreated, ParachainUpdated}, //// parathread message evenrs
                    peer::{Validator, Contract, Mode as ValidatorMode, Communicate as ValidatorCommunicate, Cmd as ValidatorCmd, UpdateMode, UpdateTx, ValidatorJoined, ValidatorUpdated, UpdateValidatorAboutMempoolTx, UpdateValidatorAboutMiningProcess}, //// peer message events
                    rafael::env::{Serverless, MetaData, Runtime as RafaelRt, EventLog, EventVariant, RuntimeLog, LinkToService} //-- loading Serverless trait to use its method on Runtime instance (based on orphan rule) since the Serverless trait has been implemented for the Runtime type
                }; 
use crate::schemas::{Transaction, Block, Slot, Chain, Staker, Db, Storage, Mode};
use crate::engine::contract::token::CRC20; //-- based on orphan rule we must use CRC20 here to use the mint() and other methods implemented for the validator actor
use mongodb::Client;
//// futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by using .await on it also try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
//// StreamExt is a trait for streaming utf8 bytes data - RemoteHandle is a handler for future objects which are returned by the remote_handle() method
use futures::{Future, StreamExt, FutureExt, executor::block_on, future::RemoteHandle}; 
use serde::{Deserialize, Serialize};
use rand::Rng;
use borsh::{BorshDeserialize, BorshSerialize};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use daemon; //// import lib.rs methods




pub mod constants;
pub mod schemas;
pub mod actors;
pub mod engine;
pub mod utils; //// we're importing the utils.rs in here as a public module thus we can access all the modules, functions and macros inside of it publicly











#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits; Box is an smart pointer which has valid lifetime for what's inside of it, we're putting the error part of the Result inside the Box since we have no idea about the size of the error or the type that caused this error happened at compile time thus we have to take a reference to it but without defining a specific lifetime
    

    



    // daemon::trash().await;
    // daemon::mactrait().await;
    // daemon::unsafer().await;

    















    
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
    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("DB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("DB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let mut run_time_info = RafaelRt(HashMap::new());
    let runtime_instance = run_time_info.run(); //-- run() method is the method of the Rafael serverless trait
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(runtime_instance)); //-- we can clone the runtime_instance without using Arc cause Clone trait is implemented for RafaelRt -> MetaData -> Validator actor
    let buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ please set buffer size in .env").parse::<usize>().unwrap();
    let max_block_size = env::var("IO_BUFFER_SIZE").expect("⚠️ please set block size in .env").parse::<usize>().unwrap();
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let rpc_port = env::var("RPC_PORT").expect("⚠️ please set rpc port in .env");
    let coiniXerr_tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let rpc_addr = format!("{}{}", host, rpc_port).as_str();
    let zmq_addr = env::var("ZMQ_ADDR").expect("⚠️ no zmq addr variable set");
    let (stream_sender, mut stream_receiver) = mpsc::channel::<(
                                                                                                                                TcpStream, 
                                                                                                                                Uuid, 
                                                                                                                                Arc<Mutex<RafaelRt>>, 
                                                                                                                                Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
                                                                                                                                Arc<Mutex<ActorRef<ChannelMsg<ValidatorUpdated>>>>, //// each channels are actors and actors in riker are of type ActorRef which can be cloned and send across threads since they are send sync and have a valid lifetime to share between threads
                                                                                                                                //// passing the coiniXerr actor system through the mpsc channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
                                                                                                                                //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
                                                                                                                                ActorSystem 
                                                                                                                                //// there is no need to pass other actor channels through stream channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
                                                                                                                            )>(buffer_size); //-- mpsc channel to send the incoming stream, the generated uuid of the runtime info object and the runtime info object itself to multiple threads through the channel for each incoming connection from the socket
    let (mempool_sender, mut mempool_receiver) = mpsc::channel::<(
                                                                                                                                Arc<Mutex<Transaction>>, 
                                                                                                                                Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
                                                                                                                                //// passing the coiniXerr actor system through the mpsc channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
                                                                                                                                //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
                                                                                                                                ActorSystem 
                                                                                                                                //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
                                                                                                                            )>(buffer_size); //-- transaction mempool channel using mpsc channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel
    
    

    








                                                                                                                            
    
    
    




    













    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                             bootstrapping the app 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    // ----------------------------------------------------------------------
    //                          DEFINING APP STORAGE
    // ----------------------------------------------------------------------

    let app_storage = db!{ //// this publicly has exported inside the utils so we can access it here
        db_name,
        db_engine,
        db_host,
        db_port,
        db_username,
        db_password
    };
    let unwrapped_storage = app_storage.unwrap(); //-- unwrapping the app storage to create a db instance
    let db_instance = unwrapped_storage.get_db().await.unwrap(); //-- getting the db inside the app storage; it might be None
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr ACTOR SYSTEM
    // ----------------------------------------------------------------------

    let coiniXerr_sys = SystemBuilder::new()
                                                    .name("coiniXerr")
                                                    .create()
                                                    .unwrap(); //// unwrapping the last functional method 
    info!("➔ 🟢 actor system and storage are set up");

    // ----------------------------------------------------------------------
    //                       STARTING TOKIO TCP SERVER
    // ----------------------------------------------------------------------

    let listener = TcpListener::bind(format!("{}:{}", host, coiniXerr_tcp_port)).await.unwrap();
    info!("➔ 🟢 tcp listener is ready");

    // ----------------------------------------------------------------------
    //                    STARTING TRANSACTION EMULATORS
    // ----------------------------------------------------------------------    
    // if dotenv initialization is before the starting the emulator process means we're ok since the whole env file will be loaded into the ram and 
    // when we want to load vars it's ok but if we put the starting the emulator process before loading dotenv we'll face error since dotenv doesn't initialize yet.

    utils::tx_emulator().await;
    utils::tx_emulator_udp().await;













    




































    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////          coiniXerr nodes and walleXerr communications using cap'n proto serialization based on rpc and zmq protocols
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    // ---------------------------------------------------------------------------------------------------------------------------
    //          ZMQ PUBLISHER AND SUBSCRIBER USING CAP'N PROTO SERIALIZATION (DESIGNED FOR coiniXerr NODES COMMUNICATION)
    // ---------------------------------------------------------------------------------------------------------------------------
    //// ZeroMQ sockets may be connected to multiple endpoints, while simultaneously accepting incoming connections from 
    //// multiple endpoints bound to the socket, thus allowing many-to-many relationships.
    //
    // 
    //// zmq contexts are thread safe data types means we can clone them to share between threads (they are Arc-ed) 
    //// and also they avoid deadlocks since zmq socket protocols use actors under the hood means 
    //// both senders and receivers are actors which use a buit in jobq to handle incoming tasks and jobs. 
    
    let zmq_ctx = zmq::Context::new(); 
    let responder = zmq_ctx.socket(zmq::REP).unwrap(); //// server
    let requester = zmq_ctx.socket(zmq::REQ).unwrap(); //// client 
    let mut msg = zmq::Message::new(); //// a message is a single frame which can be any type, either received or created locally and then sent over the wire through the zmq socket
    assert!(responder.bind(zmq_addr.as_str()).is_ok());
    assert!(requester.connect(zmq_addr.as_str()).is_ok());
    info!("➔ 🟢 zmq is ready on tcp");

    // TODO - send and receive async transaction between coiniXerr nodes
    // TODO - use cap'n proto as the serialization protocol for transaction encoding
    // TODO - validating incoming transaction tasks using validator actors  
    // TODO - fix p2p nat issue with upnp and ngrok
    //  ...
    
    responder.recv(&mut msg, 0).unwrap(); //// this node receives cap'n proto transaction data from other node (client)
    info!("➔ 🟢 Received from clients or other nodes {}", msg.as_str().unwrap());
    responder.send("➔ 🟢 Sending World to clients or other nodes", 0).unwrap(); //// this node sends cap'n proto transaction data to other node (client)

    requester.send("➔ 🟢 Sending Hello to servers or other nodes ", 0).unwrap(); //// this node sends cap'n proto transaction data from other node (server)
    info!("➔🟢 Received World from servers or other nodes {}", msg.as_str().unwrap());
    requester.recv(&mut msg, 0).unwrap(); //// this node receives cap'n proto transaction data to other node (server)

    // -----------------------------------------------------------------------------------------------
    //          RPC SERVER AND CLIENT USING CAP'N PROTO SERIALIZATION (DESIGNED FOR waleXerr)
    // -----------------------------------------------------------------------------------------------
    
    
    
    // https://github.com/capnproto/capnproto-rust/tree/master/capnp-rpc
    // start server and get requests from other rpc nodes in here
    // send requests from here to the other rpc nodes in cap'n proto format
    // ... 
    
    for worker in 0..10{ //// spawning tokio green threads for 10 workers
        tokio::spawn(async move{ //// spawning tokio worker green threadpool to solve async task
    
        });
    }


























































    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                        building coiniXerr events channels 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    let validator_joined_channel: ChannelRef<ValidatorJoined>              = channel("validator-joined-channel", &coiniXerr_sys).unwrap(); //// validator actors which are interested in this message event (the message type is supported by and implemented for all validator actors) must subscribe to all topics (like joining a new validator) of this event for validator_joined_channel channel actor
    let validator_updated_channel: ChannelRef<ValidatorUpdated>            = channel("validator-updated-channel", &coiniXerr_sys).unwrap(); //// validator actors which are interested in this message event (the message type is supported by and implemented for all validator actors) must subscribe to all topics (like updating a validator) of this event for validator_updated_channel channel actor
    let parachain_created_channel: ChannelRef<ParachainCreated>            = channel("parachain-created-channel", &coiniXerr_sys).unwrap(); //// parachain actors which are interested in this message event (the message type is supported by and implemented for all parachain actors) must subscribe to all topics (like creating a new parachain) of this event for parachain_created_channel channel actor
    let parachain_updated_channel: ChannelRef<ParachainUpdated>            = channel("parachain-updated-channel", &coiniXerr_sys).unwrap(); //// parachain actors which are interested in this message event (the message type is supported by and implemented for all parachain actors) must subscribe to all topics (like updating a parachain) of this event for parachain_updated_channel channel actor
    let mempool_updated_channel: ChannelRef<UpdateValidatorAboutMempoolTx> = channel("mempool-transaction-joined-channel", &coiniXerr_sys).unwrap(); //// validator actors which are interested in this message event (the message type is supported by and implemented for all validator actors) must subscribe to all topics (like incoming a new transaction inside the mempool channel) of this event for mempool_updated_channel channel actor
    let mining_channel: ChannelRef<UpdateValidatorAboutMiningProcess>      = channel("mining-channel", &coiniXerr_sys).unwrap(); //// validator actors which are interested in this message event (the message type is supported by and implemented for all validator actors) must subscribe to all topics (like starting mining process) of this event for mining_channel channel actor



























    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                   starting coiniXerr parachain networks 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// the ask model which is using a oneshot channel on behalf and all actors use 
    //// message passing channel algos on behalf is used from main() to communicating 
    //// between actors and allows values to be sent by actors to outside of the actor 
    //// system because main() itself is not an actor and cannot receive messages, 
    //// this pattern is useful in context of an HTTP server handler, where you need to 
    //// wait for a response from the actor system before you can send back the response 
    //// to the client, it also works well when you are using any kind of function which 
    //// can map on that future without having to explicitly block on the response 
    //// which can be solved using `.await`.
    // 
    //
    //// sender param must be None inside the tell() method if we're sending message to the actor from the main()
    //// sender param must be the actor caller iteself if we're returning a future objectr as a response from the result of calling the ask() function 

    // ----------------------------------------------------------------------
    //                      BUILDING THE SECOND PARACHAIN
    // ----------------------------------------------------------------------
    
    info!("➔ 🔗 building second parachain");
    let parachain_1_props = Props::new_args::<actors::parathread::Parachain, _>( //// prop types are inside Arc and Mutex thus we can clone them and move them between threads
                                                                                                                            (Uuid::new_v4(), 
                                                                                                                            None, //// empty slot for now
                                                                                                                            None, 
                                                                                                                            None, 
                                                                                                                            None)
                                                                                                                        );
    let parachain_1 = coiniXerr_sys.actor_of_props::<actors::parathread::Parachain>("parachain_1", parachain_1_props.clone()).unwrap(); //-- initializing the second parachain actor with its props; ActorRef is of type ParachainMsg means that we can communicate with another actor or the actor itself by sending Validator iteself as a message - props are Clone and Send and we can share them between threads

    // ----------------------------------------------------------------------
    //                GETTING THE UUID OF THE SECOND PARACHAIN
    // ----------------------------------------------------------------------
    
    info!("➔ 🎫 getting uuid of the second parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the uuid event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let current_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_sys, &parachain_1, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetParachainUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in parachain actor as a future object
    let second_parachain_uuid = current_uuid_remote_handle.await;

    // ---------------------------------------------------------------------------------
    //         BROADCASTING SECOND PARACHAIN ACTOR TO OTHER PARACHAIN ACTORS
    // ---------------------------------------------------------------------------------

    parachain_created_channel.tell( //// telling the channel that we want to publish something
                                Publish{
                                    msg: ParachainCreated(second_parachain_uuid.clone()), //// publishing the ParachainCreated message event to the parachain_created_channel channel 
                                    topic: "<second parachain created>".into(), //// setting the topic to <second parachain created> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                }, 
                                None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None 
                            );

    // ----------------------------------------------------------------------
    //                     BUILDING THE DEFAULT PARACHAIN
    // ----------------------------------------------------------------------
    
    info!("➔ 🔗 starting default parachain");
    let mut chain = Some(Chain::default());
    let current_slot_for_default_parachain = Slot::default(); //// default slot on the first run of the coiniXerr network; this field will be updated every 5 seconds for default and second parachain 
    let parachain_0_props = Props::new_args::<actors::parathread::Parachain, _>( //// prop types are inside Arc and Mutex thus we can clone them and move them between threads
                                                                                                                            (Uuid::new_v4(), 
                                                                                                                            Some(current_slot_for_default_parachain),
                                                                                                                            chain, 
                                                                                                                            Some(parachain_1.clone()), //// the next parachain or the next blockchain actor
                                                                                                                            None)
                                                                                                                        );
    let parachain_0 = coiniXerr_sys.actor_of_props::<actors::parathread::Parachain>("parachain_0", parachain_0_props.clone()).unwrap(); //-- initializing the first parachain actor with its props; ActorRef is of type ParachainMsg means that we can communicate with another actor or the actor itself by sending Validator iteself as a message - props are Clone and Send and we can share them between threads

    // ----------------------------------------------------------------------
    //     GETTING THE CURRENT BLOCK OF THE DEFAULT PARACHAIN BLOCKCHAIN
    // ----------------------------------------------------------------------

    info!("➔ 🧊 getting current block from the default parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the current block event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let current_block_remote_handle: RemoteHandle<Block> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetCurrentBlock}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the current block of the passed in parachain actor as a future object
    let mut current_block = current_block_remote_handle.await;

    // ----------------------------------------------------------------------
    //            GETTING THE BLOCKCHAIN OF THE DEFAULT PARACHAIN
    // ----------------------------------------------------------------------

    info!("➔ 🔗🧊 getting blockchain from the default parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the current blockchain event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let blockchain_remote_handle: RemoteHandle<Chain> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetBlockchain}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the blockchain of the passed in parachain actor as a future object
    let blockchain = blockchain_remote_handle.await;

    // ----------------------------------------------------------------------
    //           GETTING THE CURRENT SLOT OF THE DEFAULT PARACHAIN
    // ----------------------------------------------------------------------

    info!("➔ 🎟️ getting current slot from the default parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the current slot event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let current_slot_remote_handle: RemoteHandle<Slot> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetSlot}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the current slot of the passed in parachain actor as a future object
    let mut current_slot = current_slot_remote_handle.await;

    // ----------------------------------------------------------------------
    //                  GETTING THE UUID OF THE PARACHAIN
    // ----------------------------------------------------------------------
    
    info!("➔ 🎫 getting uuid of the default parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the uuid event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let current_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetParachainUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in parachain actor as a future object
    let default_parachain_uuid = current_uuid_remote_handle.await;

    // ---------------------------------------------------------------------------------
    //         BROADCASTING DEFAULT PARACHAIN ACTOR TO OTHER PARACHAIN ACTORS
    // ---------------------------------------------------------------------------------

    parachain_created_channel.tell( //// telling the channel that we want to publish something
                                Publish{
                                    msg: ParachainCreated(default_parachain_uuid.clone()), //// publishing the ParachainCreated message event to the parachain_created_channel channel 
                                    topic: "<default parachain created>".into(), //// setting the topic to <default parachain created> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                }, 
                                None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                            );


















    







    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                           parachain subscribers 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    parachain_updated_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                Subscribe{ 
                                    actor: Box::new(parachain_1.clone()), //// parachain_1 wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                    topic: "<default parachain updated>".into() //// <default parachain updated> topic
                                },
                                None
    );

    parachain_updated_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                Subscribe{ 
                                    actor: Box::new(parachain_0.clone()), //// parachain_0 wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                    topic: "<second parachain updated>".into() //// <second parachain updated> topic
                                },
                                None
    );

    parachain_created_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                Subscribe{ 
                                    actor: Box::new(parachain_1.clone()), //// parachain_1 wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                    topic: "<default parachain created>".into() //// <default parachain created> topic
                                },
                                None
    );

    parachain_created_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                Subscribe{ 
                                    actor: Box::new(parachain_0.clone()), //// parachain_0 wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                    topic: "<second parachain created>".into() //// <second parachain created> topic
                                },
                                None
    );























    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                updating coiniXerr parachain networks' state 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    // ---------------------------------------------------------------------------------
    //         RESETTING THE NEXT PARACHAIN SLOT FIELD OF THE DEFAULT PARACHAIN
    // --------------------------------------------------------------------------------- 
    
    info!("➔ 🔁 resetting next parachain slot field of the default parachain");
    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like resetting the slot field of the next parachain cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let update_next_parachain_slot_remote_handle: RemoteHandle<Parachain> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::WaveSlotToNextParachainActor}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to wave to the next parachain of the passed in parachain actor and return the result or response as a future object
    let update_next_parachain_slot = update_next_parachain_slot_remote_handle.await; //// next parachain field of the default parachain is the second parachain that we've just built earlier 

    // ------------------------------------------------------------------------------------------
    //      SCHEDULING EVERY 5 SECONDS TO RESET THE SLOT IN THE DEFAULT AND SECOND PARACHAIN
    // ------------------------------------------------------------------------------------------

    let delay = Duration::from_secs(1); //// run for the first time after passing 1 second
    let interval = Duration::from_secs(5); //// run every 5 seconds
    coiniXerr_sys.schedule( //// scheduling a message
                            delay, //// after 1 second delay
                            interval, //// to be executed every 5 seconds 
                            parachain_1.clone(), //// on parachain_1 actor
                            None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                            ParachainCommunicate{ //// the message event is the WaveResetSlotFromSystem variant in which the slot field of the passed in parachain will be updated
                                id: Uuid::new_v4(),
                                cmd: ParachainCmd::WaveResetSlotFromSystem //// that default parachain wants to reset the slot
                            },
                        );
    coiniXerr_sys.schedule( //// scheduling a message
                            delay, //// after 1 second delay
                            interval, //// to be executed every 5 seconds 
                            parachain_0.clone(), //// on parachain_0 actor
                            None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                            ParachainCommunicate{ //// the message event is the WaveResetSlotFromSystem variant in which the slot field of the passed in parachain will be updated
                                id: Uuid::new_v4(),
                                cmd: ParachainCmd::WaveResetSlotFromSystem //// that default parachain wants to reset the slot
                            },
                        );
    
    // ----------------------------------------------------------------------------------------------------------------
    //       BROADCASTING THE UPDATING PARACHAIN MESSAGE TO THE RELATED CHANNEL SO ALL PARACHAIN ACTORS CAN SEE
    // ----------------------------------------------------------------------------------------------------------------

    info!("➔ 🔃 updating parachains' state since slot field has been rest");

    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the uuid event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let parachain_0_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_sys, &parachain_0, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetParachainUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in parachain actor as a future object
    let parachain_0_uuid = parachain_0_uuid_remote_handle.await;

    //// we have to ask the actor that hey we want to return some info as a future object about the parachain by sending the related message like getting the uuid event cause the parachain is guarded by the ActorRef
    //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
    let parachain_1_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_sys, &parachain_1, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetParachainUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in parachain actor as a future object
    let parachain_1_uuid = parachain_1_uuid_remote_handle.await;

    parachain_updated_channel.tell( //// telling the channel that we want to publish something
                                Publish{
                                    msg: ParachainUpdated(parachain_0_uuid.clone()), //// publishing the ParachainUpdated message event to the parachain_updated_channel channel 
                                    topic: "<default parachain updated>".into(), //// setting the topic to <default parachain updated> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                }, 
                                None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
    );
    
    parachain_updated_channel.tell( //// telling the channel that we want to publish something
                                Publish{
                                    msg: ParachainUpdated(parachain_1_uuid.clone()), //// publishing the ParachainUpdated message event to the parachain_updated_channel channel 
                                    topic: "<second parachain updated>".into(), //// setting the topic to <second parachain updated> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                }, 
                                None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
    );

    // ---------------------------------------------------------------------------------
    //        SENDING RESET MESSAGE FROM DEFAULT PARACHAIN TO SECOND PARACHAIN
    // --------------------------------------------------------------------------------- 
    
    //// calling between actors using send_msg() method
    parachain_0.clone().send_msg(actors::parathread::ParachainMsg::Communicate( //// sending message from parachain_0 to parachain_1
        ParachainCommunicate{
            id: Uuid::new_v4(),
            cmd: ParachainCmd::WaveResetSlotFrom(default_parachain_uuid.to_string()) //// that default parachain wants to reset the slot  
        }),
        parachain_1.clone()); //// of the parachain_1 (second parachain)
    
    //// calling between actors using tell() method which is inside the main() and select() method which is 
    ///// inside WaveSlotToParachainActor variant to wave reset slot to second parachain (parachain_1).
    parachain_0.tell( //// we're telling the default parachain from the main()
                    ParachainCommunicate{
                        id: Uuid::new_v4(),
                        cmd: ParachainCmd::WaveSlotToParachainActor("/user/select-actor/parachain_1".to_string()) //// to tell the parachain_1 (second parachain) that default parachain wants to reset your slot  
                    },
                    None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                );
    

















    
    
    






    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                     starting validator actors for incoming transactions' bytes through a tcp streamer 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    while let Ok((stream, addr)) = listener.accept().await{ //-- await suspends the accept() function execution to solve the future but allows other code blocks to run  
        info!("➔ 🪢 connection stablished from {}", addr);
        let cloned_arc_mutex_runtime_info_object = Arc::clone(&arc_mutex_runtime_info_object); //-- cloning (making a deep copy of) runtime info object to prevent ownership moving in every iteration between threads
        let stream_sender = stream_sender.clone(); //-- we're using mpsc channel to send data between tokio tasks and each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning (making a deep copy of) the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the mutex lock
        
        // ----------------------------------------------------------------------
        //               BUILDING VALIDATOR ACTOR FOR THIS STREAM
        // ----------------------------------------------------------------------
        
        let default_parachain_slot = current_slot.clone();
        let stream_validator = default_parachain_slot.clone().get_validator(addr.clone()); // TODO - validator uniqueness
        
        //// means we don't find any validator inside the default parachain slot  
        if let None = stream_validator{ 
            current_slot = default_parachain_slot
                                                .clone()
                                                .add_validator( //// this method will return the updated slot
                                                    default_parachain_uuid, 
                                                    addr.clone() //// socket address can be a unique identifier for the connected validator since it has a unique port each time that a validator gets slided into the network 
                                                );
        }
        
        let validator = Validator{ //// we have to clone the stream_validator in each arm to prevent ownership moving since we're lossing the ownership in each arm
            id: match stream_validator.clone(){
                Some(v) => v.id,
                None => Uuid::new_v4(),
            },
            addr: match stream_validator.clone(){
                Some(v) => v.addr,
                None => addr.clone(),
            },
            recent_transaction: match stream_validator.clone(){
                Some(v) => v.recent_transaction,
                None => None,
            },
            mode: match stream_validator.clone(){
                Some(v) => v.mode,
                None => ValidatorMode::Mine,
            },
            ttype_request: match stream_validator.clone(){
                Some(v) => v.ttype_request,
                None => None,
            }
        };

        info!("➔ 👷🏼‍♂️ building validator actor for this peer");
        let validator_props = Props::new_args::<Validator, _>( //// prop types are inside Arc and Mutex thus we can clone them and move them between threads  
                                                                                                            (
                                                                                                                validator.id.clone(), 
                                                                                                                validator.addr, 
                                                                                                                validator.recent_transaction, 
                                                                                                                validator.mode, 
                                                                                                                validator.ttype_request
                                                                                                            )
                                                                                                        );
        let validator_actor = coiniXerr_sys.clone().actor_of_props::<Validator>("validator", validator_props.clone()).unwrap(); //-- initializing the validator actor with its props; ActorRef is of type ValidatorMsg means that we can communicate with another actor or the actor itself by sending Validator iteself as a message - props are Clone and Send and we can share them between threads
        let validator_actor = validator_actor.clone(); //-- cloning (making a deep copy of) the validator actor will prevent the object from moving in every iteration - trait Clone is implemented for Validator actor struct since the type is Send + Sync across threads
        let validator_updated_channel = validator_updated_channel.clone();  //-- cloning (making a deep copy of) the channel actor will prevent the object from moving in every iteration - trait Clone is implemented for channel actor struct since the type is Send + Sync across threads
        
        // ---------------------------------------------------------------------------------
        //              BROADCASTING NEW VALIDATOR TO OTHER VALIDATOR ACTORS
        // ---------------------------------------------------------------------------------

        validator_joined_channel.tell( //// telling the channel that we want to publish something
                                    Publish{
                                        msg: ValidatorJoined(validator.id.clone()), //// publishing the ValidatorJoined message event to the validator_joined_channel channel 
                                        topic: "<new validator joined>".into(), //// setting the topic to <new validator joined> so all subscribers of this channel (all validator actors) can subscribe and react to this topic of this message event
                                    }, 
                                    None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                                );
        
        // ---------------------------------------------------------------------------------
        //                 CREATED VALIDATOR SUBSCRIBES TO NEW VALIDATOR TOPIC
        // ---------------------------------------------------------------------------------

        validator_joined_channel.tell( //// telling the channel that an actor wants to subscribe to a topic - whenever a validator join current validator can subscribe to the related topic
                                    Subscribe{ 
                                        actor: Box::new(validator_actor.clone()), //// validator_actor wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                        topic: "<new validator joined>".into() //// <new validator joined> topic
                                    },
                                    None
        );

        // ----------------------------------------------------------------------
        //                  SAVING RUNTIME INFO FOR THIS STREAM
        // ----------------------------------------------------------------------
        info!("➔ 💾 saving runtime info");
        let meta_data_uuid = {
            let listener_address = format!("{:p}", &listener);
            let mut runtime_info = cloned_arc_mutex_runtime_info_object.lock().unwrap().to_owned(); //-- in order to use the to_owned() method we have to implement the Clone trait for the Runtime struct since this method will make a clone from the instance - unlocking, unwrapping and cloning (by using to_ownded() method) the runtim_info object in every iteration of incoming stream inside the local thread to convert it to an instance of the RafaelRt struct
            RafaelRt::add( //-- locking on runtime info object (mutex) must be done in order to prevent other threads from mutating it at the same time 
                runtime_info, //-- passing the mutable runtime_info object for adding new metadata into its hash map field
                MetaData{
                    id: Uuid::new_v4(),
                    node_addr: Some(addr), //-- the ip address of the validator node 
                    actor: validator_actor.clone(), //-- cloning (making a deep copy of) the validator actor will prevent the object from moving in every iteration
                    link_to_server: Some(LinkToService(listener_address)), //-- this is the location address of the tcp listener inside the ram 
                    last_crash: None,
                    first_init: Some(chrono::Local::now().naive_local().timestamp()),
                    error: None,
                }
            )
        };
        
        // ----------------------------------------------------------------------
        //                    LOGGING RAFAEL RUNTIME INSTANCE
        // ----------------------------------------------------------------------

        let rafael_event_log = EventLog{
            time: Some(chrono::Local::now().timestamp_nanos()),
            event: EventVariant::Runime(vec![
                RuntimeLog{
                    id: Uuid::new_v4().to_string(),
                    path: "/var/log/coiniXerr/runtime/rafael.log".to_string(), // TODO - save the log in /var/log/coiniXerr/runtime/
                    requested_at: Some(chrono::Local::now().timestamp_nanos()),
                    content: Box::new([]), // TODO - log content 
 
                }
            ])
        };
        info!("➔ 🎞️ rafael runtime instance log {}", rafael_event_log); //-- it'll log to the console like RAFAEL_EVENT_JSON:{"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]}

        // --------------------------------------------------------------------------------------------------------------------------------------------
        //                 SENDING THE STREAM, RUNTIME, VALIDATOR, VALIDATOR UPDATE CHANNEL AND ACTOR SYSTEM TO DOWN SIDE OF THE CHANNEL 
        // --------------------------------------------------------------------------------------------------------------------------------------------

        let arc_mutex_validator_actor = Arc::new(Mutex::new(validator_actor)); //-- creating an Arc object which is inside a Mutex to share and mutate data between threads cause Validator actor addr object doesn't implement Clone trait and the object inside Arc is not mutable thus we have to put the validator_actor object inside a mutex to be updatable between threads
        let cloned_arc_mutex_validator_actor = Arc::clone(&arc_mutex_validator_actor); //-- we're borrowing the ownership of the Arc-ed and Mutex-ed validator_actor object to move it between threads without loosing the ownership 
        
        //// putting the validator_updated_channel inside the Arc<Mutex<...>> to send it through the stream mpsc channel
        let arc_mutex_validator_update_channel = Arc::new(Mutex::new(validator_updated_channel));
        let cloned_arc_mutex_validator_update_channel = Arc::clone(&arc_mutex_validator_update_channel);   

        info!("➔ 📼 sending stream setups through the channel");
        stream_sender.send((stream, 
                            meta_data_uuid, 
                            cloned_arc_mutex_runtime_info_object, 
                            cloned_arc_mutex_validator_actor, 
                            cloned_arc_mutex_validator_update_channel, 
                            coiniXerr_sys.clone()
                        )).await.unwrap(); //-- sending the stream, the cloned runtime info and metadata uuid, cloned validator, coiniXerr actor system and the validator update channel through the mpsc channel 
            
    }








 














    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                                 waiting to receive stream and other setups asynchronously 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    while let Some((
                    mut stream, 
                    generated_uuid, 
                    cloned_arc_mutex_runtime_info_object, 
                    cloned_arc_mutex_validator_actor, 
                    cloned_arc_mutex_validator_update_channel, 
                    coiniXerr_actor_system
                )) = stream_receiver.recv().await.take(){ //-- waiting for the stream and other setups to become available to the down side of channel (receiver) to change the started validator actor's transaction for every incoming connection - stream must be mutable for reading and writing from and to socket
        
        info!("➔ 📥 receiving the stream setups");
        let mempool_sender = mempool_sender.clone(); //-- cloning mempool_sender to send signed transaction through the channel to the receiver for mining process
        
        //// ... the following will handle the the incoming stream inside a tokio worker green threadpool measn tokio::spawn() is an async task worker green threadpool solver 
        //// ... move will move everything from its behind to the new scope and take their ownership so there is not a single var after moving in second iteration of the loop thus we've passed all the requirements that might be moved by doing that we can make sure that we have them again after first iteration 
        //// ... beacuse in the first iteration async move{} will move the everything and takes the ownership from its behind thus in second iteration we don't have them and to solve this issue we have to pass them in the channel to have them in every iteration  
        tokio::spawn(async move { //-- this is an async task related to updating a validator actor on every incoming message from the sender which is going to be solved in the background on a single (without having to work on them in parallel) thread using green threadpool of tokio runtime and message passing channels like mpsc job queue channel protocol
            let mut transaction_buffer_bytes = vec![0 as u8; buffer_size]; //-- using [0 as u8; buffer_size] gives us the error of `attempt to use a non-constant value in a constant` cause [u8] array doesn't implement the Sized trait
            while match stream.read(&mut transaction_buffer_bytes).await{ //-- streaming over the incoming bytes from the socket - reading is the input and writing is the output
                Ok(size) if size == 0 => false, //-- socket closed since zero bytes data are here!
                Ok(size) => {
                    
                    // ----------------------------------------------------------------------
                    //                                 SIMD OPS
                    // ----------------------------------------------------------------------
                    
                    let heavy_func = |chunk: u8| {
                        let byte_name = Ok::<&[u8], String>(b"wildonion");
                        info!("➔ \t--------Doing some heavy operation on chunk [{:?}]", chunk);
                        chunk
                    };
                    let bytes_slice = utils::into_box_slice(&transaction_buffer_bytes).await.unwrap(); //-- converting transaction_buffer_bytes into a Box of 4 u8 slice
                    let start = Instant::now();
                    match utils::simd(u32::from_be_bytes(*bytes_slice), heavy_func).await{ //-- passing a u32 bits integer by dereferencing the Box which has the bytes_slice value itself - from_be_bytes() method creates a native endian integer value from its representation as a byte array in big endian
                        Ok(result) => {
                            let end = Instant::now();
                            let delta = end.duration_since(start);
                            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
                            // assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
                            info!("➔ ::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
                            let msg_to_write = format!("::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
                            stream.write(&msg_to_write.as_bytes()).await.unwrap(); //-- sending the simd result String message as utf8 bytes back to the peer
                        },
                        Err(e) => {
                            info!("➔ ::::: error in reading chunk caused by {:?}", e);
                            let msg_to_write = format!("::::: error in reading chunk caused by {:?}", e);
                            stream.write(&msg_to_write.as_bytes()).await.unwrap(); //-- sending the simd error String message as utf8 bytes back to the peer
                        },
                    };
                    
                    // ---------------------------------------------------------------------------------------
                    //          SERDING INCOMING IO STREAM OF TRANSACTION CHUNKS USING serde & borsh
                    // ---------------------------------------------------------------------------------------
                    // NOTE - ..size means from the beginning to the limit - 1, we could also use 0..size

                    let deserialized_transaction_union = Transaction::new(&transaction_buffer_bytes[..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes into the Transaction struct object using TransactionMem union
                    let deserialized_transaction_serde = &mut serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[..size]).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a mutable Transaction object using serde_json::from_slice to mutate the signed field 
                    let deserialized_transaction_borsh = &mut Transaction::try_from_slice(&transaction_buffer_bytes[..size]).unwrap(); //-- passing the vector of utf8 bytes into the try_from_slice() method to deserialize into the SMSResponse struct - since Vec<u8> will be coerced to &'a [u8] at compile time we've passed Vec<u8> type into the try_from_slice() method; since we want to sign the transaction thus we must define it as mutable
                    let mut transaction_serialized_into_vec_bytes_using_serede = serde_json::to_vec(&deserialized_transaction_serde).unwrap(); //-- converting the deserialized_transaction_serde object into vector of utf8 bytes using serde
                    let mut transaction_serialized_into_vec_bytes_using_borsh = deserialized_transaction_borsh.try_to_vec().unwrap(); //-- converting the transaction object into vector of utf8 bytes using borsh
                    // TODO - only if the downside of the mpsc job queue channel was available the transaction will be signed and sent through the mempool channel to be pushed inside a block for mining process
                    // TODO - make sure that the receiver is not dropped
                    // ...
                    let now = chrono::Local::now().naive_local().timestamp();
                    let must_be_signed = true;
                    if must_be_signed && deserialized_transaction_borsh.issued < now{ //// the downside of the mempool channel must be available and also the issued time of the transaction must be smaller than the current server time
                        
                        // ----------------------------------------------------------------------
                        //              SIGNING THE INCOMING TRANSACTION WITH SERVER TIME
                        // ----------------------------------------------------------------------
                        
                        info!("➔ ✍️ signing incoming transaction");
                        deserialized_transaction_borsh.signed = Some(chrono::Local::now().naive_local().timestamp()); //-- signing the incoming transaction with the current server time
                        
                        // ----------------------------------------------------------------------
                        //        ENCODING SIGNED TRANSACTION THEN SENDING BACK TO THE PEER
                        // ---------------------------------------------------------------------- 

                        let mut signed_transaction_serialized_into_vec_bytes_using_borsh = deserialized_transaction_borsh.try_to_vec().unwrap(); //-- converting the signed transaction object into vector of utf8 bytes using borsh
                        
                        // --------------------------------------------------------------------
                        //                      CONVERTING Vec<u8> -> &[u8]
                        // --------------------------------------------------------------------
                        
                        let mut utf8_bytes_using_as_mut_slice = signed_transaction_serialized_into_vec_bytes_using_borsh.as_mut_slice(); //-- converting Vec<u8> to mutable slice of &[u8] using as_mut_slice() method - remeber that signed_transaction_serialized_into_vec_bytes_using_borsh must be defined as mutable
                        let utf8_bytes_using_casting: &[u8] = &signed_transaction_serialized_into_vec_bytes_using_borsh; //-- since the Vec<u8> will be coerced to &'a [u8] with a valid lifetime at compile time we can borrow the ownership of sms_response_serialized_into_vec_bytes_using_serede using & which by doing this we're borrowing a slice of Ve<u8> from the heap memory which will be coerced to &'a [u8] since we've specified the type of sms_response_serialized_into_utf8_bytes_using_serede which is &[u8]
                        let boxed_utf8_bytes_using_box_slcie = signed_transaction_serialized_into_vec_bytes_using_borsh.into_boxed_slice(); //-- converting the Vec<u8> to Box<u8> using into_boxed_slice() method 
                        let utf_bytes_dereference_from_box = &*boxed_utf8_bytes_using_box_slcie; //-- borrow the ownership of the dereferenced boxed_utf8_bytes_using_box_slcie using & to convert it to &[u8] with a valid lifetime since the dereferenced boxed_utf8_bytes_using_box_slcie has unknown size at compile time thus working with u8 slice needs to borrow them from the heap memory to have their location address due to implemented ?Sized for [u8]
                        info!("➔ 🪙✍️ sending signed transaction back to the peer");
                        stream.write(&utf_bytes_dereference_from_box).await.unwrap(); //-- sending the signed transaction back to the peer - since Vec<u8> will be coerced to &'a [u8] with valid lifetime at compile time we can also send the signed_transaction_serialized_into_vec_bytes_using_borsh directly through the socket even though the write() method takes &'a [u8] param with a valid lifetime 
                        
                        // ----------------------------------------------------------------------
                        //       UPDATING VALIDATOR ACTOR WITH THE LATEST SIGNED TRANSACTION
                        // ----------------------------------------------------------------------

                        info!("➔ 👷🏼‍♂️🔃 updating validator actor with the recent signed transaction");
                        for (id, md) in cloned_arc_mutex_runtime_info_object.lock().unwrap().0.iter_mut(){ //-- id and md are &mut Uuid and &mut MetaData respectively - we have to iterate over our info_dict mutably and borrowing the key and value in order to update the validator actor transaction of our matched meta_data id with the incoming uuid
                            if id == &generated_uuid{
                                let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&utf_bytes_dereference_from_box).unwrap(); //-- deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                                md.update_validator_transaction(Some(signed_transaction_deserialized_from_bytes)); //-- update the validator actor with a recent signed transaction
                            }
                        }

                        // ---------------------------------------------------------------------------------
                        //              BROADCASTING VALIDATOR UPDATE TO OTHER VALIDATOR ACTORS
                        // ---------------------------------------------------------------------------------

                        let validator_update_channel = cloned_arc_mutex_validator_update_channel.lock().unwrap().clone(); //// cloning will return the T from MutexGuard
                        let current_validator = cloned_arc_mutex_validator_actor.lock().unwrap().clone(); //// cloning will return the T from MutexGuard
                        let current_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_actor_system, &current_validator, ValidatorCommunicate{id: Uuid::new_v4(), cmd: ValidatorCmd::GetValidatorUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in validator actor and return the result or response as a future object
                        let current_validator_uuid = current_uuid_remote_handle.await; //// getting the uuid of the current validator which has passed in to the stream mpsc channel
                        validator_update_channel.tell( //// telling the channel that we want to publish something
                                                    Publish{
                                                        msg: ValidatorUpdated(current_validator_uuid.clone()), //// publishing the ValidatorUpdated message event to the validator_updated_channel channel 
                                                        topic: "<validator state updated with recent transaction>".into(), //// setting the topic to <validator state updated> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                                    }, 
                                                    None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                                                );
                        
                        // ---------------------------------------------------------------------------------
                        //               CURRENT VALIDATOR SUBSCRIBES TO VALIDATOR UPDATE TOPIC
                        // ---------------------------------------------------------------------------------

                        validator_update_channel.tell( //// telling the channel that an actor wants to subscribe to a topic - whenever a validator status update current validator can subscribe to the related topic
                                                    Subscribe{ 
                                                        actor: Box::new(current_validator.clone()), //// current_validator wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                                        topic: "<validator state updated with recent transaction>".into() //// <validator state updated with recent transaction> topic
                                                    },
                                                    None
                        );

                        // ---------------------------------------------------------------------------------------
                        //      SENDING SIGNED TRANSACTION TO DOWN SIDE OF THE CHANNEL FOR CONSENSUS PROCESS
                        // ---------------------------------------------------------------------------------------
                        
                        info!("➔ 📼✍️ sending signed transaction to down side of the channel for consensus process");
                        let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&utf_bytes_dereference_from_box).unwrap(); //-- deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                        let arc_mutex_transaction = Arc::new(Mutex::new(signed_transaction_deserialized_from_bytes)); //-- putting the signed_transaction_deserialized_from_bytes inside a Mutex to borrow it as mutable inside Arc by locking the current thread 
                        let cloned_arc_mutex_transaction = Arc::clone(&arc_mutex_transaction); //-- cloning the arc_mutex_transaction to send it through the mpsc job queue channel 
                        mempool_sender.send((cloned_arc_mutex_transaction, cloned_arc_mutex_validator_actor.clone(), coiniXerr_actor_system.clone())).await.unwrap(); //-- sending signed transaction plus the validator actor info through the mpsc job queue channel asynchronously for mining process - we must clone the cloned_arc_mutex_validator_actor in each iteration to prevent ownership moving
                        true
                    } else{
                        
                        // ----------------------------------------------------------------------
                        //       REJECTING THE INCOMING TRANSACTION BACK TO THE VALIDATOR
                        // ----------------------------------------------------------------------
                        
                        info!("➔ 🙅 rejecting incoming transaction caused by Unavailable Mempool Channel issue");
                        stream.write(&transaction_buffer_bytes[..size]).await.unwrap(); //-- rejecting the transaction back to the peer
                        true
                    }
                },
                Err(e) => {
                    info!("➔ 🔚 terminating connection with validator {}", stream.peer_addr().unwrap());
                    stream.shutdown().await.unwrap(); //-- shuts down the output stream
                    false
                }
            } {} //-- the while match must be a block which will return true on its Ok() arm and false on its Err arm
        }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
    }


















    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////   waiting to receive signed transactions asynchronously from the sender to push them inside the current block - this buffer zone is the transaction mempool channel
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

    while let Some((transaction, validator, coiniXerr_actor_system)) = mempool_receiver.recv().await{ //-- waiting for each transaction to become available to the down side of channel (receiver) for mining process cause sending is done asynchronously 
        info!("➔ 📥 receiving new transaction and its related validator to push inside the current block");
        let mutex_transaction = transaction.lock().unwrap().clone();
        info!("➔ 🪙 new transaction {:?} in mempool", mutex_transaction);
        let mutex_validator_actor = validator.lock().unwrap().clone();

        let current_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_actor_system, &mutex_validator_actor, ValidatorCommunicate{id: Uuid::new_v4(), cmd: ValidatorCmd::GetValidatorUuid}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in validator actor and return the result or response as a future object
        let current_validator_uuid = current_uuid_remote_handle.await; //// getting the uuid of the current validator which has passed in to the stream mpsc channel
        info!("➔ 👷🏼‍♂️ validator actor with id [{}] and info {:?} in mempool", current_validator_uuid, mutex_validator_actor);
        
        // ----------------------------------------------------------------------
        //            COMMUNICATE WITH THE VALIDATOR BASED ON TX TYPE
        // ----------------------------------------------------------------------

        //// since we're not sending following messages from another actor actually we're sending from the main() and main() is not an actor thus the sender in tell() method must be None
        if mutex_transaction.ttype == 0x00{ //-- regular transaction
            ///// tell the validator actor from the main() that we have the message of type Contract with the 0x00 ttype
            mutex_validator_actor.tell(Contract{id: Uuid::new_v4(), ttype: 0x00}, None); //// 0x00 means regular transaction like transferring tokens
        } else if mutex_transaction.ttype == 0xFF{ //-- CRC21 smart contract transaction
            ///// tell the validator actor from the main() that we have the message of type Contract with the 0xFF ttype 
            mutex_validator_actor.tell(Contract{id: Uuid::new_v4(), ttype: 0xFF}, None); //// 0xFF means CRC21 transaction like minting NFT 
        } else if mutex_transaction.ttype == 0x02{ //-- CRC20 smart contract transaction
            ///// tell the validator actor from the main() that we have the message of type Contract with the 0x02 ttype 
            mutex_validator_actor.tell(Contract{id: Uuid::new_v4(), ttype: 0x02}, None); //// 0x02 means CRC20 transaction like minting FT
        } else if mutex_transaction.ttype == 0x03{ //-- CRC22 smart contract transaction
            ///// tell the validator actor from the main() that we have the message of type Contract with the 0x02 ttype 
            mutex_validator_actor.tell(Contract{id: Uuid::new_v4(), ttype: 0x03}, None); //// 0x03 means CRC22 transaction which supports FN and NFT methods
        }
        
        // ------------------------------------------------------------------------------------------
        //      BROADCASTING NEW INCOMING TRANSACTION INTO THE MEMPOOL TO OTHER VALIDATOR ACTORS
        // ------------------------------------------------------------------------------------------

        mempool_updated_channel.tell( //// telling the channel that we want to publish something
                                    Publish{
                                        msg: UpdateValidatorAboutMempoolTx(mutex_transaction.id.clone()), //// publishing the UpdateValidatorAboutMempoolTx message event to the mempool_updated_channel channel 
                                        topic: "<new transaction slided into the mempool>".into(), //// setting the topic to <new transaction slided into the mempool> so all subscribers of this channel (all validator actors) can subscribe and react to this topic of this message event
                                    }, 
                                    None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                                );
        
        
        // ---------------------------------------------------------------------------------
        //              CURRENT VALIDATOR SUBSCRIBES TO NEW BLOCK MINED TOPIC
        // ---------------------------------------------------------------------------------

        mempool_updated_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                    Subscribe{ 
                                        actor: Box::new(mutex_validator_actor.clone()), //// mutex_validator_actor wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                        topic: "<new transaction slided into the mempool>".into() //// <new transaction slided into the mempool> topic
                                    },
                                    None
        );
        
        // ----------------------------------------------------------------------
        //                  CONSENSUS AND BUILDING BLOCKS PROCESS
        // ----------------------------------------------------------------------

        while std::mem::size_of_val(&current_block) <= max_block_size{ //-- returns the dynamically-known size of the pointed-to value in bytes by passing a reference or pointer to the value to this method - push incoming transaction into the current_block until the current block size is smaller than the max_block_size
            current_block.push_transaction(mutex_transaction.clone()); //-- cloning transaction object in every iteration to prevent ownership moving and loosing ownership - adding pending transaction from the mempool channel into the current block for validating that block
            if std::mem::size_of_val(&current_block) > max_block_size{
                // TODO - calculate the block and merkle_root hash
                // TODO - consensus and block validation process here
                // ...
                info!("➔ ⚒️🧊 shaping a new block to add transactions");
                let (prev, last) = {
                    let current_blockchain = blockchain.clone(); //-- creating longer lifetime since `let` will create a longer lifetime for the value - can't have blockchain.clone().blocks.iter().rev() cause blockchain.clone() lifetime will be ended beforer reach the blocks field
                    let mut rev_iter = current_blockchain.blocks.iter().rev(); //-- cloning (making a deep copy of) the blockchain of the parachain actor will prevent the object from moving and loosing ownership - we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    (rev_iter.next().unwrap().to_owned(), rev_iter.next().unwrap().to_owned()) //-- converting &Block to Block by using to_owned() method in which cloning process will be used 
                };
                current_block = blockchain.clone().build_raw_block(&prev); //-- passing the previous block by borrowing it - cloning (making a deep copy of) the blockchain of the parachain actor will prevent the object from moving and loosing ownership; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
            }
        }
        if let (Some(merkle_root), Some(block_hash)) = (current_block.clone().merkle_root, current_block.clone().hash){ //-- checking the block's hash and merkle_root hash for transactions finality
            info!("➔ 🥑 block with id [{}] is valid", current_block.id);
            current_block.is_valid = true;
            info!("➔ 🧣 adding the created block to the chain");
            blockchain.clone().add(current_block.clone()); //-- adding the cloned of current block to the coiniXerr parachain blockchain - cloning must be done to prevent current_block and the blockchain parachain from moving in every iteration mempool_receiver loop; we can also use as_ref() method instead of clone() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
        } else{
            info!("➔ ⛔ block with id [{}] is invalid", current_block.id);
            current_block.is_valid = false;
        }

        // ---------------------------------------------------------------------
        //              BROADCASTING MINING PROCESS TO ALL ACTORS
        // ---------------------------------------------------------------------

        mining_channel.tell( //// telling the channel that we want to publish something
                            Publish{
                                msg: UpdateValidatorAboutMiningProcess(current_block.id.clone()), //// publishing the UpdateValidatorAboutMiningProcess message event to the mining_channel channel 
                                topic: "<new block has mined>".into(), //// setting the topic to <new block has mined> so all subscribers of this channel (all validator actors) can subscribe and react to this topic of this message event
                            }, 
                            None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                        );
        
        // ---------------------------------------------------------------------------------
        //              CURRENT VALIDATOR SUBSCRIBES TO NEW BLOCK MINED TOPIC
        // ---------------------------------------------------------------------------------

        mining_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                            Subscribe{ 
                                actor: Box::new(mutex_validator_actor.clone()), //// mutex_validator_actor wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                topic: "<new block has mined>".into() //// <new block has mined> topic
                            },
                            None
        );

        // ------------------------------------------------------------------------
        //          UPDATING PARACHAIN ACTOR STATE AT THE END OF THE LOOP
        // ------------------------------------------------------------------------

        info!("➔ 🔃 updating default parachain state");
        //// we have to ask the actor that hey we want to update some info about the parachain by sending the related message cause the parachain is guarded by the ActorRef
        //// ask returns a future object which can be solved using block_on() method or by awaiting on it 
        let update_parachain_remote_handle: RemoteHandle<Parachain> = ask(&coiniXerr_actor_system, &parachain_0, UpdateParachainEvent{slot: Some(current_slot.clone()), blockchain: Some(blockchain.clone()), current_block: Some(current_block.clone())}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to update the state of the passed in parachain actor and return the result or response as a future object
        let update_default_parachain = update_parachain_remote_handle.await;

        // --------------------------------------------------------------------------------
        //         BROADCASTING DEFAULT PARACHAIN UPDATE TO OTHER PARACHAIN ACTORS
        // --------------------------------------------------------------------------------

        parachain_updated_channel.tell( //// telling the channel that we want to publish something
                                    Publish{
                                        msg: ParachainUpdated(update_default_parachain.id.clone()), //// publishing the ParachainUpdated message event to the parachain_updated_channel channel 
                                        topic: "<default parachain updated>".into(), //// setting the topic to <default parachain updated> so all subscribers of this channel (all parachain actors) can subscribe and react to this topic of this message event
                                    }, 
                                    None, //// since we're not sending this message from another actor actually we're sending from the main() (main() is the sender) and main() is not an actor thus the sender param must be None
                                );
        
        // ---------------------------------------------------------------------------------
        //           SECOND PARACHAIN SUBSCRIBES TO UPDATE DEFAULT PARACHAIN TOPIC
        // ---------------------------------------------------------------------------------

        parachain_updated_channel.tell( //// telling the channel that an actor wants to subscribe to a topic
                                    Subscribe{ 
                                        actor: Box::new(parachain_1.clone()), //// parachain_1 wants to subscribe to - since in subscribing a message the subscriber or the actor must be bounded to Send trait thus we must either take a reference to it like &dyn Tell<Msg> + Send or put it inside the Box like Box<dyn Tell<Msg> + Send> to avoid using lifetime directly since the Box is a smart pointer and has its own lifetime     
                                        topic: "<default parachain updated>".into() //// <default parachain updated> topic
                                    },
                                    None
        );

        // ----------------------------------------------------------------------
        //                 INSERTING THE PARACHAIN INTO THE DB
        // ----------------------------------------------------------------------
        
        let parachains = db_instance.clone().database(&db_name).collection::<schemas::InsertParachainInfo>("parachains");
        let parachain_info = schemas::InsertParachainInfo{
            //// we're cloning each field since we're inside the loop and we want to prevent ownership moving
            id: Uuid::new_v4(),
            slot: Some(current_slot.clone()),
            blockchain: Some(blockchain.clone()),
            next_parachain_id: Some(default_parachain_uuid.clone()), //// this is the uuid of the next parachain which is linked to this parachain since connected parachains form a circular pattern
            current_block: Some(current_block.clone()),
        };
        match parachains.insert_one(parachain_info, None).await{ //-- serializing the user doc which is of type RegisterRequest into the BSON to insert into the mongodb
            Ok(insert_result) => info!("➔ 🛢️🧣 inserted new parachain into db with uuid [{}] and mongodb id [{}]", default_parachain_uuid.clone(), insert_result.inserted_id.as_object_id().unwrap()),
            Err(e) => error!("😕 error inserting parachain with id [{}]: {}", default_parachain_uuid, e)
        }


    }





    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");



    
    Ok(()) //// everything went well






}
