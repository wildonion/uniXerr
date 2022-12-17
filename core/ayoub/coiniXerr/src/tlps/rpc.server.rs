



use crate::*;






//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and veifying process.
pub async fn bootstrap(storage: Option<Arc<Storage>>, env_vars: HashMap<String, String>){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////        RPC stream to decode incoming transactions using cap'n proto serialization from walleXerr
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let rpc_addr = env_vars.get("RPC_ADDR").unwrap().as_str();
    
    // -------------------------------------------------------
    //              DEFINING MEMPOOL MPSC CHANNEL
    // -------------------------------------------------------
    //// following channels will be used to share data between tokio threads
    //// also inside each node; mempool channel is an mpsc job queue channel which 
    //// all transactions must be sent through this channel for mining process.
    //// to follow Rust's whole thing of guaranteeing thread safety for mutation 
    //// we need to wrap our data in a Mutex and also the data must be Send and Sync.
    
    let (mempool_sender, mempool_receiver) = mpsc::channel::<(
                                                                                                                                                Arc<Mutex<Transaction>>, 
                                                                                                                                                Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
                                                                                                                                                //// passing the coiniXerr actor system through the mpsc channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
                                                                                                                                                //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
                                                                                                                                                ActorSystem 
                                                                                                                                                //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
                                                                                                                                            )>(buffer_size); //-- transaction mempool channel using mpsc channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel


    // -----------------------------------------------------------------------
    //    STARTING ACTORS TO SEND TRANSACTIONS ASYNCLY TO MEMPOOL CHANNELS
    // -----------------------------------------------------------------------
    let (
            mut current_slot, 
            validator_joined_channel, 
            validator_updated_channel,
            default_parachain_uuid,
            arc_mutex_runtime_info_object,
            coiniXerr_sys
        ) = actors::daemonize(mempool_receiver, storage.clone()).await;

    // -----------------------------------------------------------       
    //          RPC SERVER USING CAP'N PROTO SERIALIZATION
    // -----------------------------------------------------------
    // https://github.com/capnproto/capnproto-rust/tree/master/capnp-rpc    
    // https://capnproto.org/language.html
    //// in RPC both server and client know the exact structure of the request and response
    //// for realtime streaming which will be defined by the cap'n proto serialization schemas.
    //
    //// RPC allows us to directyly call methods on other machines and it's a 
    //// bidirectional full-duplex streaming in which the client can request and 
    //// the server can respond simultaneously and at the same time. 


    // TODO - implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr with compilation commands in `app.sh` 
    // TODO - we must send the new decoded transaction to the downside of the mempool channel for mining and verifying process
    // ... 








    


    
}