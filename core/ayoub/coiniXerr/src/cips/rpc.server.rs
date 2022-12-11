



use crate::*;







pub async fn bootstrap(storage: Option<Arc<Storage>>, env_vars: HashMap<String, String>){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////                 coiniXerr walleXerr communications using cap'n proto serialization based on RPC protocol
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

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

    // -----------------------------------------------------------------------------------------------
    //          RPC SERVER AND CLIENT USING CAP'N PROTO SERIALIZATION (DESIGNED FOR waleXerr)
    // -----------------------------------------------------------------------------------------------
    // https://github.com/capnproto/capnproto-rust/tree/master/capnp-rpc    
    //// in RPC both server and client know the exact structure of the request and response 
    //// for realtime streaming which will be defined by the cap'n proto serialization schemas.

    for worker in 0..10{ //// spawning tokio green threads for 10 workers
        tokio::spawn(async move{ //// spawning tokio worker green threadpool to solve async task
            
            //// any heavy logic here that must be shared using tokio channels inside a threadpool  
            //// ...
            
        });
    }







    


    
}