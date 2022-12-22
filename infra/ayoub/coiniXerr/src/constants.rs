


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
pub static APP_STORAGE: Lazy<Option<Arc<Storage>>> = Lazy::new(|| {
    let app_storage_future = async{
        db!{ //// this publicly has exported from the utils in here so we can access it here; db macro must be inside an async block or method since there is some async method in it
            daemon::get_env_vars().get("DB_NAME").unwrap().to_string(),
            daemon::get_env_vars().get("DB_ENGINE").unwrap().to_string(),
            daemon::get_env_vars().get("DB_HOST").unwrap().to_string(),
            daemon::get_env_vars().get("DB_PORT").unwrap().to_string(),
            daemon::get_env_vars().get("DB_USERNAME").unwrap().to_string(),
            daemon::get_env_vars().get("DB_PASSWORD").unwrap().to_string()
        }    
    };
    block_on(app_storage_future)
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