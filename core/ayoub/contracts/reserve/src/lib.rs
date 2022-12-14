





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
use near_sdk::serde_json::json;
use near_sdk::{ 
                serde_json,
                promise_result_as_success, //-- returns the result of the promise if successful, otherwise returns None
                env::STORAGE_PRICE_PER_BYTE, //-- loading the price of each byte in yocto Ⓝ (1e24)
                Gas, ext_contract, PromiseResult, env, near_bindgen, assert_one_yocto, //-- we're using the assert_one_yocto() function from the near_sdk cause it's using the env::panic_str() one the background 
                AccountId, Balance, CryptoHash, Promise, //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
                PanicOnDefault, PromiseOrValue, BorshStorageKey //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
            }; 






            



// https://github.com/Bitdad-Dev/bitzio-nft-near-contracts




#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Conse{
    pub owner_id: AccountId,
}



#[near_bindgen]
impl Conse{

    #[init]
    pub fn new(owner_id: AccountId) -> Self{ //// owner_id is the one who initialized this method

        Self{
            owner_id,
        }


    }


}