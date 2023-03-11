



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

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////          RPC stream to verify cap'n proto transactions coming from walleXerr
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let rpc_addr = env_vars.get("RPC_ADDR").unwrap().as_str();                                                                                                                         //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop                                                                                                                                            )>(buffer_size); //// transaction mempool channel using mpsc channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel
    
    // -----------------------------------------------------------       
    //        RPC PUBLISHER USING CAP'N PROTO SERIALIZATION
    // -----------------------------------------------------------
    // https://capnproto.org/rpc.html
    //// with RPC we can call a method of an cap'n proto encoded 
    //// actor object directly from other devices like calling 
    //// a contract method from another contract also the results 
    //// of an RPC call are returned to the client instantly, before 
    //// the server even receives the initial request.
    //
    //// the generated code from the compiled capnp file 
    //// includes a Server trait for each interfaces,
    //// and to create an RPC-enabled object, we must implement 
    //// that trait for the RPC object which can be an actor object
    //// also for each defined interface, the generated 
    //// code includes a Client struct that can be used 
    //// to call the interface's or actor object's methods.
    //
    //// zmq and rpc will be used for device actor communication 
    //// if we want to use them in browser we must setup websocket 



    
    // https://github.com/capnproto/capnproto-rust/tree/master/capnp-rpc/examples
    // https://capnproto.org/language.html
    // TODO - defien get_latest_blockhash, get_balance_of and etc ... methods to call them directly from the client using an RPC call to the coiniXerr node  
    // TODO - RPC capnp and borsh codec method call based on pubsub streaming
    // TODO - accessing Arc<Mutex<data>> + Send + Sync + 'static between threads using tokio channels  
    // TODO - implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr with compilation commands in `app.sh` 
    // TODO - first decode transaction then sign it like in tcp and udp server
    // TODO - send the signed transaction to the downside of the mempool channel for mining and verifying process inside the node.rs
    // ... 

    


    


    
}
