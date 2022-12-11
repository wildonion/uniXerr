






use crate::*;



//// we can't use env::var() to make a rust constant type since by
//// all vars inside the env file will be loaded at runtime into the ram
//// not at compile time also const does not only mean a constant, it means a compile-time constant, 
//// a value determined at compile-time and inscribed in the read-only memory of the program, 
//// it is not suitable for your usecase.
//
//// static means a global variable, with a lifetime that will span the entire program, 
//// it may be mutable, in which case it must be Sync to avoid concurrent modifications, 
//// a static variable must be initialized from a constant, in order to be available from the start of the program.
//
//// for statics to be mutable in rust, you need to wrap them in 
//// a Mutex to follow rust's whole thing of guaranteeing thread safety.

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