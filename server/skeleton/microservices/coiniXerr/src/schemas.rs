


use std::rc::{Rc, Weak};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::net::SocketAddr; //-- these structures are not async; to be async in reading and writing from and to socket we must use tokio::net 
use actix::{*, prelude::*}; //-- loading actix actors and handlers for threads communication using their address and defined events 
use crate::libs::actors::Miner;
use std::collections::HashMap;






// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                         Token Schemas                   
// ==========--------------==========--------------==========--------------==========--------------==========--------------
pub mod Token{

    use uuid::Uuid;
    pub struct CRC20{
        pub id: Uuid,
        pub balance: i32,
    }

}















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                      Miner Pool Schema                      
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct MinerPool(pub Vec<Addr<Miner>>); //-- pool of miners
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------











// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                     RuntimeInfo Schema 
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Debug, Clone)]
pub struct RuntimeInfo{
    pub info_dict: HashMap<Uuid, MetaData>, //-- MetaData struct should implements the Debug and Clone trait also
}

impl RuntimeInfo{

    pub fn new() -> Self{
        RuntimeInfo{info_dict: HashMap::new()}
    }

    pub fn add(&mut self, meta_data: self::MetaData) -> Uuid{ //-- &self means borrowing the ownership of all RuntimeInfo fields - it must be mutable cause we want to insert into the info_dict
        let generated_uuid = Uuid::new_v4();
        self.info_dict.insert(generated_uuid, meta_data);
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
    pub actor: Miner, //-- Miner actor should implements the Debug and Clone trait also
}

impl MetaData{
    pub fn update_miner_transaction(&mut self, transaction: Option<Transaction>){ //-- updating the transaction field of the miner actor is done using a mutable borrower (pointer) as the parameter of the update_miner_transaction() method 
        self.actor.transaction = transaction;
    }
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------












// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                         Chain Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chain{
    pub branch_id: Uuid,
    pub branch_name: String,
    pub blocks: Vec<Block>,
}

impl Chain{
    
    pub fn default() -> Self{
        Chain{
            branch_id: Uuid::new_v4(),
            branch_name: "main".to_string(),
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
}
// ==========--------------==========--------------==========--------------==========--------------==========--------------
// ==========--------------==========--------------==========--------------==========--------------==========--------------

















// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                                                          Block Schema
// ==========--------------==========--------------==========--------------==========--------------==========--------------
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block{
    pub id: Uuid,
    pub is_genesis: bool,
    pub prev_hash: String,
    pub hash: String,
    pub merkle_root: String, //-- hash of all transactions in the form of a binary tree-like structure called merkle tree such that each hash is linked to its parent following a parent-child tree-like relation
    pub nonce: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //-- can't implement the Copy trait for Vec thus can't bound it to the Block structure 
    pub is_mined: bool,
}

impl Block{
    pub fn push_transaction(&mut self, transaction: Transaction) -> Self{ //-- the first param is a mutable pointer to every field of the struct
        self.transactions.push(transaction);
        Block{ //-- don't return &self when constructing the struct cause we'll face lifetime issue for struct fields - &mut T is not bounded to Copy trait due to ownership and borrowing rules which we can't have multiple mutable pointer at the same time
            id: self.id,
            is_genesis: self.is_genesis,
            prev_hash: self.prev_hash.clone(), //-- self.prev_hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            hash: self.hash.clone(), //-- self.hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            merkle_root: self.merkle_root.clone(), //-- self.merkle_root is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            nonce: self.nonce,
            timestamp: self.timestamp,
            transactions: self.transactions.clone(), //-- self.transactions is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for Vec thus we have to clone it 
            is_mined: self.is_mined,
        }
    }
}

impl Default for Block{
    fn default() -> Self{
        Block{
            id: Uuid::new_v4(),
            is_genesis: true,
            prev_hash: "hash of pervious block".to_string(), // TODO -
            hash: "hash of current block".to_string(), // TODO -
            merkle_root: "hash of merkle root".to_string(), // TODO - 
            nonce: 0,
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![Transaction::default()],
            is_mined: true,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub id: Uuid,
    pub amount: i32,
    pub from_address: String,
    pub to_address: String,
    pub issued: i64,
    pub signed: Option<i64>,
    pub hash: String,
}

impl Default for Transaction{
    fn default() -> Self{
        Transaction{
            id: Uuid::new_v4(),
            amount: 100,
            from_address: "genesis wallet address here".to_string(), // TODO - the address of the coiniXerr network 
            to_address: "a lucky user wallet address here".to_string(), // TODO - the address of the wildonion wallet
            issued: chrono::Local::now().naive_local().timestamp(),
            signed: Some(chrono::Local::now().naive_local().timestamp()),
            hash: "hash of the current transaction".to_string(), // TODO -
        }
    }
}

impl Transaction{
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