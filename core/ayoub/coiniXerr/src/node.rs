



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

        https://github.com/wildonion/uniXerr/wiki/coiniXerr#coinixerr-node-design-pattern
      




*/









// #![allow(unused)] //-- will let the unused vars be there - we have to put this on top of everything to affect the whole crate
// #![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate

use async_trait::async_trait;
use lazy_static::lazy_static;
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
use crate::constants::*;
use crate::utils::DbORM::StorageModel;
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





#[path="tlps/tcp.server.rs"]
pub mod tcp;
#[path="tlps/rpc.server.rs"]
pub mod rpc;
pub mod constants;
pub mod schemas;
pub mod actors;
pub mod engine;
pub mod utils; //// we're importing the utils.rs in here as a public module thus we can access all the modules, functions and macros inside of it publicly















#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits; Box is an smart pointer which has valid lifetime for what's inside of it, we're putting the error part of the Result inside the Box since we have no idea about the size of the error or the type that caused this error happened at compile time thus we have to take a reference to it but without defining a specific lifetime
    

    



    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                  getting env vars
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    let env_vars = daemon::get_env_vars().await;







    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////             logging setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();






    

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           app storage setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    let app_storage: Option<Arc<Storage>> = db!{ //// this publicly has exported from the utils in here so we can access it here; db macro must be inside an async block or method since there is some async method in it
        env_vars.get("DB_NAME").unwrap().to_string(),
        env_vars.get("DB_ENGINE").unwrap().to_string(),
        env_vars.get("DB_HOST").unwrap().to_string(),
        env_vars.get("DB_PORT").unwrap().to_string(),
        env_vars.get("DB_USERNAME").unwrap().to_string(),
        env_vars.get("DB_PASSWORD").unwrap().to_string()
    };





    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                       bootstrapping TLPS
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// env_vars and app_storage don't implement Copy trait thus 
    //// we have to clone them to prevent ownership moving.
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr RPC SERVER
    // ----------------------------------------------------------------------
    //// used to send transaction from the walleXerr
    
    rpc::bootstrap(app_storage.clone(), env_vars.clone()).await; //// capn'p proto rpc
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr TCP SERVER
    // ----------------------------------------------------------------------
    //// used to send transaction from a TCP client 

    tcp::bootstrap(app_storage.clone(), env_vars.clone()).await; //// tokio tcp 
    




    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           graceful shutdown
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");








    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////             w're fine!
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    Ok(()) //// everything went well






}
