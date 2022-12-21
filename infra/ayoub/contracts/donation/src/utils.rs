/*
Coded by
 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  
*/


 
    

use std::{mem::size_of, fmt::format};
use crate::*;  // loading all defined crates, structs and functions from the root crate which is lib.rs in our case













// ------------------------------ example of near actor design pattern
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// https://docs.near.org/docs/tutorials/contracts/xcc-rust
// 
// 
// NOTE - since every transaction is a promise or a future object thus they must communicate 
//        with each other using the id of the contract actor account to pass async messages.
// -------------------------------------------------------------------------------------------------
pub fn actor_ds_ex(){
    
    

    // creating a new promise (future object) ActionReceipt from accountA.testnet account_id which will 
    // create a new empty promise (future object) ActionReceipt (async message) to pass it between 
    // contract actor through mpsc channel using actor address which is the 
    // hash of the account_id (accountA.testnet) in here
    let scheduling_promise_in_account = "accountA.testnet".parse().unwrap(); //-- building the account_id from the &str
    let promise_id = env::promise_batch_create(&scheduling_promise_in_account); //-- a u64 bits or 8 bytes id which could be a pointer to the address of the promise
    
    


    env::promise_batch_action_function_call( //-- filling the created promise (future object) ActionReceipt with a transaction like calling the ft_balance_of() method of the current contract actor which is accountA.testnet account
        promise_id, //-- this is the id of the created promise which contains an empty promise (future object) ActionReceipt 
        "ft_balance_of", //-- calling ft_balance_of() method of the current contract actor which is accountA.testnet
        &serde_json::json!({"account_id": "accountB.testnet".to_string()}).to_string().into_bytes(), //-- data to be passed to the ft_balance_of() method in form of utf8 bytes
        0, //-- amount of yocto Ⓝ (1e24) to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        Gas(5_000_000_000_000) //-- gas fee to attach
    );
    


    
    // the following is a callback promise (future object) ActionReceipt to receive the DataReceipt of the promise_id (the first promise) 
    // the ActionReceipt of this promise is dependent on the previous promise (future object) ActionReceipt whenever it gets solved we'll 
    // have the DataReceipt inside the following created promise (future object) ActionReceipt
    let current_account_id = env::current_account_id().as_str().parse().unwrap();
    let callback_promise_id = env::promise_batch_then( //-- creating the second promise which also will create an empty ActionReceipt to fulfill the callback promise with the incoming message or receipt which contains the data from the first promise (future object) ActionReceipt
        promise_id, //-- this is the id of the first promise (future object) ActionReceipt which contains the DataReceipt either pending, postponed or solved coming from the first promise (future object) ActionReceipt
        &current_account_id, //-- the current_account_id() which is the one who owns this contract (accountA.testnet) is the receiver of this created promise (future object) ActionReceipt    
    );
    



    // attacing a callback function to the callback promise (future object) ActionReceipt
    env::promise_batch_action_function_call(
        callback_promise_id, //-- this is the id of the second promise (future object) ActionReceipt which contains the DataReceipt from the first promise (future object) ActionReceipt
        "my_callback", //-- the callback function which must be call after fulfilling the promise with the DataReceipt coming from the first promise (future object) ActionReceipt
        b"{}", //-- data to be passed to the my_callback() method in form of utf8 bytes
        0, //-- amount of yocto Ⓝ (1e24) to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        Gas(5_000_000_000_000) //-- gas fee to attach
    );



    env::promise_return(callback_promise_id) //-- returning the solved DataReceipt of the callback promise 
    
    
}



// ------------------------------ internal functions 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub fn panic_not_self(){
    if env::current_account_id() != env::predecessor_account_id(){ //-- current_account_id() is the one who owns this contract - the owner (or the signer of the contract if it's not a cross contract call) is not the previous contract actor account or the last (current) caller of this method
        env::panic_str("last caller is not the owner of this contract"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn panic_one_yocto(){
    if env::attached_deposit() != 1{
        env::panic_str("Requires attached deposit of exactly 1 yocto Ⓝ (1e24)"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn panic_atleast_one_yocto(){
    if env::attached_deposit() < 1{
        env::panic_str("Requires attached deposit of at least 1 yocto Ⓝ (1e24)"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unkown size at compile time we must borrow it by taking a pointer to its location
    }
}


pub fn hash_account_id(account_id: &AccountId) -> CryptoHash{ //-- we'll pass the account_id as a borrowed type to this function - account_id in CryptoHash format is a 32 bytes or 256 bits which will be 64 chars in hex
    let mut hash = CryptoHash::default(); //-- getting the default hash which will be 32 elements of utf8 bytes (8 bits or 1 byte long for each)
    hash.copy_from_slice(&env::sha256(account_id.as_bytes())); //-- extending the defined hash with the borrowed type of the bytes of the hash of the account_id by converting its String into utf8 bytes first; the source or the length of the hash of the account_id bytes must be the same as the defined hash variable 
    hash
}


pub fn bytes_for_account_id(account_id: &AccountId) -> u64{ //-- calculating the exact amount of bytes of the passed in account_id in u64 bits or 8 bytes maximum (usize on 64 bits arch system)
    (
        account_id.as_str().len() as u64 //-- getting the length (in bytes) of the &str of the account_id String (as u64 cause the default is usize) which is equals to its length of utf8 bytes array (&[u8])
        + 4 //-- adding 4 bytes to the length of the &str - this 4 bytes or 32 bits is added by Borsh serialization to store the length of the string when deserializing it into String from its utf8 bytes
        + size_of::<u64>() as u64 //-- adding the total size of the u64 bits in bytes as u64 bits or 8 bytes maximum
    ) as u64 //-- casting all calculated bytes into u64 bits which will be something between 0 and 18446744073709551615 (2^64 − 1) cause the type of storage in near protocol is u64 bits or 8 bytes maximum (usize on 64 bits arch system)
}
