




// NOTE - we've put T inside the Option cause T might be None at initializing stage or a dangling pointer on later changes 


use std::rc::{Rc, Weak};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::net::SocketAddr; //-- these structures are not async; to be async in reading and writing from and to socket we must use tokio::net 
use actix::{*, prelude::*}; //-- loading actix actors and handlers for threads communication using their address and defined events 
use crate::actors::peer::Validator;
use std::collections::HashMap;
use rand::Rng;









#[cfg(test)]
pub mod tests{

    #[test]
    fn staking(){
        
    }
}









// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                        Stake Info Schema                      
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct Staker{
    pub id: Uuid,
    pub deposit: i32,
    pub owner: Validator, //-- delegator or owner or staker is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------














// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                        Vote Info Schema                      
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct Voter{
    pub parachain_id: Uuid, //-- voter will vote in this parachain using delegator stakes
    pub owner: Validator, //-- owner is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
    pub staker_id: Uuid, //-- delegator id who staked his/her money for this voter
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------
















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                  Parachain Slot Schema                      
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct Slot{ //-- pool of validators for slot auctions
    pub id: Uuid,
    pub name: String,
    pub validators: Vec<Voter>, //-- auction voters for this slot
    pub epoch: u32, //-- number of created blocks to generate new slot auction process 
} 
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------
















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                     RuntimeInfo Schema 
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct RuntimeInfo(pub HashMap<Uuid, MetaData>); //-- MetaData struct should implements the Debug and Clone trait also

impl RuntimeInfo{

    pub fn new() -> Self{
        RuntimeInfo(HashMap::new()) //-- unit like struct
    }

    pub fn add(mut rti: Self, meta_data: self::MetaData) -> Uuid{ //-- &rti means borrowing the ownership of all RuntimeInfo fields - it must be mutable cause we want to insert into its hash map field
        let generated_uuid = Uuid::new_v4();
        rti.0.insert(generated_uuid, meta_data);
        generated_uuid
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                       MetaData Schema 
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)] 
pub struct MetaData{
    pub address: SocketAddr,
    pub actor: Validator, //-- Validator actor should implements the Debug and Clone trait also
}

impl MetaData{
    pub fn update_validator_transaction(&mut self, transaction: Option<Transaction>){ //-- updating the recent_transaction field of the validator actor is done using a mutable borrower (pointer) as the parameter of the update_validator_transaction() method 
        self.actor.recent_transaction = transaction;
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                         Chain Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Serialize, Deserialize, Clone, Debug)] //-- encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Chain{
    pub branch_id: Uuid, //-- chain id
    pub branch_name: String,
    pub blocks: Vec<Block>,
}

impl Chain{
    
    pub fn default() -> Self{
        Chain{
            branch_id: Uuid::new_v4(),
            branch_name: format!("cc-{}", rand::thread_rng().gen::<u32>().to_string()),
            blocks: vec![Block::default()],
        }
    }

    pub fn new(branch_id: Uuid, branch_name: String, blocks: Vec<Block>) -> Self{ //-- constructor of Chain struct - creating another branch or fork
        Chain{
            branch_id,
            branch_name,
            blocks,
        }
    }
    
    pub fn add(&mut self, block: Block) -> Self{ //-- the first param is a mutable pointer to every field of the struct - self takes a copy of all fields and &mut borrow the ownership of those fields for mutating them
        self.blocks.push(block);
        Chain{
            branch_id: self.branch_id,
            branch_name: self.branch_name.clone(), //-- Copy trait is not implemented for String thus we have to clone it to return from the function
            blocks: self.blocks.clone(), //-- Copy trait is not implemented for blocks thus we have to clone it to return from the function
        }
    }

    pub fn get_genesis(&self) -> &Block{
        let genesis = &self.blocks[0];
        genesis
    }

    pub fn store(&mut self) -> Self{
        // TODO - save the whole chain state inside a db or a persistence storage like bigchain db
        // ...
        todo!();

    }

    pub fn build_raw_block(&self, prev_block: &Block) -> Block{ //-- this method get an immutable pointer to the block (borrowed) as its second argument 
        Block{
            id: Uuid::new_v4(),
            index: prev_block.clone().index + 1, //-- we have to clone the prev_block cause Block struct doesn't implement the Copy trait
            is_genesis: false,
            prev_hash: prev_block.clone().hash, //-- first block inside the chain is the genesis block - we have to clone the prev_block cause Block struct doesn't implement the Copy trait 
            hash: None, // TODO -
            merkle_root: None, // TODO - 
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![],
            is_valid: false,
        }
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------

















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                         Block Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Serialize, Deserialize, Clone, Debug)] //-- encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Block{
    pub id: Uuid,
    pub index: u32,
    pub is_genesis: bool,
    pub prev_hash: Option<String>, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
    pub hash: Option<String>, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
    pub merkle_root: Option<String>, //-- hash of all transactions in the form of a binary tree-like structure called merkle tree such that each hash is linked to its parent following a parent-child tree-like relation
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //-- valid transactions (came through mempool channel) waiting to be confirmed and signed - can't implement the Copy trait for Vec thus can't bound it to the Block structure 
    pub is_valid: bool,
}

impl Block{
    pub fn push_transaction(&mut self, transaction: Transaction) -> Self{ //-- the first param is a mutable pointer to every field of the struct
        self.transactions.push(transaction);
        Block{ //-- don't return &self when constructing the struct cause we'll face lifetime issue for struct fields - &mut T is not bounded to Copy trait due to ownership and borrowing rules which we can't have multiple mutable pointer at the same time
            id: self.id,
            index: self.index,
            is_genesis: self.is_genesis,
            prev_hash: Some(self.prev_hash.clone().unwrap()), //-- self.prev_hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            hash: Some(self.hash.clone().unwrap()), //-- self.hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            merkle_root: Some(self.clone().merkle_root.unwrap()), //-- self.merkle_root is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            timestamp: self.timestamp,
            transactions: self.transactions.clone(), //-- self.transactions is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for Vec thus we have to clone it 
            is_valid: self.is_valid,
        }
    }
}

impl Default for Block{
    fn default() -> Self{
        Block{
            id: Uuid::new_v4(),
            index: 0,
            is_genesis: true,
            prev_hash: Some("prev block hash here".to_string()), // TODO -
            hash: Some("current block hash here".to_string()), // TODO -
            merkle_root: Some("merkle root hash here".to_string()), // TODO - 
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![Transaction::default()],
            is_valid: true,
        }
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------













// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                      Merkle Tree Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug)]
pub struct Node{
    pub id: Uuid,
    pub data: Transaction,
    pub parent: Weak<Node>, //-- child -> parent using Weak to break the cycle, counting immutable none owning references to parent - weak pointer or none owning reference to a parent cause deleting the child shouldn't delete the parent node
    pub children: Vec<Rc<Node>>, //-- parent -> child, counting immutable references or borrowers to childlren - strong pointer to all children cause every child has a parent which the parent owns multiple node as its children and once we remove it all its children must be removed
}

impl Node{

    pub fn is_leaf(&mut self) -> bool{
        todo!();
    }

    pub fn add_child(&mut self, node: Node){
        self.children.push(Rc::new(node));
    }

    pub fn children(&mut self, node: Node) -> Result<Vec<Rc<Self>>, String>{ //-- &mut self means we're borrowing Node fields using a mutable pointer which is a shallow copy of the fields (if we change the pointer value the actual object will be changed) for updaing the desired field
        if node.children.len() != 0{
            Ok(node.children)
        } else{
            Err(format!("node -{}- has no children", node.id).to_string())
        }
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                        Transaction Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// NOTE - all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
// NOTE - there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
// NOTE - enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself
unsafe impl Send for TransactionMem {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads
union TransactionMem{
    pub data: *mut self::Transaction, //-- defining the data as a raw mutable pointer to a mutable Transaction object, changing the data will change the Transaction object and vice versa
    pub buffer: *const u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)] //-- encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Transaction{
    pub id: Uuid,
    pub ttype: u8, //-- 00000000 or 0x00 is one byte - every 4 bits in one byte is a hex number so 8 bits is 2 hex number in one byte representation bits and every 3 digit in one byte is a oct number
    pub amount: i32,
    pub from_address: String, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
    pub to_address: String, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
    pub issued: i64,
    pub signed: Option<i64>,
    pub signature: Option<String>, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
    pub hash: String, //-- 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
}

impl Default for Transaction{
    fn default() -> Self{
        Transaction{
            id: Uuid::new_v4(),
            ttype: 0x00, //-- 0x00 means 0 in hex and a regular transaction - 0xFF or 1 (CRC21) and 0x02 or 2 (CRC20) and 0x03 or 3 (CRC22) in hex means smart contract transaction
            amount: 100,
            from_address: "the address of coiniXerr network wallet".to_string(), // TODO - the address of the coiniXerr network - public key is used to generate wallet address
            to_address: "the address of wildonion wallet network".to_string(), // TODO - the address of the wildonion wallet - public key is used to generate wallet address
            issued: chrono::Local::now().naive_local().timestamp(),
            signed: Some(chrono::Local::now().naive_local().timestamp()),
            signature: Some("signature hash of the transaction signed with sender's private key".to_string()), // TODO - transaction object needs to be signed using the sender's private key and this cryptographically proves that the transaction could only have come from the sender and was not sent fraudulently
            hash: "hash of the current transaction".to_string(), // TODO -
        }
    }
}

impl Transaction{ //-- a transaction decoder or deserializer using union
    pub fn new(buffer: &[u8]) -> Result<&mut Self, Box<dyn std::error::Error>>{ //-- self is a copy to all values of the struct; &self is a pointer to those values means by doing this we will borrow ownership of all original values
        unsafe{ // NOTE - if neither Copy nor Clone is not implemented for the object by moving it into a function we loose the ownership of the value of that object; we can borrow the ownership by taking a pointer to it using &
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //-- filling the buffer field will also fill the data cause thay have a same memory storage
            let deserialized_transaction = &mut *transaction.data; //-- mutable pointer to the dereferenced transaction data - since the data inside the union is a raw pointer to a mutable Transaction object we have to dereference it to return a Transaction object; we also want to change the object later so we have to take a mutable pointer or reference (&mut) to the dereferenced object to borrow the ownership of the original object for mutation
            Ok(deserialized_transaction)
        }
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------
