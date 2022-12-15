





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

            https://www.near-sdk.io/contract-structure/collections ➔ Near Sdk Collection Performence


*/








use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itself cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128, U64}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ 
                serde_json,
                promise_result_as_success, //-- returns the result of the promise if successful, otherwise returns None
                env::STORAGE_PRICE_PER_BYTE, //-- loading the price of each byte in yocto Ⓝ (1e24)
                Gas, ext_contract, PromiseResult, env, near_bindgen, assert_one_yocto, //-- we're using the assert_one_yocto() function from the near_sdk cause it's using the env::panic_str() one the background 
                AccountId, Balance, CryptoHash, Promise, //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
                PanicOnDefault, PromiseOrValue, BorshStorageKey //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
            };


pub mod utils;






// NOTE - we have to make money from callers since updating structs cost money and we have to force the caller to deposit some amount to cover the updating cost in our contract thus we must not to spend from our moeny to do this and we have to make the method payable
// NOTE - HashMap keeps all data in memory, to access it, the contract needs to deserialize the whole map and it deserializes (and serializes) the entire collection in one storage operation; accessing the entire collection is cheaper in gas than accessing all elements through N storage operations
// NOTE - try to validate the input, context, state and access using require! before taking any actions; the earlier you panic, the more gas you will save for the caller
// NOTE - borsh is used for internal STATE serialization and serde for external JSON serialization
// NOTE - `Donation` struct contains some data structures to store on chain infos about tokens and their owners at runtime
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - all payable methods needs to deposit some yocto Ⓝ (1e24) since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain to minimize the amount of gas and the SDK collections should be used in most cases to reduce the gas fee since these collections deserialize the exact data that we need it instead of deserializing all entries each time the state and the app runtime is loaded like HashMap
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 
// NOTE - in private methods current_account_id(), predecessor_account_id() and signer_account_id() are the same an is the contract actor account owner itself






#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Donation{
    pub owner_id: AccountId,
}



#[near_bindgen] //// implementing the #[near_bindgen] proc macro attribute on `Donation` struct to compile all its methods to wasm so we can call them in near cli
impl Donation{

    #[init] //// means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    //// initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with default metadata so the user don't have to manually type metadata
    pub fn new(owner_id: AccountId) -> Self{ //// owner_id is the one who initialized this method

        Self{
            owner_id,
        }


    }


}