



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

//// sync creates are types that are thread safe and can be shared between threads safety
//// since types can be shareable if they are bounded to Send Sync and have valid lifetimes
//// also can be mutated by blocking the thread that wants to mutate it. 
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::mem;
use is_type::Is;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use log::{info, error, LevelFilter};
use tokio::net::{TcpListener, TcpStream, UdpSocket}; //-- async tcp listener and stream
///// read from the input and write to the output - AsyncReadExt and AsyncWriteExt 
//// are traits which are implemented for an object of type TcpStream and based 
//// on orphan rule we must use them here to use the read() and write() method 
//// asyncly which has been implemented for the object of TcpStream (these trait 
//// have been implemented for TcpStream structure) also The BufReader struct adds buffering to any reader.
//
//// based on orphan rule AsyncBufReadExt trait is rquired 
//// to be imported to call the lines() method 
//// on Lines<BufReader<Stdin>> structure.
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}; 
use tokio::sync::{mpsc, broadcast}; //// to broadcast transactions to from multiple senders to multiple receivers
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::{fmt, fmt::Write, num::ParseIntError};
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
//// loading all the required network stacks
//// to build a p2p blockchain node.
use libp2p::{
    gossipsub,
    futures::StreamExt, //// trait for streams
    core::upgrade,
    identity, identity::Keypair, 
    mdns, mplex, noise::{Keypair as NoiseKeypair, X25519Spec, NoiseConfig}, 
    swarm::{Swarm, behaviour, NetworkBehaviour, SwarmEvent, SwarmBuilder},
    tcp as libp2pTCP, Multiaddr, PeerId, Transport,
    gossipsub::{
      MessageId, Gossipsub, GossipsubEvent, GossipsubMessage, 
      IdentTopic as Topic, MessageAuthenticity, ValidationMode,
    }
};
use crate::engine::cvm;
use crate::actors::{
                    parathread::{Parachain, ParachainMsg, Communicate as ParachainCommunicate, Cmd as ParachainCmd, UpdateParachainEvent, ParachainCreated, ParachainUpdated}, //// parathread message evenrs
                    peer::{Validator, ValidatorMsg, Contract, Mode as ValidatorMode, Communicate as ValidatorCommunicate, Cmd as ValidatorCmd, UpdateMode, UpdateTx, ValidatorJoined, ValidatorUpdated, UpdateValidatorAboutMempoolTx, UpdateValidatorAboutMiningProcess}, //// peer message events
                    rafael::env::{Serverless, MetaData, Runtime as RafaelRt, EventLog, EventVariant, RuntimeLog, LinkToService} //-- loading Serverless trait to use its method on Runtime instance (based on orphan rule) since the Serverless trait has been implemented for the Runtime type
                }; 
use crate::schemas::{Transaction, Block, Slot, Chain, Staker, Db, Storage, Mode, P2PChainResponse, P2PLocalChainRequest, P2PAppBehaviourEvent, P2PAppBehaviour};
use crate::constants::*;
use crate::utils::DbORM::StorageModel;
use mongodb::Client;
//// futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by using .await on it also try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
//// StreamExt, FutureExt,  is a trait for streaming utf8 bytes data - RemoteHandle is a handler for future objects which are returned by the remote_handle() method
use futures::{Future, executor::block_on, future::RemoteHandle}; 
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
#[path="tlps/p2p.pubsub.rs"]
pub mod p2p;
pub mod constants;
pub mod schemas;
pub mod actors;
pub mod engine;
pub mod utils; //// we're importing the utils.rs in here as a public module thus we can access all the modules, functions and macros inside of it publicly















#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits; Box is an smart pointer which has valid lifetime for what's inside of it, we're putting the error part of the Result inside the Box since we have no idea about the size of the error or the type that caused this error happened at compile time thus we have to take a reference to it but without defining a specific lifetime
    

    

          
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           mempool job queue channel initialization
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// mempool channel is a broadcast job queue channel with 
    //// multi-producer and multi-consumer in which many values 
    //// can be sent and each receiver sees every value thus 
    //// all transactions must be sent through this 
    //// channel for mining process.
    //
    //// to follow Rust's whole thing of guaranteeing thread safety for mutation 
    //// we need to wrap our data in a Mutex and also the data must be Send and Sync.

    let (mempool_sender, mut mempool_receiver) = broadcast::channel::<(
                                                                    Arc<Mutex<Transaction>>, 
                                                                    Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
                                                                    //// passing the coiniXerr actor system through the broadcast channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
                                                                    //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
                                                                    ActorSystem,
                                                                    //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
                                                                )>(daemon::get_env_vars().get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap()); //-- transaction mempool channel using broadcast channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel




    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                  getting env vars
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    let env_vars = daemon::get_env_vars();


      




    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////             logging setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();






    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                 starting actors
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// by starting actors coiniXerr node state will be 
    //// initialized and the last state types will be
    //// returned here to pass them to different TLPs. 
    
    let (
        mut current_slot, 
        validator_joined_channel, 
        default_parachain_uuid,
        cloned_arc_mutex_runtime_info_object,
        meta_data_uuid,
        cloned_arc_mutex_validator_actor,
        cloned_arc_mutex_validator_update_channel,
        coiniXerr_sys,
        parachain_updated_channel,
        parachain_1,
        parachain_0,
        mining_channel,
        mempool_updated_channel,
        blockchain,
        mut current_block,
    ) = actors::daemonize().await;


    




    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                       bootstrapping TLPS
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// all passed in vars don't implement Copy trait thus 
    //// we have to clone them to prevent ownership moving.
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr RPC SERVER
    // ----------------------------------------------------------------------
    //// used to send transaction from the walleXerr
    //// actor daemonization will be bootstrapped by starting the TCP server
    
    rpc::bootstrap(
        mempool_sender.clone(), //// we can clone only the sender since it's safe to share between new scopes and threads 
        APP_STORAGE.clone(), 
        env_vars.clone(),
        current_slot.clone(),
        validator_joined_channel.clone(),
        default_parachain_uuid.clone(),
        cloned_arc_mutex_runtime_info_object.clone(),
        meta_data_uuid.clone(),
        cloned_arc_mutex_validator_actor.clone(),
        cloned_arc_mutex_validator_update_channel.clone(),
        coiniXerr_sys.clone()
      ).await; //// cap'n proto RPC
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr TCP SERVER
    // ----------------------------------------------------------------------
    //// used to send transaction from a TCP client 
    //// actor daemonization will be bootstrapped by starting the RPC server
    
    tcp::bootstrap(
        mempool_sender.clone(), //// we can clone only the sender since it's safe to share between new scopes and threads 
        APP_STORAGE.clone(), 
        env_vars.clone(),
        current_slot.clone(),
        validator_joined_channel.clone(),
        default_parachain_uuid.clone(),
        cloned_arc_mutex_runtime_info_object.clone(),
        meta_data_uuid.clone(),
        cloned_arc_mutex_validator_actor.clone(),
        cloned_arc_mutex_validator_update_channel.clone(),
        coiniXerr_sys.clone()
      ).await; //// tokio TCP 
    
    // ----------------------------------------------------------------------
    //                    STARTING coiniXerr P2P STACKS
    // ----------------------------------------------------------------------
    //// used to communicate with other coiniXerr nodes
    
    p2p::bootstrap(
        mempool_sender.clone(), //// we can clone only the sender since it's safe to share between new scopes and threads 
        APP_STORAGE.clone(), 
        env_vars.clone(),
        current_slot.clone(),
        validator_joined_channel.clone(),
        default_parachain_uuid.clone(),
        cloned_arc_mutex_runtime_info_object.clone(),
        meta_data_uuid.clone(),
        cloned_arc_mutex_validator_actor.clone(),
        cloned_arc_mutex_validator_update_channel.clone(),
        coiniXerr_sys.clone()
    ).await; //// libp2p stack




    
    
    

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                 bootstrapping coiniXerr VM
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// used to compile the whole node into the BPF bytecode
    //// so we can execute it from the kernel.
    
    cvm::bpf::loader().await;
    
    
    
    





    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           waiting to receive signed transactions asynchronously from the sender to push them inside the current block
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    //// mempool channel is sepecific to each node 
    //// means that only the node itself can see
    //// what's happening inside the mempool
    //// cause it's the transactions' buffer.
    //
    //// since we can't move receiver between new scopes
    //// and threads thus we must receive from the 
    //// job queue channel in here.
 
    while let Ok((transaction, 
                    validator, 
                    coiniXerr_actor_system)) = mempool_receiver.recv().await{ //-- waiting for each transaction to become available to the down side of channel (receiver) for mining process cause sending is done asynchronously also reading from the receiver is a mutable process
        info!("➔ 📥 receiving new transaction and its related validator to push inside the current block");
        let mutex_transaction = transaction.lock().unwrap().clone();
        info!("➔ 🪙 new transaction {:?} in mempool", mutex_transaction);
        let mutex_validator_actor = validator.lock().unwrap().clone();

        let current_uuid_remote_handle: RemoteHandle<Uuid> = ask(&coiniXerr_actor_system, &mutex_validator_actor, ValidatorCommunicate{id: Uuid::new_v4(), cmd: ValidatorCmd::GetValidatorPeerId}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in validator actor and return the result or response as a future object
        let current_validator_uuid = current_uuid_remote_handle.await; //// getting the uuid of the current validator which has passed in to the stream mpsc channel
        info!("➔ 👷🏼‍♂️ validator actor with id [{}] and info {:?} in mempool", current_validator_uuid, mutex_validator_actor);
        
        // ----------------------------------------------------------------------
        //            COMMUNICATE WITH THE VALIDATOR BASED ON TX TYPE
        // ----------------------------------------------------------------------

        //// since we're not sending following messages from another actor actually we're sending from the main() and main() is not an actor thus the sender in tell() method must be None
        if mutex_transaction.ttype == 0x00{ //-- regular transaction comming from walleXerr
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

        while std::mem::size_of_val(&current_block) <= daemon::get_env_vars().get("MAX_BLOCK_SIZE").unwrap().parse::<usize>().unwrap(){ //-- returns the dynamically-known size of the pointed-to value in bytes by passing a reference or pointer to the value to this method - push incoming transaction into the current_block until the current block size is smaller than the daemon::get_env_vars().get("MAX_BLOCK_SIZE")
            current_block.push_transaction(mutex_transaction.clone()); //-- cloning transaction object in every iteration to prevent ownership moving and loosing ownership - adding pending transaction from the mempool channel into the current block for validating that block
            if std::mem::size_of_val(&current_block) > daemon::get_env_vars().get("MAX_BLOCK_SIZE").unwrap().parse::<usize>().unwrap(){
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
        //      INSERTING THE PARACHAIN INTO THE DB USING StorageModel ORM
        // ----------------------------------------------------------------------
        
        //// StorageModel trait is implemented for the InsertParachainInfo struct
        //// thus we can call its ORM methods on each instance of the InsertParachainInfo struct. 
        let parachain_info = schemas::InsertParachainInfo{
            //// we're cloning each field since we're inside the loop and we want to prevent ownership moving
            id: Uuid::new_v4(),
            slot: Some(current_slot.clone()),
            blockchain: Some(blockchain.clone()),
            next_parachain_id: Some(default_parachain_uuid.clone()), //// this is the uuid of the next parachain which is linked to this parachain since connected parachains form a circular pattern
            current_block: Some(current_block.clone()),
        };
        //// calling the save() method of the StorageModel ORM on the parachain_info instance
        //// we have to pass the storage instance each time we're calling one of the ORM method
        //// since we can't save the initialized storage some where inside the struct or the trait
        //// because we can't create instance from the traits!
        match parachain_info.save().await{
            Ok(insert_result) => info!("➔ 🛢️🧣 inserted new parachain into db with uuid [{}] and mongodb id [{}]", default_parachain_uuid.clone(), insert_result.inserted_id.as_object_id().unwrap()),
            Err(e) => error!("😕 error inserting parachain with id [{}]: {}", default_parachain_uuid, e)
        };

    }












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
