





use crate::*;
use self::peer::ValidatorMsg;


pub mod peer;
pub mod parathread;
pub mod rafael;









//// coiniXerr daemonization is the backbone of the coiniXerr network
//// consists of a secured p2p communication between nodes using libp2p, 
//// coiniXerr actors setup, broadcasting events using libp2p pub/sub streams 
//// and receiving asyncly from the mempool channel for mining and verifying process. 
pub async fn daemonize() 
    -> ( //// returning types inside the Arc<Mutex<T>> will allow us to share the type between threads safely
        Slot, 
        ChannelRef<ValidatorJoined>,
        Uuid,
        Arc<Mutex<RafaelRt>>,
        Uuid,
        Arc<Mutex<ChannelRef<ValidatorUpdated>>>,
        Arc<Mutex<ActorRef<ValidatorMsg>>>, //// the validator actor
        ActorSystem,
        ActorRef<ChannelMsg<ParachainUpdated>>,
        ActorRef<ParachainMsg>,
        ActorRef<ParachainMsg>,
        ActorRef<ChannelMsg<UpdateValidatorAboutMiningProcess>>,
        ActorRef<ChannelMsg<UpdateValidatorAboutMempoolTx>>,
        Chain,
        Block,
    ){ 



    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                           env vars initialization
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    let coiniXerr_sys = SystemBuilder::new()
                                                    .name("coiniXerr")
                                                    .create()
                                                    .unwrap(); //// unwrapping the last functional method 
    info!("➔ 🟢 actor system and storage are set up");
    let mut runtime_info = RafaelRt(HashMap::new());
    let runtime_instance = runtime_info.run(); //-- run() method is the method of the Rafael serverless trait
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(runtime_instance)); //-- we can clone the runtime_instance without using Arc cause Clone trait is implemented for RafaelRt -> MetaData -> Validator actor
    let buffer_size = daemon::get_env_vars().get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    //// if the type doesn't implement the Copy trait 
    //// thus we can't move out of the dereference type
    //// cause it'll be moved and we have to either 
    //// borrow the dereferenced type using & or clone 
    //// it also we can't move a shared reference into 
    //// new scopes since it's a shared reference and 
    //// is being used by other scopes if we do that 
    //// we'll face dangling pointer issue which 
    //// rust doesn't like it!
    //
    //// Clone trait is not implemented for receiver thus
    //// the Copy trait can't be implemented also since 
    //// Clone is a supertrait of Copy and because of this
    //// we can't move out of the type by deferencing it at all!
    let storage = APP_STORAGE.clone(); //// cloning it in order not to move since Copy trait is not implemented for the dereferenced type or Option<Arc<schemas::Storage>> thus we have to either borrow it in case that the clone is also not implemented or clone it




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
    //// the ask model which is using a oneshot channel on behalf also all actors use 
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

    // ----------------------------------------------------------------------
    //         BUILDING VALIDATOR ACTOR FOR THIS STREAM USING PEER ID
    // ----------------------------------------------------------------------
    //// every node or peer is a validator actor which is known by
    //// a keypair (public and private) and the generated
    //// peer_id from that keypair.
    
    let this_peer_id = PEER_ID.to_string(); //// dereferencing the peer_id for this peer also peer_id can be a unique identifier for the connected validator since it has a unique id each time that a validator gets slided into the network
    info!("➔ 🎡 peer id for this node [{}]", this_peer_id);
    let cloned_arc_mutex_runtime_info_object = Arc::clone(&arc_mutex_runtime_info_object); //-- cloning (making a deep copy of) runtime info object to prevent ownership moving in every iteration between threads
    let default_parachain_slot = current_slot.clone();
    let peer_validator = default_parachain_slot.clone().get_validator(this_peer_id.clone()); //// passing the current peer_id of this node to get the validator info
    if let None = peer_validator{ //// means we don't find any validator inside the default parachain slot  
        current_slot = default_parachain_slot
                                            .clone()
                                            //// this method will return the updated slot by adding new validator info to the parachain slot 
                                            //// adding a new validator with the generated peer_id and key pairs of this node 
                                            .add_validator( 
                                                default_parachain_uuid, 
                                                this_peer_id.clone(), 
                                            );
    }
    
    //// building a validator instance from the peer_validator 
    //// returned from the default_parachain_slot, we have to
    //// set each field to a default value if the returned validator
    //// was None.  
    let validator = Validator{ //// we have to clone the peer_validator in each arm to prevent ownership moving since we're lossing the ownership in each arm
        peer_id: match peer_validator.clone(){
            Some(v) => v.peer_id,
            None => this_peer_id.clone(), //// if there was not peer_id we'll use the one inside the constant
        },
        recent_transaction: match peer_validator.clone(){
            Some(v) => v.recent_transaction,
            None => None,
        },
        mode: match peer_validator.clone(){
            Some(v) => v.mode,
            None => ValidatorMode::Mine,
        },
        ttype_request: match peer_validator.clone(){
            Some(v) => v.ttype_request,
            None => None,
        }
    };

    info!("➔ 👷🏼‍♂️ building validator actor for this peer");
    let validator_props = Props::new_args::<Validator, _>( //// prop types are inside Arc and Mutex thus we can clone them and move them between threads  
                                                                                                        (
                                                                                                            validator.peer_id.clone(), 
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
                                    msg: ValidatorJoined(validator.peer_id.clone()), //// publishing the ValidatorJoined message event to the validator_joined_channel channel 
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
        let mut runtime_info = cloned_arc_mutex_runtime_info_object.lock().unwrap().to_owned(); //-- in order to use the to_owned() method we have to implement the Clone trait for the Runtime struct since this method will make a clone from the instance - unlocking, unwrapping and cloning (by using to_ownded() method) the runtim_info object in every iteration of incoming stream inside the local thread to convert it to an instance of the RafaelRt struct
        RafaelRt::add( //-- locking on runtime info object (mutex) must be done in order to prevent other threads from mutating it at the same time 
            runtime_info, //-- passing the mutable runtime_info object for adding new metadata into its hash map field
            MetaData{ //// this metadata will be used for selecting new validators inside a shard
                id: Uuid::new_v4(),
                node_peer_id: Some(this_peer_id.clone()), //// this is the peer_id of this node 
                actor: validator_actor.clone(), //-- cloning (making a deep copy of) the validator actor will prevent the object from moving in every iteration
                link_to_server: None,
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



    (   
        current_slot.clone(), 
        validator_joined_channel.clone(),
        default_parachain_uuid.clone(),
        cloned_arc_mutex_runtime_info_object.clone(),
        meta_data_uuid.clone(),
        cloned_arc_mutex_validator_update_channel.clone(),
        cloned_arc_mutex_validator_actor.clone(),
        coiniXerr_sys.clone(),
        parachain_updated_channel.clone(),
        parachain_1.clone(),
        parachain_0.clone(),
        mining_channel.clone(),
        mempool_updated_channel.clone(),
        blockchain.clone(),
        current_block.clone(),
    )



} 
