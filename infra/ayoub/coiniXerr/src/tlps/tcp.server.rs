



use crate::*;





//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and consensus process.
pub async fn bootstrap(
        mempool_sender: broadcast::Sender<(Arc<Mutex<Transaction>>, Arc<Mutex<ActorRef<ValidatorMsg>>>, ActorSystem)>, //// we'll use this sender to send transactions to downside of the mempool channel for mining process
        storage: Option<Arc<Storage>>, 
        env_vars: HashMap<String, String>,
        current_slot: Slot, 
        validator_joined_channel: ChannelRef<ValidatorJoined>,
        default_parachain_uuid: Uuid,
        cloned_arc_mutex_runtime_info_object: Arc<Mutex<RafaelRt>>,
        meta_data_uuid: Uuid,
        cloned_arc_mutex_validator_update_channel: Arc<Mutex<ChannelRef<ValidatorUpdated>>>,
        cloned_arc_mutex_validator_actor: Arc<Mutex<ActorRef<ValidatorMsg>>>, //// the validator actor
        coiniXerr_sys: ActorSystem
    ){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////          TCP stream to decode incoming transactions using borsh and serde from TCP clients
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------
    
    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let tcp_addr = env_vars.get("TCP_ADDR").unwrap().as_str();
    
    // ----------------------------------------------------------------------
    //                  DEFINING TCP STREAMER MPSC CHANNELS
    // ----------------------------------------------------------------------
    //// following channels will be used to share 
    //// data between tokio threads
    //// also inside each node.
    
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
                                                                                                                            )>(buffer_size); //// mpsc channel to send the incoming stream, the generated uuid of the runtime info object and the runtime info object itself to multiple threads through the channel for each incoming connection from the socket
    
    // ----------------------------------------------------------------------
    //                       STARTING TOKIO TCP SERVER
    // ----------------------------------------------------------------------

    let listener = TcpListener::bind(env_vars.get("TCP_ADDR").unwrap()).await.unwrap();
    info!("➔ 🟢 tcp listener is ready");

    // ----------------------------------------------------------------------
    //                    STARTING TRANSACTION EMULATORS
    // ----------------------------------------------------------------------    
    // if dotenv initialization is before the starting the emulator process means we're ok since the whole env file will be loaded into the ram and 
    // when we want to load vars it's ok but if we put the starting the emulator process before loading dotenv we'll face error since dotenv doesn't initialize yet.

    utils::tcp_tx_emulator().await;

    


    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                                 waiting on TCP streamer to get transaction bytes asyncly 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    while let Ok((stream, addr)) = listener.accept().await{ //// await suspends the accept() function execution to solve the future but allows other code blocks to run      
        info!("➔ 🪢 connection stablished from {}", addr);
        let stream_sender = stream_sender.clone(); //// we're using mpsc channel to send data between tokio tasks and each task or stream needs its own sender; based on multi producer and single consumer pattern we can achieve this by cloning (making a deep copy of) the sender for each incoming stream means sender can be owned by multiple threads but only one of them can have the receiver at a time to acquire the mutex lock        
        info!("➔ 📼 sending stream setups through the channel");
        stream_sender.send((stream, 
                            meta_data_uuid, 
                            cloned_arc_mutex_runtime_info_object.clone(), 
                            cloned_arc_mutex_validator_actor.clone(), 
                            cloned_arc_mutex_validator_update_channel.clone(), 
                            coiniXerr_sys.clone()
                        )).await.unwrap(); //// sending the stream, the cloned runtime info and metadata uuid, cloned validator, coiniXerr actor system and the validator update channel through the mpsc channel 
            
    }




    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                                 waiting to receive stream and other setups asynchronously 
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// we could also use the following syntax:
    ////    while match stream_receiver.recv().await{
    ////       Some() => {
    ////
    ////       }, 
    ////       None => {
    ////               
    ////       }
    ////    }

    while let Some((
                    mut stream, 
                    generated_uuid, 
                    cloned_arc_mutex_runtime_info_object, 
                    cloned_arc_mutex_validator_actor, 
                    cloned_arc_mutex_validator_update_channel, 
                    coiniXerr_actor_system
                )) = stream_receiver.recv().await.take(){ //// waiting for the stream and other setups to become available to the down side of channel (receiver) to change the started validator actor's transaction for every incoming connection - stream must be mutable for reading and writing from and to socket
        
        info!("➔ 📥 receiving the stream setups");
        let mempool_sender = mempool_sender.clone(); //// cloning mempool_sender to send signed transaction through the channel to the receiver for mining process
        
        //// ... the following will handle the the incoming stream inside a tokio worker green threadpool measn tokio::spawn() is an async task worker green threadpool solver 
        //// ... move will move everything from its behind to the new scope and take their ownership so there is not a single var after moving in second iteration of the loop thus we've passed all the requirements that might be moved by doing that we can make sure that we have them again after first iteration 
        //// ... beacuse in the first iteration async move{} will move the everything and takes the ownership from its behind thus in second iteration we don't have them and to solve this issue we have to pass them in the channel to have them in every iteration  
        tokio::spawn(async move { //// this is an async task related to updating a validator actor on every incoming message from the sender which is going to be solved in the background on a single (without having to work on them in parallel) thread using green threadpool of tokio runtime and message passing channels like mpsc job queue channel protocol
            let mut transaction_buffer_bytes = vec![0 as u8; buffer_size]; //// using [0 as u8; buffer_size] gives us the error of `attempt to use a non-constant value in a constant` cause [u8] array doesn't implement the Sized trait
            while match stream.read(&mut transaction_buffer_bytes).await{ //// streaming over the incoming bytes from the socket - reading is the input and writing is the output
                Ok(size) if size == 0 => false, //// socket closed since zero bytes data are here!
                Ok(size) => {
                    
                    // ----------------------------------------------------------------------
                    //                                 SIMD OPS
                    // ----------------------------------------------------------------------
                    
                    let heavy_func = |chunk: u8| {
                        let byte_name = Ok::<&[u8], String>(b"wildonion");
                        info!("➔ \t--------Doing some heavy operation on chunk [{:?}]", chunk);
                        chunk
                    };
                    let bytes_slice = utils::into_box_slice(&transaction_buffer_bytes).await.unwrap(); //// converting transaction_buffer_bytes into a Box of 4 u8 slice
                    let start = Instant::now();
                    match utils::simd(u32::from_be_bytes(*bytes_slice), heavy_func).await{ //// passing a u32 bits integer by dereferencing the Box which has the bytes_slice value itself - from_be_bytes() method creates a native endian integer value from its representation as a byte array in big endian
                        Ok(result) => {
                            let end = Instant::now();
                            let delta = end.duration_since(start);
                            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
                            // assert_eq!(3985935_u32, result); //// it'll panic on not equal condition
                            info!("➔ ::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
                            let msg_to_write = format!("::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
                            stream.write(&msg_to_write.as_bytes()).await.unwrap(); //// sending the simd result String message as utf8 bytes back to the peer
                        },
                        Err(e) => {
                            info!("➔ ::::: error in reading chunk caused by {:?}", e);
                            let msg_to_write = format!("::::: error in reading chunk caused by {:?}", e);
                            stream.write(&msg_to_write.as_bytes()).await.unwrap(); //// sending the simd error String message as utf8 bytes back to the peer
                        },
                    };
                    
                    // ---------------------------------------------------------------------------------------
                    //          SERDING INCOMING IO STREAM OF TRANSACTION CHUNKS USING serde & borsh
                    // ---------------------------------------------------------------------------------------
                    // NOTE - ..size means from the beginning to the limit - 1, we could also use 0..size

                    let deserialized_transaction_union = Transaction::new(&transaction_buffer_bytes[..size]).unwrap(); //// decoding process of incoming transaction - deserializing a new transaction bytes into the Transaction struct object using TransactionMem union
                    let deserialized_transaction_serde = &mut serde_json::from_slice::<Transaction>(&transaction_buffer_bytes[..size]).unwrap(); //// decoding process of incoming transaction - deserializing a new transaction bytes coming from the steamer into a mutable Transaction object using serde_json::from_slice to mutate the signed field 
                    let deserialized_transaction_borsh = &mut Transaction::try_from_slice(&transaction_buffer_bytes[..size]).unwrap(); //// passing the vector of utf8 bytes into the try_from_slice() method to deserialize into the SMSResponse struct - since Vec<u8> will be coerced to &'a [u8] at compile time we've passed Vec<u8> type into the try_from_slice() method; since we want to sign the transaction thus we must define it as mutable
                    let mut transaction_serialized_into_vec_bytes_using_serede = serde_json::to_vec(&deserialized_transaction_serde).unwrap(); //// converting the deserialized_transaction_serde object into vector of utf8 bytes using serde
                    let mut transaction_serialized_into_vec_bytes_using_borsh = deserialized_transaction_borsh.try_to_vec().unwrap(); //// converting the transaction object into vector of utf8 bytes using borsh

                    // ----------------------------------------------------------------------
                    //              SIGNING TRANSACTION WITH THE CURRENT NODE TIME
                    // ----------------------------------------------------------------------
                    //// the transaction can only be signed by the node
                    //// if the signature is a valid one.
                    //
                    //// the issued time of the transaction must be smaller than the current node time
                    //// also if the transaction signature was valid we could sign the transaction 
                    //// with the node current time.
                    
                    let must_be_signed = deserialized_transaction_borsh.is_transaction_valid();
                    let now = chrono::Local::now().naive_local().timestamp();
                    if must_be_signed && deserialized_transaction_borsh.issued < now{ 
                        
                        // ----------------------------------------------------------------------
                        //             GENERATING THE HASH OF THE SIGNED TRANSACTION
                        // ----------------------------------------------------------------------
                        info!("➔ 🥣 generating the hash of the signed transaction comming from the waleXerr");
                        deserialized_transaction_borsh.generate_hash(); //// this hash will be used in calculating merkle root process

                        // ----------------------------------------------------------------------
                        //            SIGNING THE INCOMING TRANSACTION WITH SERVER TIME
                        // ----------------------------------------------------------------------
                        
                        info!("➔ ✍️ signing incoming transaction with the current node time");
                        deserialized_transaction_borsh.signed = Some(chrono::Local::now().naive_local().timestamp()); //// signing the incoming transaction with the current server time
                        
                        // ----------------------------------------------------------------------
                        //        ENCODING SIGNED TRANSACTION THEN SENDING BACK TO THE PEER
                        // ---------------------------------------------------------------------- 

                        let mut signed_transaction_serialized_into_vec_bytes_using_borsh = deserialized_transaction_borsh.try_to_vec().unwrap(); //// converting the signed transaction object into vector of utf8 bytes using borsh                    
                        let mut utf8_bytes_using_as_mut_slice = signed_transaction_serialized_into_vec_bytes_using_borsh.as_mut_slice(); //// converting Vec<u8> to mutable slice of &[u8] using as_mut_slice() method - remeber that signed_transaction_serialized_into_vec_bytes_using_borsh must be defined as mutable
                        let utf8_bytes_using_casting: &[u8] = &signed_transaction_serialized_into_vec_bytes_using_borsh; //// since the Vec<u8> will be coerced to &'a [u8] with a valid lifetime at compile time we can borrow the ownership of sms_response_serialized_into_vec_bytes_using_serede using & which by doing this we're borrowing a slice of Ve<u8> from the heap memory which will be coerced to &'a [u8] since we've specified the type of sms_response_serialized_into_utf8_bytes_using_serede which is &[u8]
                        let boxed_utf8_bytes_using_box_slcie = signed_transaction_serialized_into_vec_bytes_using_borsh.into_boxed_slice(); //// converting the Vec<u8> to Box<u8> using into_boxed_slice() method 
                        let utf_bytes_dereference_from_box = &*boxed_utf8_bytes_using_box_slcie; //// borrow the ownership of the dereferenced boxed_utf8_bytes_using_box_slcie using & to convert it to &[u8] with a valid lifetime since the dereferenced boxed_utf8_bytes_using_box_slcie has unknown size at compile time thus working with u8 slice needs to borrow them from the heap memory to have their location address due to implemented ?Sized for [u8]
                        info!("➔ 🪙✍️ sending signed transaction back to the peer");
                        stream.write(&utf_bytes_dereference_from_box).await.unwrap(); //// sending the signed transaction back to the peer - since Vec<u8> will be coerced to &'a [u8] with valid lifetime at compile time we can also send the signed_transaction_serialized_into_vec_bytes_using_borsh directly through the socket even though the write() method takes &'a [u8] param with a valid lifetime 
                        
                        // ----------------------------------------------------------------------
                        //       UPDATING VALIDATOR ACTOR WITH THE LATEST SIGNED TRANSACTION
                        // ----------------------------------------------------------------------

                        info!("➔ 👷🏼‍♂️🔃 updating validator actor with the recent signed transaction");
                        for (id, md) in cloned_arc_mutex_runtime_info_object.lock().unwrap().0.iter_mut(){ //// id and md are &mut Uuid and &mut MetaData respectively - we have to iterate over our info_dict mutably and borrowing the key and value in order to update the validator actor transaction of our matched meta_data id with the incoming uuid
                            if id == &generated_uuid{
                                let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&utf_bytes_dereference_from_box).unwrap(); //// deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                                md.update_validator_transaction(Some(signed_transaction_deserialized_from_bytes)); //// update the validator actor with a recent signed transaction
                            }
                        }

                        // ---------------------------------------------------------------------------------
                        //              BROADCASTING VALIDATOR UPDATE TO OTHER VALIDATOR ACTORS
                        // ---------------------------------------------------------------------------------

                        let validator_update_channel = cloned_arc_mutex_validator_update_channel.lock().unwrap().clone(); //// cloning will return the T from MutexGuard
                        let current_validator = cloned_arc_mutex_validator_actor.lock().unwrap().clone(); //// cloning will return the T from MutexGuard
                        let current_peer_id_remote_handle: RemoteHandle<String> = ask(&coiniXerr_actor_system, &current_validator, ValidatorCommunicate{id: Uuid::new_v4(), cmd: ValidatorCmd::GetValidatorPeerId}); //// no need to clone the passed in parachain since we're passing it by reference - asking the coiniXerr system to return the uuid of the passed in validator actor and return the result or response as a future object; the passed in Uuid is the Uuid of the generated command 
                        let current_validator_peer_id = current_peer_id_remote_handle.await; //// getting the peer_id of the current validator which has passed in to the stream mpsc channel
                        validator_update_channel.tell( //// telling the channel that we want to publish something
                                                    Publish{
                                                        msg: ValidatorUpdated(current_validator_peer_id.clone()), //// publishing the ValidatorUpdated message event to the validator_updated_channel channel 
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
                        let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&utf_bytes_dereference_from_box).unwrap(); //// deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
                        let arc_mutex_transaction = Arc::new(Mutex::new(signed_transaction_deserialized_from_bytes)); //// putting the signed_transaction_deserialized_from_bytes inside a Mutex to borrow it as mutable inside Arc by locking the current thread 
                        let cloned_arc_mutex_transaction = Arc::clone(&arc_mutex_transaction); //// cloning the arc_mutex_transaction to send it through the mpsc job queue channel 
                        mempool_sender.send((cloned_arc_mutex_transaction, cloned_arc_mutex_validator_actor.clone(), coiniXerr_actor_system.clone())).unwrap(); //// sending signed transaction plus the validator actor info through the mpsc job queue channel asynchronously for mining process - we must clone the cloned_arc_mutex_validator_actor in each iteration to prevent ownership moving
                        true
                    } else{
                        
                        // ----------------------------------------------------------------------
                        //       REJECTING THE INCOMING TRANSACTION BACK TO THE VALIDATOR
                        // ----------------------------------------------------------------------
                        
                        info!("➔ 🙅 rejecting incoming transaction caused by invalid transaction signature");
                        stream.write(&transaction_buffer_bytes[..size]).await.unwrap(); //// rejecting the transaction back to the peer
                        true
                    }
                },
                Err(e) => {
                    info!("➔ 🔚 terminating connection with validator {}", stream.peer_addr().unwrap());
                    stream.shutdown().await.unwrap(); //// shuts down the output stream
                    false
                }
            } {} //// the while match must be a block which will return true on its Ok() arm and false on its Err arm
        }); //// awaiting on tokio::spawn() will block the current task which is running in the background so we don't await on it just let it be as it is :)
    }

}
