


// https://stackoverflow.com/questions/50297252/actors-thread-based-vs-event-driven
// TODO - networking projects that must build with actors:
//      • traffic forwarding tools like ngrok using iptables
//      • proxy and all layers load balancer like pingora based on cpu task scheduling, weighted round robin dns, vector clock, event loop and simd vectorization 
//      • vpn like v2ray protocols with zero proof of knowledge  
//      • binary address transmition protocol like onionary://010101000001:2324 acts as a message broker like rmq, zmq, kafka, load balancer and proxy




use crate::*;
pub mod peer;
pub mod parathread;
pub mod rafael;
pub mod unixerr;








//// starting coiniXerr actors, broadcast events using ZMQ pub/sub 
//// and receiving asyncly from the mempool channel.
pub async fn daemonize(mut mempool_receiver: tokio::sync::mpsc::Receiver<(
        Arc<Mutex<Transaction>>, 
        Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
        //// passing the coiniXerr actor system through the mpsc channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
        //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
        ActorSystem 
        //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
    )>,
    storage: Option<Arc<Storage>>
) -> (Slot, 
      ChannelRef<ValidatorJoined>, 
      ChannelRef<ValidatorUpdated>, 
      Uuid,
      Arc<Mutex<RafaelRt>>,
      ActorSystem
    ){



    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                           env vars initialization
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    let coiniXerr_sys = SystemBuilder::new()
                                                    .name("coiniXerr")
                                                    .create()
                                                    .unwrap(); //// unwrapping the last functional method 
    info!("➔ 🟢 actor system and storage are set up");
    let mut run_time_info = RafaelRt(HashMap::new());
    let runtime_instance = run_time_info.run(); //-- run() method is the method of the Rafael serverless trait
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(runtime_instance)); //-- we can clone the runtime_instance without using Arc cause Clone trait is implemented for RafaelRt -> MetaData -> Validator actor
    let buffer_size = daemon::get_env_vars().await.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let zmq_addr = daemon::get_env_vars().await.get("ZMQ_ADDR").unwrap().to_string();


    

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////              ZMQ pub/sub stream to broadcast actors' events to the whole networks using cap'n proto serialization
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// ➔ we'll use the ZMQ pub/sub to broadcast the network events from RPC or tokio TCP to other ndoes
    ////    also when a socket is bound to an endpoint it automatically starts accepting connections.
    // 
    //// ➔ ZMQ sockets may be connected to multiple endpoints, while simultaneously accepting incoming connections from 
    ////    multiple endpoints bound to the socket, thus allowing many-to-many relationships.
    // 
    //// ➔ ZMQ contexts are thread safe data types means we can clone them to share between threads (they are Arc-ed) 
    ////    and also they avoid deadlocks since ZMQ socket protocols use actors under the hood means 
    ////    both senders and receivers are actors which use a buit in jobq to handle incoming tasks and jobs. 
    // 
    //// ➔ ZMQ creates queues (actor) per underlying connection of each socket type if your socket is connected to three peer sockets, 
    ////    then there are three messages queues behind the scenes, queues are created as individual peers connect to the bound socket   
    //
    //// ➔ every ZMQ sender and receiver socket type is an actor which sends and receive in parallel manner since actors use worker threadpool
    ////    (like tokio::spawn() worker green based threadpool + tokio channels for sharing messages between threads), 
    ////    job or task queue channels for task scheduling, pub/sub channels for broadcasting messages to other actors  
    ////    and mailbox to communicate with each other and outside of the actor system under the hood.
    //
    //// ➔ RPC allows us to directyly call methods on other machines and it's a 
    ////    bidirectional full-duplex streaming in which the client can request and 
    ////    the server can respond simultaneously and at the same time.  
    //
    //// ➔ ZMQ supports N-to-N pattern means a publisher will accept any number of subscribers 
    ////    and the subscriber can connect to multiple publishers.
    //
    //// ➔ ZMQ patterns are:
    ////      • Request-reply, which connects a set of clients to a set of services. This is a remote procedure call and task distribution pattern.
    ////      • Pub-sub, which connects a set of publishers to a set of subscribers. This is a data distribution pattern.
    ////      • Pipeline, which connects nodes in a fan-out/fan-in pattern that can have multiple steps and loops. This is a parallel task distribution and collection pattern.
    ////      • Exclusive pair, which connects two sockets exclusively. This is a pattern for connecting two threads in a process, not to be confused with “normal” pairs of sockets.
    ////      • Client-server, which allows a single ZMQ server talk to one or more ZMQ clients. The client always starts the conversation, after which either peer can send messages asynchronously, to the other.
    ////      • Radio-dish, which used for one-to-many distribution of data from a single publisher to multiple subscribers in a fan out fashion.    

    // ---------------------------------------------------------------------------------------------------------------------------
    //        ZMQ P2P PUBLISHER AND SUBSCRIBER USING CAP'N PROTO SERIALIZATION (DESIGNED FOR coiniXerr NODES COMMUNICATION)
    // ---------------------------------------------------------------------------------------------------------------------------
    
    let zmq_ctx = zmq::Context::new(); 
    let publisher = zmq_ctx.socket(zmq::XPUB).unwrap(); //// the publisher actor node
    let subscriber = zmq_ctx.socket(zmq::XSUB).unwrap(); //// the subscriber actor node 
    let mut msg = zmq::Message::new(); //// a message is a single frame which can be any type, either received or created locally and then sent over the wire through the zmq socket

    //// both publisher and subscriber are inside the node
    //// means that the publisher can be the subscriber too
    //// to subscriber what has been just published.
    publisher
        .bind(zmq_addr.as_str()) //// binding the publisher to the passed in address
        .unwrap();

    subscriber
        .connect(zmq_addr.as_str()) //// connecting the subscriber to the passed in address
        .unwrap();
    

    // TODO - use cap'n proto as the serialization protocol for event encoding
    // TODO - broadcast actors' events to other nodes using publisher
    // TODO - subscribe to actors' events using the subscriber
    // TODO - test the p2p behavior of the node using zmq pub/sub n-to-n pattern
    // ...





    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           libp2p pub/sub stream to broadcast actors' events to the whole networks
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    // ...








    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                     building actor coiniXerr events channels 
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
    ///////           waiting to receive signed transactions asynchronously from the sender to push them inside the current block
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    //// mempool channel is sepecific to each node 
    //// means that only the node itself can see
    //// what's happening inside the mempool
    //// cause it's the transactions' buffer.
 
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

        while std::mem::size_of_val(&current_block) <= daemon::get_env_vars().await.get("MAX_BLOCK_SIZE").unwrap().parse::<usize>().unwrap(){ //-- returns the dynamically-known size of the pointed-to value in bytes by passing a reference or pointer to the value to this method - push incoming transaction into the current_block until the current block size is smaller than the daemon::get_env_vars().await.get("MAX_BLOCK_SIZE")
            current_block.push_transaction(mutex_transaction.clone()); //-- cloning transaction object in every iteration to prevent ownership moving and loosing ownership - adding pending transaction from the mempool channel into the current block for validating that block
            if std::mem::size_of_val(&current_block) > daemon::get_env_vars().await.get("MAX_BLOCK_SIZE").unwrap().parse::<usize>().unwrap(){
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
        match parachain_info.save(storage.clone()).await{
            Ok(insert_result) => info!("➔ 🛢️🧣 inserted new parachain into db with uuid [{}] and mongodb id [{}]", default_parachain_uuid.clone(), insert_result.inserted_id.as_object_id().unwrap()),
            Err(e) => error!("😕 error inserting parachain with id [{}]: {}", default_parachain_uuid, e)
        };

    }


    //// returning the tuple of current slot, 
    //// validator joined channel, validator updated channel 
    //// default parachain uuid, arc and mutex-ed rafael runtime
    //// and the coiniXerr actor system. 
    (   
        current_slot.clone(), 
        validator_joined_channel.clone(), 
        validator_updated_channel.clone(),
        default_parachain_uuid.clone(),
        arc_mutex_runtime_info_object.clone(),
        coiniXerr_sys.clone()
    )

} 