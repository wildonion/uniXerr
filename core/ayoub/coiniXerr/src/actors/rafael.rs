















pub mod env{ //-- rafael env which contains runtime functions and actors to mutate the state of the runtime object like near-sdk env


    
    
    // load wasm file => https://procmarco.netlify.app/blog/2021-05-04-a-story-about-async-rust-and-using-send-types/
    // TODO - use some kinda register setup process to get and mutate the vars of the env like near registers in its env module for promises or futures
    // TODO - a register contains the current buffer inside the ram related to the passed in id means we have to read the buffer from inside of it related to the passed in id
    // TODO - we have to read the content of a specific register and save it inside a buffer
    // TODO - rafael serverless runtime must be like #[rafael::main] on top of a server instance  


    // near, cloudflare and shuttle are serverless:
    //      - write contract or serverless methods in rust then compile to wasm32-wasi
    //      - deploy using cli to the runtime which will balance requests using a high performence proxy like pingora, k8s and krustlet 
    //      - load the deployed file in js and call its methods



    pub use crate::*;
    pub use std::{fmt, env, sync::{Arc, Mutex}};
    pub use borsh::{BorshSerialize, BorshDeserialize};
    pub use uuid::Uuid;
    pub use std::net::SocketAddr;
    pub use serde::{Serialize, Deserialize};
    use futures::channel::mpsc as future_mpsc;
    use tokio::sync::mpsc as tokio_mpsc;
    use std::{sync::mpsc as std_mpsc, time::Duration};
    use futures::join as futures_join;
    use futures_util::join as futures_util_join;
    use tokio::join as tokio_join;
    use rayon::join as rayon_join;










    
    


    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    //                RAFAEL DATA STRUCTURES
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag="event", content="data")] //-- the deserialized data of the following enum  will be : {"event": "runtime", "data": [{...RuntimeLog_instance...}, {...ServerlessLog_instance...}]} or {"event": "serverless", "data": [{...ServerlessLog_instance...}, {...ServerlessLog_instance...}]}
    #[serde(rename_all="snake_case")] //-- will convert all fields into snake_case
    pub enum EventVariant{
        Runime(Vec<RuntimeLog>),
        Serverless(Vec<ServerlessLog>),
    }



    #[derive(Serialize, Deserialize, Debug)]
    pub struct EventLog{ //-- an interface to capture the data about and event - this is the EVENT_JSON
        pub time: Option<i64>, //-- the time of the event data log
        #[serde(flatten)] //-- flatten to not have "event": {<EventVariant>} in the JSON, just have the contents of {<EventVariant>} which is the value of the data key itself - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the number of exact fields inside the struct or enum or what's exactly inside the body of an api comming from the client to decode or map it into the struct or enum thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
        pub event: EventVariant, //-- the data which is a vector of all either Serverless or Runime variant events - we'll have {"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]}
    }



    impl fmt::Display for EventLog{ //-- implementing the Display trait for the EventLog struct to show its instances' fields like RAFAEL_EVENT_JSON:{"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]} when we're calling logging functions like println!() which is a formatted stream of strings - any value or type that implements the Display trait can be passed to format_args!() macro, as can any Debug implementation be passed to a {:?} within the formatting string; Debug must be implemented for the type
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{ //-- self referes to the instance of the EventLog 
            f.write_fmt( //-- writing some formatted information using format_args!() macro into the formatter instance which is `f`
                format_args!( //-- format_args!(), unlike its derived macros, avoids heap allocations
                    "RAFAEL_EVENT_JSON:{}", //-- it'll start with RAFAEL_EVENT_JSON:{} when you log the instance of the EventLog
                    &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() //-- formatting every field of the self which is the instance of the EventLog struct into the string to writ into the `f` and catch the fmt::error of each message or field if there was any when we're creating the stream by formatting the struct
                ) 
            ) // NOTE - we can print the string instance of the EventLog like so: println!("{:?}", event_log_instance.to_string()); since the Display trait is implemented for EventLog struct
        }
    }




    #[derive(Serialize, Deserialize, Clone, Debug)] // NOTE - Copy trait is not implemented for Box-ed types since the Box is a smart pointer to a heap allocated type and heap types have unknown size at compile time since they're not bounded to Sized trait
    pub struct RuntimeLog{ // TODO - initialize this inside the main() function
        pub id: String, //-- since serde traits is not satisfied for Uuid we've used the stringified of the Uuid as the id 
        pub path: String, //-- the path of the log file in server
        #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
        pub requested_at: Option<i64>, //-- the time of the log request
        pub content: Box<[u8]>, //-- the array of utf8 bytes contains the content of the log inside the Box
    }



    #[derive(Serialize, Deserialize, Clone, Debug)] // NOTE - Copy trait is not implemented for Box-ed types since the Box is a smart pointer to a heap allocated type and heap types have unknown size at compile time since they're not bounded to Sized trait
    pub struct ServerlessLog{ // TODO - initialize this inside the main() function
        pub id: String, //-- since serde traits is not satisfied for Uuid we've used the stringified of the Uuid as the id 
        pub path: String, //-- the path of the log file in server
        pub method: String, //-- the method name that the log data is captured for
        #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
        pub requested_at: Option<i64>, //-- the time of the log request
        pub content: Box<[u8]>, //-- the array of utf8 bytes contains the content of the log inside the Box
    }
    
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct LinkToService(pub String); // NOTE - LinkToService contains the address of the socket service located inside the memory with usize as its size, u64 bits or 8 bytes or 32 btis or 4 bytes (based on arch)
    


    #[derive(Serialize, Deserialize, Copy, Clone, Debug)] // TODO - use error derive proc macro attributes on the following enum fields
    pub enum AppError{ //-- enum like union shares a common memory location between all its fields that means the space an enum needs is as much as the largest variant but unlike union the enum uses some extra memory to keep track of the enum variant which is called tag and is a pointer with 8 bytes length or 64 bits 
        OnRuntime, //-- caused by too much loading and requests
        OnStorage, //-- caused by storage services errors 
    }
    


    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    pub enum Service{
        Stake,
        Deposit,
        Withdraw,
    }

    

    
    #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
    pub enum FutureResult{
        Successful(Vec<u8>), //-- the successful result of the future object in form utf8 bytes
        Pending, //-- future is not ready
        Failed, //-- the fail result of the future object 
    }


    
    #[derive(Clone, Debug)]
    pub struct MetaData{ 
        pub id: Uuid,
        pub actor: ActorRef<<Validator as Actor>::Msg>, //-- validator actor with the mailbox of type Msg; aslo Validator actor should implements the Debug and Clone trait also
        pub link_to_server: Option<LinkToService>, //-- we've just saved the location address of the socket service inside the memory
        pub error: Option<AppError>, //-- any runtime error caused either by the runtime itself or the storage crash
        pub node_addr: Option<SocketAddr>, //-- socket address of this node
        pub last_crash: Option<i64>, //-- last crash timestamp
        pub first_init: Option<i64>, //-- first initialization timestamp 
    }

    impl MetaData{
        pub fn update_validator_transaction(&mut self, transaction: Option<Transaction>){ //-- updating the recent_transaction field of the validator actor is done using a mutable borrower (pointer) as the parameter of the update_validator_transaction() method 
            self.actor.tell(UpdateTx{id: Uuid::new_v4(), tx: transaction}, None); //-- telling the validator actor that we want to update the last transaction of this validator
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct Runtime(pub HashMap<Uuid, MetaData>); //-- MetaData struct should implements the Debug and Clone trait also

    impl Runtime{

        pub fn new() -> Self{
            Runtime(HashMap::new())
        }
    
        pub fn add(mut rti: Self, meta_data: self::MetaData) -> Uuid{ //-- &rti means borrowing the ownership of all Runtime fields - it must be mutable cause we want to insert into its hash map field
            let generated_uuid = Uuid::new_v4();
            rti.0.insert(generated_uuid, meta_data);
            generated_uuid
        }
    }



    
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    //                      RAFAEL ACTOR
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡

    impl Actor for Runtime{

        type Msg = Vec<u8>; 

        fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender){ //// ctx is the actor system which we can build child actors with it also sender is another actor 

            todo!();        

        }

    }




    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    //              RAFAEL SERVERLESS METHODS
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    pub trait Serverless{ /////// a functional Serverless trait for Runtimes - this trait is not object safe trait since we're returning the self and Self in method param and the returning signature 

        type Service; //-- the service type; game, auth, nft & etc...
        type App;
        type Cost; //-- current storage cost of runtime socket calls or the total cost of the serverless trait method calls during an especific period of time based on amount of CPU, network, and IO, and the amount of data already stored in runtime storage which is the VPS ram and has determined by the load balancer


        ////////////////////////////////////////////////////////////////////////////////
        ///// FOLLOWING METHODS MIGHT BE CALLED MORE THAN 1000 TIMES PER SECOND BY USERS 
        ///// THUS WE HAVE CODE THEM AS EFFICIENT AS POSSIBLE.
        ////////////////////////////////////////////////////////////////////////////////


        fn run(&mut self) -> Self; // NOTE - the type that this trait which must be implemented for must be defined as mutable - the return type is the type that this trait will be implemented for
        fn stop(&mut self) -> Self; 
        fn build(&mut self) -> Self; //-- building a wasm file from the rust code (yew and wasi)
        fn schedule(&self) -> Self; 
        fn callback(&self) -> Self;
        fn refund(&mut self) -> Self; //-- &mut self is because we want to mutate the state if the runtime by refunding an account
        fn deposit(&mut self) -> Self; //-- &mut self is because we want to mutate the state if the runtime by adding some amount to an account
        fn storage_usage(&self) -> u64; //-- no need to add &mut self for the first param since we want to return the total used bytes 
        fn storage_byte_cost(&self) -> Self::Cost; //-- no need to add &mut self for the first param since we want to return the total cost of the total used bytes
        fn current_timestamp(&self) -> u64; //-- current runtime timestamp in nanoseconds
        fn init(&self) -> Self::App; //-- initialize the whole app for the first time; this method will panic on second call
        fn health(&self) -> Self;
        fn caller(&self) -> SocketAddr; //-- the current caller of one of the Serverless trait methods
        fn future_result(&self, idx: u64) -> FutureResult; //-- getting the result of the passed in future id
        fn make_tx(&mut self) -> schemas::Transaction;

    }



    impl Serverless for Runtime{

        type Service = Service;  
        type App     = String; 
        type Cost    = u128; 

        fn run(&mut self) -> Self{ //-- the first param is a shared mutable pointer to the instance of the runtime 
            Self::new()
        }

        fn refund(&mut self) -> Self{

            todo!()

        }

        fn deposit(&mut self) -> Self{

            todo!()

        }

        fn build(&mut self) -> Self {
            
            // TODO - build the walleXerr app using build.sh script  
            // ...
            
            todo!()
        
        }

        fn stop(&mut self) -> Self{
            Runtime(HashMap::new()) //-- returning an empty runtime and cleanup everything
        }

        fn schedule(&self) -> Self{

            // TODO - use multithreading, channels (oneshot and mpsc) and cryptography algos and also types must be send + sync + static across threads and .awaits
            // TODO - use Arc<Mutex<T>> (use Arc::new(&Arc<Mutex<T>>) to clone the arced and mutexed T which T can also be Receiver<T>) in multithreaded env and RefCell<Rc<T>> in single threaded env
            // TODO - actors will send encoded data through the mpsc channel from their free thread, so we have to deserialize them when we resolve them outside of the fulfilled future object 
            // TODO - every receipt is a transaction with a specific id which will be created by scheduling an ActionReceipt 
            // TODO - scheduling a promise of future object contains the method call (ActionReceipt) and get the resolved of the pending DataReceipt object from the executed future object inside a callback inside where we've scheduled the call
            // TODO - try different IO streaming and future traits on a defined buffer from the following crates like mpsc and Mutex data structures
            // TODO - consider every service a shard which can communicate (like executing each other's methods asyncly) with each other using their actors which has been implemented for each service through mpsc channels  
            // TODO - scheduling an event which is a future object contains an async message like calling one of the method of the second service to be executed and triggered inside the second service and get the response inside a callback method using .then()
            // TODO - incoming scheduled event from a thread of the first service actor inside a free thread of the second service actor must be of type Arc<Mutex<T>> (use Arc::new(&Arc<Mutex<T>>) to clone the arced and mutexed T which T can also be Receiver<T>) in order to avoid data races and dead locks 
            // TODO - sending async message from the socket service to another serivce using actor that has been implemented for each service
            // TODO - vector of || async move{} of events for an event manager struct like riker scheduling logic and vector clock schemas and call new event every 5 seconds from vector of event of closures 
            // TODO - build a protocol based on binary address to transmit data between actors using mpsc tunnel like onionary://0001010101:2349
            // TODO - use functional programming design pattern to call nested method on a return type of a struct method: events.iter().skip().take().map().collect()
            // ...
            /*
                let message = Arc::new( //-- we can send this message asyncly between each services actors threads using mpsc channel since mpsc means multiple thread can access the Arc<Mutex<T>> (use Arc::new(&Arc<Mutex<T>>) to clone the arced and mutexed T which T can also be Receiver<T>) but only one of them can mutate the T out of the Arc by locking on the Mutex
                    Mutex::new(
                            utils::Storagekey::ByNFTContractIdInner{ 
                                account_id_hash: [23, 24] 
                            }
                        )
                    );
                let resp = Schedule::on(service_address)
                                .data(message) //-- this is the data that must be executed on second service and it can be the name of a method inside that service 
                                .run_in_parallel()
                                .then(self.callback()); //-- wait to solve the future
                NOTE - scheduling a promise object which will call the built-in method of the near protocol the transfer() method which will be executed later asyncly to transfer Ⓝ to the creator contract acctor account
                let resp = Schedule::on(service_address) //-- scheduling a future object in here on another service which must gets executed later asyncly to run the scheduled method which in our case is the transfer() method
                                .transfer(3) //-- this is the amount that must gets transferred to the second service
                                .run_in_parallel()
                                .then(self.callback()); //-- wait to solve the future
                NOTE - calling between two wasm files (since every wasm file is a service) is done like the following since every wasm file is an actor which can send message through mpsc channel to another actor or wasm file or service based on their unique address
                let resp = self.current_service.send(msg).to(another_serivce).await; //-- msg must be a json stringified in form "{ \"key\": \"value\" }" like "{ \"storage_cost\": \"5\" }" which can be decoded in destination service 
            */

            todo!()

        }

        fn callback(&self) -> Self{
            

            // TODO - a callback method to get the response of the executed event in a specific service actor
            // ... 

            
            // -------------
            // if let syntax
            // -------------
            let fut_res = if let FutureResult::Successful(encoded_result) = self.future_result(0){ //-- getting the result of the future object only if it was successful
                // TODO - deserialize the result of the executed future object into a pre defined structure
                // ... 
            } else if let FutureResult::Failed = self.future_result(0){
                
            } else{

            };


            // -------------
            // match pattern
            // -------------
            match self.future_result(0){
                FutureResult::Successful(data) => {
                    
                },
                FutureResult::Failed => {

                },
                _ => { //-- if it was Pending

                },
            }

            todo!()

        }

        fn storage_usage(&self) -> u64 {

            // TODO - the total bytes in u64 bits (8 bytes) format used by the runtime
            // ...

            todo!()

        }

        fn storage_byte_cost(&self) -> Self::Cost {
            
            // TODO - the cost of the total used storage by the runtime
            // ...

            todo!()

        }

        fn current_timestamp(&self) -> u64 {
            
            // TODO - get the current timestamp of the runtime in nanoseconds
            // ...

            todo!()
        }

        fn init(&self) -> Self::App {
            
            // TODO - initialize the whole app state inside n threads for the first time and will panic on second call (need a flag to be set on one of the env register to check the state)
            // ...

            todo!()
        }

        fn health(&self) -> Self {
            
            // TODO - check the healthiness of the serverless runtime
            // ...

            todo!()

        }
        
        fn caller(&self) -> SocketAddr{
        
            // TODO - return the socket address of the caller of a method for handling method call costs
            // ...
    
            todo!()
        }

        fn future_result(&self, idx: u64) -> FutureResult{
        
            // TODO - 
            // ...
            // match super::env::future_get_result_of(idx){ // TODO - future_get_result_of() function must return Result<FutureResult, FutureError>
            //     Err(FutureResult::Pending) => FutureResult::Pending,
            //     Err(FutureResult::Failed) => FutureResult::Failed,
            //     Ok(()) => {
            //         let data = super::env::expect_register(read_register(ATOMIC_OP_REGISTER));
            //         FutureResult::Successful(data)
            //     } 
            // }
            
            todo!()
        }

        fn make_tx(&mut self) -> schemas::Transaction{
            
            
            // TODO -
            // make a transaction from the runtime socket calls with their cost
            // ...
            
            todo!()

        }

    }



}
