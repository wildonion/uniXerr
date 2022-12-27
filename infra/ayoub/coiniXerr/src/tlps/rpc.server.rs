



use crate::*;






//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and veifying process.
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

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           RPC stream to decode incoming transactions using cap'n proto serialization from walleXerr
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let rpc_addr = env_vars.get("RPC_ADDR").unwrap().as_str();                                                                                                                         //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop                                                                                                                                            )>(buffer_size); //-- transaction mempool channel using mpsc channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel

    // -----------------------------------------------------------       
    //          RPC SERVER USING CAP'N PROTO SERIALIZATION
    // -----------------------------------------------------------
    //// with RPC we can call a method of an encoded actor object
    //// using cap'n proto directly from other devices.
    


    
    // https://github.com/capnproto/capnproto-rust/tree/master/capnp-rpc
    // https://capnproto.org/language.html
    // TODO - implementing cap'n proto structures for coiniXerr transactions comming from the walleXerr with compilation commands in `app.sh` 
    // TODO - we must send the new decoded transaction to the downside of the mempool channel for mining and verifying process
    // ... 








    


    
}
