







use crate::*;






//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and consensus process.
pub async fn bootstrap(
        reset_slot_receiver: mpsc::Receiver<bool>,
        mempool_sender: broadcast::Sender<(Arc<Mutex<Transaction>>, Arc<Mutex<ActorRef<ValidatorMsg>>>, ActorSystem)>, //// we'll use this sender to send transactions to downside of the mempool channel for mining process
        storage: Option<Arc<Storage>>, 
        env_vars: HashMap<String, String>,
        current_slot: Slot, 
        validator_joined_channel: ChannelRef<ValidatorJoined>,
        default_parachain_uuid: Uuid,
        parachain: ActorRef<ParachainMsg>,
        parachain_updated_channel: ActorRef<ChannelMsg<ParachainUpdated>>,
        cloned_arc_mutex_runtime_info_object: Arc<Mutex<RafaelRt>>,
        meta_data_uuid: Uuid,
        cloned_arc_mutex_validator_update_channel: Arc<Mutex<ChannelRef<ValidatorUpdated>>>,
        cloned_arc_mutex_validator_actor: Arc<Mutex<ActorRef<ValidatorMsg>>>, //// the validator actor
        cloned_arc_mutex_new_chain_channel: Arc<Mutex<ActorRef<ChannelMsg<UpdateValidatorAboutNewChain>>>>,
        coiniXerr_sys: ActorSystem
    ){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////          scaffolding p2p websocket network stacks, services and requirements
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let websocket_addr = env_vars.get("WEBSOCKET_ADDR").unwrap().as_str();                                                                                                                         //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop                                                                                                                                            )>(buffer_size); //// transaction mempool channel using mpsc channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel
    


    
}
