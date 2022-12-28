


use crate::*;






//// Lazy is just like lazy_static! macro 
//// which is a thread safe structure
//// that we can create static type.
//// Lazy also works with local variable
pub static KEYS: Lazy<Keypair> = Lazy::new(identity::Keypair::generate_ed25519); //// generating a thread safe static keypair (public and private)
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public())); //// generating a thread safe peer id from the generated keypair
pub static PARACHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("parachains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
pub static NETWORK_STAT: Lazy<Topic> = Lazy::new(|| Topic::new("netstat")); //// this is the topic about network status and updates

/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
///////           app storage setup
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
pub static APP_STORAGE: Lazy<Option<Arc<Storage>>> = Lazy::new(|| { //// the new() method takes a closure so we can pass a function which returns a type that must be staticly initialized
    block_on( //// despite the tokio runtime methods the block_on() will block the current thread to complete the future 
        async{ //// since we're using mongodb async lib we need to initialize it inside an async block or method thus in here we have to initialize the db inside an async block 
            db!{ //// this publicly has exported from the utils in here so we can access it here; db macro must be inside an async block or method since there is some async method in it
                daemon::get_env_vars().get("DB_NAME").unwrap().to_string(),
                daemon::get_env_vars().get("DB_ENGINE").unwrap().to_string(),
                daemon::get_env_vars().get("DB_HOST").unwrap().to_string(),
                daemon::get_env_vars().get("DB_PORT").unwrap().to_string(),
                daemon::get_env_vars().get("DB_USERNAME").unwrap().to_string(),
                daemon::get_env_vars().get("DB_PASSWORD").unwrap().to_string()
            }    
        }
    ) //// blocking the current thread to solve the future object
});

/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
///////    mempool channel initialization
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//// mempool channel is broadcast job queue channel which 
//// all transactions must be sent through this channel for mining process.
//// to follow Rust's whole thing of guaranteeing thread safety for mutation 
//// we need to wrap our data in a Mutex and also the data must be Send and Sync.
//
//// Clone trait is not implemented for receiver thus
//// the Copy trait can't be implemented also since 
//// Clone is a supertrait of Copy and because of this
//// we can't move out of MEMPOOL_CHANNEL by deferencing it at all!
//
//// shared reference is being used by other threads and scopes 
//// and they must be dereferenced either by cloning or *; 
//// also it can't be dereferenced it the Clone trait is not 
//// implemented for that since Clone is a supertrait of the Copy.

pub static MEMPOOL_CHANNEL
            : 
            Lazy<(
                tokio::sync::broadcast::Sender<(Arc<Mutex<Transaction>>, Arc<Mutex<ActorRef<ValidatorMsg>>>, ActorSystem)>, 
                tokio::sync::broadcast::Receiver<(Arc<Mutex<Transaction>>, Arc<Mutex<ActorRef<ValidatorMsg>>>, ActorSystem)>
            )> = 
                Lazy::new(|| {
                    broadcast::channel::<(
                        Arc<Mutex<Transaction>>, 
                        Arc<Mutex<ActorRef<<Validator as Actor>::Msg>>>, //// we're getting the mailbox type of Validator actor first by casting it into an Actor then getting its Msg mailbox which is of type ValidatorMsg  
                        //// passing the coiniXerr actor system through the broadcast channel since tokio::spawn(async move{}) inside the loop will move all vars, everything from its behind to the new scope and takes the ownership of them in first iteration and it'll gets stucked inside the second iteration since there is no var outside the loop so we can use it! hence we have to pass the var through the channel to have it inside every iteration of the `waiting-on-channel-process` loop
                        //// no need to put ActorSystem inside the Arc since it's bounded to Clone trait itself and also we don't want to change it thus there is no Mutex guard is needed
                        ActorSystem 
                        //// there is no need to pass other actor channels through mempool channel since there is no tokio::spawn(async move{}) thus all the vars won't be moved and we can access them in second iteration of the loop
                    )>(daemon::get_env_vars().get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap()) //-- transaction mempool channel using broadcast channel to send all transactions of all peers' stream plus the related validator actor info to down side of the channel asynchronously for mining process - buffer_size is the number of total bytes we can send and have through and inside the channel
                });






pub type MainResult<T, E> = std::result::Result<T, E>;
pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type GenericResult<T, E> = std::result::Result<T, E>;
pub const STORAGE_COST: u128 = 3;
pub const COMPUTATIONAL_COST: u128 = 2; 
pub const VALIDATOR_REWARD_COST: u128 = 4;
pub const MESSAGE_FETCHED_SUCCESS: &str = "Fetched successfully";
pub const MESSAGE_SPEND_SUCCESS: &str = "Spend successfully";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";
pub const MESSAGE_ACCESS_DENIED: &str = "Access Denied";
pub const MESSAGE_NOT_ENOUGH_COINS: &str = "Not enough coins";
pub const AUTHORIZATION: &str = "Authorization";
pub const EMPTY: &str = "";
pub const NEW_TRANSACTION_TOPIC: &str = "new transaction";
pub const VERIFYING_TRANSACTION_TOPIC: &str = "verifyin new transaction";