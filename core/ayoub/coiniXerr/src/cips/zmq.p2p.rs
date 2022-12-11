






use crate::*;








pub async fn bootstrap(storage: Option<Arc<Storage>>, env_vars: HashMap<String, String>){
    
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                 coiniXerr nodes communications using cap'n proto serialization based on ZMQ protocol
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// ➔ ZeroMQ sockets may be connected to multiple endpoints, while simultaneously accepting incoming connections from 
    ////    multiple endpoints bound to the socket, thus allowing many-to-many relationships.
    // 
    //// ➔ ZMQ contexts are thread safe data types means we can clone them to share between threads (they are Arc-ed) 
    ////    and also they avoid deadlocks since ZMQ socket protocols use actors under the hood means 
    ////    both senders and receivers are actors which use a buit in jobq to handle incoming tasks and jobs. 
    // 
    //// ➔ ZeroMQ creates queues (actor) per underlying connection of each socket type if your socket is connected to three peer sockets, 
    ////    then there are three messages queues behind the scenes, queues are created as individual peers connect to the bound socket   
    //
    //// ➔ every sender and receiver socket type in ZMQ is an actor since actors use worker threadpool
    ////    (like tokio::spawn() worker green based threadpool + tokio channels for sharing messages between threads), 
    ////    jobq channels, pub/sub channels to communicate with another actor and task scheduling 
    ////    and mailbox to communicate with each other under the hood.    
    
    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let unwrapped_storage = storage.unwrap(); //-- unwrapping the app storage to create a db instance
    let db_instance = unwrapped_storage.get_db().await.unwrap(); //-- getting the db inside the app storage; it might be None
    let coiniXerr_sys = SystemBuilder::new()
                                                    .name("coiniXerr")
                                                    .create()
                                                    .unwrap(); //// unwrapping the last functional method 
    info!("➔ 🟢 actor system and storage are set up");
    let mut run_time_info = RafaelRt(HashMap::new());
    let runtime_instance = run_time_info.run(); //-- run() method is the method of the Rafael serverless trait
    let arc_mutex_runtime_info_object = Arc::new(Mutex::new(runtime_instance)); //-- we can clone the runtime_instance without using Arc cause Clone trait is implemented for RafaelRt -> MetaData -> Validator actor
    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    
    // ---------------------------------------------------------------------------------------------------------------------------
    //        ZMQ P2P PUBLISHER AND SUBSCRIBER USING CAP'N PROTO SERIALIZATION (DESIGNED FOR coiniXerr NODES COMMUNICATION)
    // ---------------------------------------------------------------------------------------------------------------------------
    // https://zeromq.org/socket-api/
    
    let zmq_ctx = zmq::Context::new(); 
    let publisher = zmq_ctx.socket(zmq::XPUB).unwrap(); //// the publisher actor node
    let subscriber = zmq_ctx.socket(zmq::XSUB).unwrap(); //// the subscriber actor node 
    let mut msg = zmq::Message::new(); //// a message is a single frame which can be any type, either received or created locally and then sent over the wire through the zmq socket

    // TODO - fix p2p nat issue with upnp and ngrok
    // TODO - use cap'n proto as the serialization protocol for transaction encoding
    // TODO - a coiniXerr node can subscribes to the new transaction topic for verifying process 
    // TODO - a new transaction coming from the walleXerr will be published to the channel with new-tx topic for verifying and mining process 
    //  ...











}