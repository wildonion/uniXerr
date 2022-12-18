


use crate::*;


//// Lazy is just like lazy_static! macro 
//// which is a thread safe structure
//// that we can create static type.
//// Lazy also works with local variable
pub static KEYS: Lazy<Keypair> = Lazy::new(identity::Keypair::generate_ed25519); //// generating a thread safe static keypair (public and private)
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public())); //// generating a thread safe peer id from the generated keypair
pub static PARACHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("parachains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
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