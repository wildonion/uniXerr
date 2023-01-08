









use ring::signature::KeyPair;

use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case




///// NOTE - 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
///// NOTE - borsh like codec ops : Box<[u8]> (automatic lifetime) or &'a [u8] <-> vec[u8] <-> struct
///// NOTE - &[u8] bytes to str using str::from_utf8() -> parse it and build the key:value hashmap -> build the struct instance from the hashmap
///// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value ----serde_json::to_string()----> json string or &str ----serde_json::from_str()----> struct
///// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value or json!({}) ----serde_json::from_value()---->  struct
///// NOTE - deserialization using slice       : &[u8] buffer ----serde_json::from_slice()----> struct
///// NOTE - serializing                       : struct instance ----serde_json::to_vec()----> Vec<u8> which will be coerced to &[u8] at compile time
///// NOTE - serializing                       : struct instance ----serde_json::to_string()----> json string will be coerced to &str at compile time 








//// since dynamic types like Vec, String and traits are on the heap, 
//// by passing them into new scopes they will be moved by rust compiler 
//// from the heap in order to free the memory location that they've just 
//// allocated to free up some huge space at runtime and this why rust doesn't
//// have garbage collector and use borrowing and ownership rule instead of that, 
//// the solution is to borrow the ownership of them from where they are stored 
//// either by cloning which is expensive or by taking a reference to them using
//// as_ref() method or putting & behind them to create a pointer which will 
//// point to the location of their heap area and is good to know that their 
//// pointers are fat ones since extra bytes which has been dedicated to their 
//// length inside the heap are in their pointers also their pointers must have 
//// valid lifetime across scopes and threads in order to avoid dangling pointer 
//// issue which we can achive this either by defining a lifetime in struct, enum 
//// fields or function signatur or by putting them inside the Box 
//// (with dyn keyword for trait) which is a smart pointer and 
//// have a valid lifetime in itself.
//
//// as_ref() will convert the type into a shared reference 
//// by returning the T as &T which we can't move out of it 
//// when it's being used by other scopes and threads thus 
//// we have to dereferene it either by * or cloning if the 
//// Clone trait is implemented for that, otherwise we CAN'T 
//// dereference or move it at all because Clone is a supertrait 
//// of the Copy trait.














// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                        P2P Schemas                      
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Debug, Serialize, Deserialize)] //// we'll use serde serialization and deserialization traits for json ops
pub struct P2PChainResponse{ //// local chain response from other peer - used for if someone sends us their local blockchain and use to send them our local chain
    pub blocks: Vec<Block>, //// blocks from other peers
    pub receiver: String, //// the receiver node (peer_id) of the incoming chain or blocks
}

//// if we send a LocalChainRequest with the peer_id 
//// of another node in the system, this will trigger 
//// that they send us their chain back.
#[derive(Debug, Serialize, Deserialize)]
pub struct P2PLocalChainRequest{ //// local chain request from a specific peer
    pub from_peer_id: String, //// a peer sends a request to get the local chain from other peers
}

//// for handling incoming messages, lazy initialization, and keyboard-input by the client's user
//// we've used the following enum which helps us send events across the application 
//// to keep our application state in sync with incoming and outgoing network traffic.
#[allow(clippy::large_enum_variant)] //// this will allow large size differences between enum variants
pub enum P2PBehaviourEvent{ //// this will be used inside the loop to change the code flow of the started swarm based on these events
    LocalChainResponse(P2PChainResponse), //// the response of the local chain request
    Input(String), //// keyboard inputs
    Init, //// lazy initialization
}


//// NetworkBehaviour trait defines the behaviour of the local node on the network
//// in contrast to Transport which defines how to send bytes on the network, 
//// NetworkBehaviour defines what bytes to send and to whom 
//// which in our case is based on gossipsub and mdns protocol. 
#[derive(NetworkBehaviour)] //// implementing the NetworkBehaviour trait for the P2PAppBehaviour struct
pub struct P2PAppBehaviour{
    pub gossipsub: Gossipsub, //// gossipsub protocol for p2p pub/sub: https://docs.libp2p.io/concepts/pubsub/overview/
    pub mdns: mdns::tokio::Behaviour, //// mDNS protocol is used to discover other nodes on the local network that support libp2p
}

impl P2PAppBehaviour{

    pub async fn new() -> Self{ //// it'll return a new app behaviour 

        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) //// hearbeat every 10 seconds
            .validation_mode(ValidationMode::Strict) //// this requires the message author to be a valid PeerId and to be present as well as the sequence number aslo all messages must have valid signatures
            .build()
            .unwrap();

        //// building the network behaviour
        //// based on gossipsub and mdns protcols.
        let mut behaviour = Self{
            gossipsub: Gossipsub::new( //// building a gossipsub network behaviour
                MessageAuthenticity::Signed(
                    KEYS.clone()), //// since the Clone trait is implemented for Topic struct hence we can clone the KEYS to dereference it   
                    gossipsub_config).unwrap(),
            mdns: mdns::Behaviour::new(mdns::Config::default()).unwrap(), //// building a default mdns
        };

        //// subscribing to parachain and block topics
        //// using gossip protocol.
        behaviour.gossipsub.subscribe(&PARACHAIN_TOPIC.clone()).unwrap();
        behaviour.gossipsub.subscribe(&BLOCK_TOPIC.clone()).unwrap();

        behaviour

    }

}


pub struct P2PEventLoop{
    //// we'll use the following channels the response_sender and init_sender 
    //// to send the local chain and a boolean of the lazy initialization 
    //// between multiple threads and scopes of different 
    //// parts of the app respectively.   
    pub response_sender: mpsc::Sender<P2PChainResponse>,
    pub init_sender: mpsc::Sender<bool>,
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                        Stake Info Schema                      
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Debug, Clone)]
pub struct Staker{
    pub id: Uuid,
    pub deposit: i32,
    pub owner: Validator, //// delegator or owner or staker is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈














// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                        Voter Info Schema                      
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voter{
    pub parachain_id: Uuid, //// voter will vote in this parachain using delegator stakes
    pub owner: Validator, //// owner is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
    pub staker_id: Option<Uuid>, //// delegator id who staked his/her money for this voter
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                  Parachain Slot Schema                      
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Slot{ //// pool of validators for slot auctions
    pub id: Uuid,
    pub name: String,
    pub voters: Vec<Voter>, //// auction voters for this slot
    pub epoch: u32, //// one epoch is the time taken to process 600k blocks which might takes 1 day or less depends on the coiniXerr network performence, after end of each epoch a new slot auction process will be started 
} 

impl Slot{

    //// we've cloned the self.validators and current_validators to prevent ownership moving
    pub fn get_validator(&self, validator_peer_id: String) -> Option<Validator>{
        let current_voters = self.voters.clone();
        let index = current_voters.iter().position(|v| v.owner.peer_id == validator_peer_id); //// this user has already participated in this event
        if index != None{
            Some(current_voters[index.unwrap()].clone().owner) //// returning the validator of the passed in socket address
        } else{
            None
        }
    }

    pub fn add_validator(&mut self, pid: Uuid, validator_peer_id: String) -> Self{
        
        //// building a new voter to push into the voters 
        let new_voter = Voter{
            parachain_id: pid,
            owner: Validator{
                peer_id: validator_peer_id,
                recent_transaction: None, //// it must be filled inside the stream channel the receiver side once the his/her incoming transaction gets signed
                mode: ValidatorMode::Mine,
                ttype_request: None, //// it must be filled inside the transaction mempool channel the receiver side once the transaction arrived
            },
            rewards: Some(0),
            signed_at: Some(chrono::Local::now().naive_local().timestamp()),
            staker_id: None,
        };

        self.voters.push(new_voter);
        Self { id: self.id, name: self.name.clone(), voters: self.voters.clone(), epoch: self.epoch }
    }

}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈




















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                         Chain Schema
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Serialize, Deserialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Chain{
    pub branch_id: Uuid, //// chain id
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

    pub fn new(branch_id: Uuid, branch_name: String, blocks: Vec<Block>) -> Self{ //// constructor of Chain struct - creating another branch or fork
        Chain{
            branch_id,
            branch_name,
            blocks,
        }
    }
    
    pub fn add(&mut self, block: Block) -> Self{ //// the first param is a mutable pointer to every field of the struct - self takes a copy of all fields and &mut borrow the ownership of those fields for mutating them
        self.blocks.push(block);
        Chain{
            branch_id: self.branch_id,
            branch_name: self.branch_name.clone(), //// Copy trait is not implemented for String thus we have to clone it to return from the function
            blocks: self.blocks.clone(), //// Copy trait is not implemented for blocks thus we have to clone it to return from the function
        }
    }

    pub fn get_genesis(&self) -> Block{
        let genesis = self.blocks[0].clone(); //// cloning the self.blocks[0] to prevent ownership moving since &self is an immutable reference to self which is a shared reference (means it has a valid lifetime and is being used by other methods) and can't be moved  
        genesis
    }

    pub fn build_raw_block(&self, prev_block: &Block) -> Block{ //// this method get an immutable pointer to the block (borrowed) as its second argument 
        Block{
            id: Uuid::new_v4(),
            index: prev_block.clone().index + 1, //// we have to clone the prev_block cause Block struct doesn't implement the Copy trait
            is_genesis: false,
            prev_hash: prev_block.clone().hash, //// first block inside the chain is the genesis block - we have to clone the prev_block cause Block struct doesn't implement the Copy trait 
            hash: None,
            merkle_root: None, 
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![],
            is_valid: false,
        }
    }

    pub fn is_chain_valid(&self) -> bool{

        // TODO - check that the chain is valid or not
        // TODO - call is_block_valid() method on the first two block instance of the chain 
        // ...

        todo!()

    }

}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                   Parachain mongodb schema
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InsertParachainInfo{
    pub id: Uuid,
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub next_parachain_id: Option<Uuid>, //// next parachain uuid
    pub current_block: Option<Block>,
}

#[async_trait]
impl StorageModel for InsertParachainInfo{

    type AppStorage = Option<Arc<Storage>>; //// the type of the AppStorage GAT is the Arc-ed Storage inside the Option since we don't know the exact engine in runtime 

    async fn save(&self) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>{ 
        let data = InsertParachainInfo{ //// building the instance from self since insert_one() method gets T not &T
            //// we must clone each field to prevent the self ownership from moving 
            //// since Copy is not implemented for InsertParachainInfo struct.
            id: self.id,
            slot: self.slot.clone(),
            blockchain: self.blockchain.clone(),
            next_parachain_id: self.next_parachain_id.clone(),
            current_block: self.current_block.clone(),
        };
        let unwrapped_storage = APP_STORAGE.clone().unwrap(); //// unwrapping the app storage to create a db instance
        let db_instance = unwrapped_storage.get_db().await.unwrap(); //// getting the db inside the app storage; it might be None
        let parachains = db_instance.clone().database(daemon::get_env_vars().get("DB_NAME").unwrap()).collection::<schemas::InsertParachainInfo>("parachains");
        match parachains.insert_one(data.clone(), None).await{ //// serializing the user doc which is of type RegisterRequest into the BSON to insert into the mongodb
            Ok(insert_result) => Ok(insert_result),
            Err(e) => Err(e)
        }
    } 

    //// we can either implement traits for the type by using them inside the crate 
    //// and use `impl for` them or bound the type in function signature to those 
    //// traits using where clause.
    async fn fetch(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized{

        todo!()

    }

    async fn filter(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized{

        todo!()

    }

}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈













// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                         Block Schema
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[derive(Serialize, Deserialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Block{
    pub id: Uuid,
    pub index: u32,
    pub is_genesis: bool,
    pub prev_hash: Option<String>, //// 
    pub hash: Option<String>,
    pub merkle_root: Option<String>, //// hash of all transactions in the form of a binary tree-like structure called merkle tree such that each hash is linked to its parent following a parent-child tree-like relation
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //// valid transactions (came through mempool channel) waiting to be confirmed and signed by the node time - can't implement the Copy trait for Vec thus can't bound it to the Block structure 
    pub is_valid: bool,
}

impl Block{

    pub fn push_transaction(&mut self, transaction: Transaction) -> Self{ //// the first param is a mutable pointer to every field of the struct
        self.transactions.push(transaction);
        Block{ //// don't return &self when constructing the struct cause we'll face lifetime issue for struct fields - &mut T is not bounded to Copy trait due to ownership and borrowing rules which we can't have multiple mutable pointer at the same time
            id: self.id,
            index: self.index,
            is_genesis: self.is_genesis,
            prev_hash: Some(self.prev_hash.clone().unwrap()), //// self.prev_hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            hash: Some(self.hash.clone().unwrap()), //// self.hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            merkle_root: Some(self.clone().merkle_root.unwrap()), //// self.merkle_root is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            timestamp: self.timestamp,
            transactions: self.transactions.clone(), //// self.transactions is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for Vec thus we have to clone it 
            is_valid: self.is_valid,
        }
    }

    //// this method will be used for generating the hash of the 
    //// block from the json string of the block 
    //// or the instance of the Block struct.
    fn serialize_block(&self) -> String{ 
        //// all the block data except hash must 
        //// be convert to the string first
        //// in order to generate its hash.
        let tx_data = serde_json::json!({ //// making a serde Value from the block data except the hash field since we want to fill it later
            "id": self.id,
            "index": self.index,
            "is_genesis": self.is_genesis,
            "prev_hash": self.prev_hash,
            "merkle_root": self.merkle_root,
            "timestamp": self.timestamp,
            "transactions": self.transactions,
            "is_valid": self.is_valid,
        });
        serde_json::to_string(&self).unwrap()
    }
     
    pub fn generate_hash(&mut self){
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let json_string_serialized_block = self.serialize_block(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        let block_hash_bytes = json_string_serialized_block.as_bytes(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        self.hash = Some(argon2::hash_encoded(block_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap());
    }
    
    pub fn is_block_valid(&self, prev_block: &Block) -> bool{

        // TODO - validation processes of this block with the prev block 
        // ...

        //// since we have separated if blocks 
        //// we must return something using the 
        //// `return` keyword only.
        if self.prev_hash != prev_block.hash{

            return false;

        } 
        
        if self.index != prev_block.index + 1{
            
            return false;

        } 
        
        //// the block must be validated using consensus algorithms first in order to be a valid
        //// since the is_valid flag will be set to true in there and we have to just check 
        //// the flag again in here.
        if self.is_valid == true && prev_block.is_valid == true{ 

            return false;
        } 
        
        true

    }

    pub fn generate_merkle_root_hash(&mut self) -> Self{

        // TODO - generate merkle root hash of this block
        // TODO - update self.merkle_root field
        // ...

        todo!()

    }

}

impl Default for Block{
    fn default() -> Self{ //// this must be used to generate the genesis block so some field are None
        Block{
            id: Uuid::new_v4(),
            index: 0,
            is_genesis: true,
            prev_hash: None, //// genesis block doens't have prev hash
            hash: None, //// must be filled later in consensus process
            merkle_root: None, //// must be filled later in consensus process
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![Transaction::default()],
            is_valid: true,
        }
    }
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈













// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                      Merkle Tree Schema
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//// Rc is a smart pointer used for counting the incoming references to the type which shared its ownership using &
//// and see how many owners the borrowed type has in its entire scope as long as its lifetime is not dropped also 
//// it has nothing to do with the garbage collection rule cause rust doesn't have it.
//
//// all transactions inside a block will be stored in form of a merkle tree and since 
//// it'll chain transaction hash together is a useful algorithm for proof of chain.
#[derive(Debug)]
pub struct Node{
    pub id: Uuid,
    pub data: Transaction, //// in each node data is the transaction hash; if the data is of type Transaction just call the data.hash since the hash of the transaction has been generated in different TLPs
    pub parent: RefCell<Weak<Node>>, //// we want to modify which nodes are parent of another node at runtime, so we have a RefCell<T> in parent around the Vec<Weak<Node>> - child -> parent using Weak to break the cycle, counting immutable none owning references to parent - weak pointer or none owning reference to a parent cause deleting the child shouldn't delete the parent node
    pub children: RefCell<Vec<Rc<Node>>>, //// we want to modify which nodes are children of another node at runtime, so we have a RefCell<T> in children around the Vec<Rc<Node>> - parent -> child, counting immutable references or borrowers to childlren - strong pointer to all children cause every child has a parent which the parent owns multiple node as its children and once we remove it all its children must be removed
}

impl Node{

    pub fn is_leaf(&mut self) -> bool{
        todo!();
    }

    pub fn add_child(&mut self, node: Node){
        self.children.borrow_mut().push(Rc::new(node)); //// in order to push into the self.children field we have to borrow it as mutable at runtime since it has wrapped around the RefCell
    }

    pub fn children(&mut self, node: Node) -> Result<Vec<Rc<Self>>, String>{ //// &mut self means we're borrowing Node fields using a mutable pointer which is a shallow copy of the fields (if we change the pointer value the actual object will be changed) for updaing the desired field
        if node.children.borrow_mut().len() != 0{ //// first borrow the ownership of the self.children field at runtime then check its length
            Ok(node.children.borrow_mut().to_vec()) //// we have to borrow the ownership of the self.children field at runtime and convert it to vector to return it cause it's inside RefCell
        } else{
            Err(format!("node -{}- has no children", node.id).to_string())
        }
    }

    pub fn generate_hash(&mut self, right_node: Node) -> String{

        // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
        // https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
        // https://nomicon.io/DataStructures/MerkleProof
        // TODO - use argon2 to generate the hash of self.data.hash + right_node.data.hash
        // ...

        todo!()

    }
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                        Transaction Schema
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//// `Ed25519` is being used instead of `ECDSA` as the public-key 
//// (asymmetric) digital signature encryption to generate the 
//// wallet address keypair the public and private keys; the hash 
//// of the public key is being used to generate the wallet address 
//// also we **MUST** use the public key for transaction signature 
//// verification; the private key on the other hand will be used 
//// to sign transaction data (tx data **MUST** first serialized 
//// to an array of `utf8` bytes) finally for the transaction, 
//// block and `merkle` root hash and generating wallet address 
//// (hash of the public key) `Argon2` is being used as the `KDF` method.
 
unsafe impl Send for TransactionMem {} //// due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads and avoid concurrent mutations.
//// all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
//// there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
//// enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself
union TransactionMem{
    pub data: *mut self::Transaction, //// defining the data as a raw mutable pointer to a mutable Transaction object, changing the data will change the Transaction object and vice versa
    pub buffer: *const u8,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Transaction{
    #[borsh_skip] //// skip serializing this field since the BorshSerializer trait is not implemented for Uuid types
    pub id: Uuid,
    pub ttype: u8, //// 00000000 or 0x00 is one byte - every 4 bits in one byte is a hex number so 8 bits is 2 hex number in one byte representation bits and every 3 digit in one byte is a oct number
    pub amount: i32,
    pub public_key_bytes: Vec<u8>,
    pub from_address: String,
    pub to_address: String,
    pub issued: i64,
    pub signed: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub hash: Option<String>,
}

impl Default for Transaction{
    fn default() -> Self{
        
        //// unwrap() method takes self not &self thus it takes ownership of 
        //// the receiver `self`, which moves value means that by calling the 
        //// unwrap() method on COINIXERR_NODE_WALLET_KEYPAIR we can't call other
        //// methods of the COINIXERR_NODE_WALLET_KEYPAIR instance again since 
        //// the instance will be moved, the solution is to use as_ref() method
        //// on COINIXERR_NODE_WALLET_KEYPAIR instance to borrow its ownership
        //// the call the .unwrap() to prevent ownership moving by the first call.
        let coiniXerr_node_keypair = COINIXERR_NODE_WALLET_KEYPAIR.as_ref().unwrap(); 
        let public_key = coiniXerr_node_keypair.public_key().as_ref(); //// as_ref() converts to &[u8] which is an array or an slice of the Vec<u8> since Vec<u8> will be coerced to &[u8] at compile time
        let wallet_address = Self::generate_wallet_address_from(public_key); //// generating public key from the wallet address

        let mut tx = Transaction{ //// sending a transaction from to this coiniXerr node
            id: Uuid::new_v4(),
            ttype: 0x00, //// 0x00 means 0 in hex and a regular transaction - 0xFF or 1 (CRC21) and 0x02 or 2 (CRC20) and 0x03 or 3 (CRC22) in hex means smart contract transaction
            amount: 100,
            public_key_bytes: public_key.to_owned(), //// to_owned() will convert the borrowed data or &[u8] to owned data or Vec<u8>; Vec<u8> will be coerced to &[u8] at compile time to be stroed on the stack
            from_address: wallet_address.clone(), //// sender wallet address
            to_address: wallet_address, //// receiver wallet address
            issued: chrono::Local::now().naive_local().timestamp(),
            signed: Some(chrono::Local::now().naive_local().timestamp()),
            signature: None, //// transaction object needs to be signed using the sender's private key from walleXerr and this cryptographically proves that the transaction could only have come from the sender and was not sent fraudulently
            hash: None
        };


        //// we didn't save the Self::serialize_transaction_from(&tx)
        //// into a variable since the variable can't be used again
        //// to pass it to the Self::generate_hash_from() method
        //// to generate the hash because after updating signature field
        //// we must serialize the transaction data gain hence it's better
        //// to call Self::serialize_transaction_from(&tx) every time we need it!
        //  
        //// signing the data which is the serialized transaction bytes
        //// with the generated private key; the result is the transaction
        //// signature which must be verified inside the coiniXerr node.
        info!("➔ 🖊️ signing serialized transaction bytes using private key 🔑"); 
        let signature = coiniXerr_node_keypair
                                        .sign(
                                            //// at this moment when we serialize the tx
                                            //// the signature field is empty and we're 
                                            //// serializing with empty signature field
                                            //// to produce the signature.
                                            Self::serialize_transaction_from(&tx)
                                                    .as_bytes()
                                            ); //// if we call as_ref() method in here value before it will be dropped sooner thus we have to make a longer lifetime then call the as_ref() method
                                            
        let signature = signature.as_ref();
        
        //// updating the signature field before 
        //// generating the hash since in we need
        //// it to produce the hash of the tx.
        //
        //// to_owned() will convert the borrowed data or 
        //// &[u8] to owned data or Vec<u8>
        tx.signature = Some(signature.to_owned()); 
        tx.hash = Some(
                        Self::generate_hash_from(
                            //// calling Self::serialize_transaction_from(&tx) again since we've updated the signature field
                            Self::serialize_transaction_from(&tx))
                        );
        tx //// returning the default transaction

    }
}

impl Transaction{
    
    //// a transaction decoder or deserializer using union
    pub fn new(buffer: &[u8]) -> Result<&mut Self, Box<dyn std::error::Error>>{ //// self is a copy to all values of the struct; &self is a pointer to those values means by doing this we will borrow ownership of all original values
        unsafe{ // NOTE - if neither Copy nor Clone is not implemented for the object by moving it into a function we loose the ownership of the value of that object; we can borrow the ownership by taking a pointer to it using &
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //// filling the buffer field will also fill the data cause thay have a same memory storage
            let deserialized_transaction = &mut *transaction.data; //// mutable pointer to the dereferenced transaction data - since the data inside the union is a raw pointer to a mutable Transaction object we have to dereference it to return a Transaction object; we also want to change the object later so we have to take a mutable pointer or reference (&mut) to the dereferenced object to borrow the ownership of the original object for mutation
            Ok(deserialized_transaction)
        }
    }

    //// this method will be used for generating the hash of the 
    //// transaction from the json string of the transaction 
    //// or the instance of the Transaction struct.
    fn serialize_transaction(&self) -> String{ //// we'll use this method to hash the signed transaction comming from the walleXerr
        //// all the transaction data except hash must 
        //// be convert to the string first
        //// in order to generate its hash.
        let tx_data = serde_json::json!({ //// making a serde Value from the transaction data except the hash field since we want to fill it later
            "id": self.id,
            "ttype": self.ttype,
            "amount": self.amount,
            "public_key_bytes": self.public_key_bytes,
            "from_address": self.from_address,
            "to_address": self.to_address,
            "issued": self.issued,
            "signed": self.signed,
            "signature": self.signature
        });
        serde_json::to_string(&tx_data).unwrap()
    }
     
    //// we'll use this method to serialize the passed in transaction
    //// to json string to produce an array of utf8 bytes (&[u8]) for signing it 
    //// with the private key and filling the hash field.  
    pub fn serialize_transaction_from(transaction: &Transaction) -> String{ 
        //// the return type of tx_data is Value
        //// which we'll convert it into the json string
        let tx_data = if let Some(_) = transaction.signature{ //// if the signature has some value means that we're serializing to generate the hash
            serde_json::json!({ //// this is the data that must be hashed
                "id": transaction.id,
                "ttype": transaction.ttype,
                "amount": transaction.amount,
                "public_key_bytes": transaction.public_key_bytes,
                "from_address": transaction.from_address,
                "to_address": transaction.to_address,
                "issued": transaction.issued,
                "signed": transaction.signature,
                "signature": transaction.signature
            })
        } else{ //// if the signature field was None means that we're serializing to sign the transaction with private key to generate the signature
            serde_json::json!({ //// this is the data that must be signed
                "id": transaction.id,
                "ttype": transaction.ttype,
                "amount": transaction.amount,
                "public_key_bytes": transaction.public_key_bytes,
                "from_address": transaction.from_address,
                "to_address": transaction.to_address,
                "issued": transaction.issued
            })
        };
        serde_json::to_string(&tx_data).unwrap()
    }

    pub fn generate_hash(&mut self){ //// this method must be called only inside the coiniXerr node
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let json_string_serialized_transaction = self.serialize_transaction(); //// we're generating a longer lifetime since in self.serialize_transaction().as_bytes(); the self.serialize_transaction() will be dropped before we calling as_bytes() method;
        let transaction_hash_bytes = json_string_serialized_transaction.as_bytes();
        self.hash = Some(argon2::hash_encoded(transaction_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap());
    } 

    pub fn generate_hash_from(serialized_tx: String) -> String{ //// this method must be called only inside the coiniXerr node
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let transaction_hash_bytes = serialized_tx.as_bytes();
        argon2::hash_encoded(transaction_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap()
    } 

    fn generate_wallet_address_from(pubk: &[u8]) -> String{ //// generating wallet address from the public key using Argon2
        let salt = daemon::get_env_vars().get("GENERATE_WALLET_ADDRESS_SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        argon2::hash_encoded(pubk, salt_bytes, &argon2::Config::default()).unwrap()
    }

    pub fn is_transaction_valid(&self) -> bool{

       self.verify_signature() 

    }

    fn verify_signature(&self) -> bool{
        info!("➔ 🏷️ verifying transaction signature using public-key (asymmetric) digital signature encryption based on Ed25519");
        match self.signature.clone(){ // https://cryptobook.nakov.com/digital-signatures | we'll use the self.signature later thus we have to clone it
            Some(tx_sig) => {                
                info!("➔ 🕵🏾‍♀️ verifying transaction signature using public key 🔑"); 
                let json_string_serialized_transaction = self.serialize_transaction(); //// we're generating a longer lifetime since in self.serialize_transaction().as_bytes(); the self.serialize_transaction() will be dropped before we calling as_bytes() method;
                let serialized_tx_bytes = json_string_serialized_transaction .as_bytes(); 
                if let None = self.signature{ //// we can use self.signature.is_none() also 
                    info!("➔ ⛔ null signature 🔑"); 
                    //// since we have separated if blocks 
                    //// we must return something using the 
                    //// `return` keyword only.
                    return false; 
                }
                if !self.public_key_bytes.is_empty() && self.signature.is_some(){ 
                    let peer_public_key = ring_signature::UnparsedPublicKey::new(&ring_signature::ED25519, self.public_key_bytes.clone()); //// generating the public key from the public key bytes 
                    let sig = &self.signature.clone().unwrap() as &[u8]; //// casting the &Vec<u8> to &[u8]
                    //// verifying the signature based on the 
                    //// serialized transaction bytes and the  
                    //// public key; public key will be used to 
                    //// find the private key of the signer 
                    //// which is inside the walleXerr to 
                    //// complete the verification process.
                    match peer_public_key.verify(serialized_tx_bytes, sig){ 
                        Ok(_) => true,
                        Err(_) => false
                    }
                } else{
                    false //// public address is empty which contains zero bytes
                }
            
            },
            None => false //// empty signature means false
        }
    }

}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
















// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                                                      Db and Storage Schemas
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[derive(Clone, Debug)] //// can't bound Copy trait cause engine and url are String which are heap data structure 
pub struct Db{
    pub mode: Mode,
    pub engine: Option<String>,
    pub url: Option<String>,
    pub instance: Option<Client>,
}

impl Default for Db{
    fn default() -> Db {
        Db{
            mode: self::Mode::Off,
            engine: None,
            url: None,
            instance: None,
        }
    }
}

impl Db{
    
    pub async fn new() -> Result<Db, Box<dyn std::error::Error>>{
        Ok(
            Db{ //// building an instance with generic type C which is the type of the db client instance
                mode: Mode::On, //// 1 means is on 
                engine: None, 
                url: None,
                instance: None,
            }
        )
    }
    
    pub async fn GetMongoDbInstance(&self) -> Client{ //// it'll return an instance of the mongodb client - we set the first argument to &self in order to have the instance of the object later on after calling this method and prevent ownership moving
        Client::with_uri_str(self.url.as_ref().unwrap()).await.unwrap() //// building mongodb client instance
    }

}



#[derive(Clone, Debug)]
pub struct Storage{
    pub id: Uuid,
    pub db: Option<Db>, //// we could have no db at all
}

impl Storage{
    pub async fn get_db(&self) -> Option<&Client>{
        match self.db.as_ref().unwrap().mode{
            Mode::On => self.db.as_ref().unwrap().instance.as_ref(), //// return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client> or Option<&T>
            Mode::Off => None, //// no db is available cause it's off
        }
    }
}



#[derive(Copy, Clone, Debug)]
pub enum Mode{ //// enum uses 8 bytes (usize which is 64 bits on 64 bits arch) tag which is a pointer pointing to the current variant - the total size of this enum is 8 bytes tag + the largest variant size = 8 + 0 = 8 bytes; cause in our case On and Off variant both have 0 size
    On, //// zero byte size
    Off, //// zero byte size
}
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈




































// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                          RabbitMQ RMQAccount Stream Contains Publisher and Subscriber using lopin crate
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

/*


    mq clients in rust and js
        | 
        -------- coiniXerr mq producer and consumer actor streamer -------- conse hyper server 
                        |                             |
                        |                             -------- mongodb
                        |
                        <---tcp socket--> |broker actor streamer on VPS <---routing channel exchange--> job or task queue buffer| 
                                                                                                            |
                                                                                                            |
                                                                                                            |
                                                                                                            <---mpsc channel---> worker threadpools




    https://www.cloudamqp.com/blog/part1-rabbitmq-for-beginners-what-is-rabbitmq.html#exchanges


    • Producer: Application that sends the messages.
    • Consumer: Application that receives the messages.
    • Queue: Buffer that stores messages.
    • Message: Information that is sent from the producer to a consumer through RabbitMQ.
    • Connection: A TCP connection between your application and the RabbitMQ broker.
    • Channel: A virtual connection inside a connection. When publishing or consuming messages from a queue - it's all done over a channel.
    • Exchange: Receives messages from producers and pushes them to queues depending on rules defined by the exchange type. To receive messages, a queue needs to be bound to at least one exchange.
    • Binding: A binding is a link between a queue and an exchange.
    • Routing key: A key that the exchange looks at to decide how to route the message to queues. Think of the routing key like an address for the message.
    • AMQP: Advanced Message Queuing Protocol is the protocol used by RabbitMQ for messaging.



    mq is actually a tcp socket channel based on actor desing pattern that will send and receive buffers like any other socket channels
    but the only difference between others is it can manage incoming payloads in a specific manner like:
        • it uses an async job or task queue like mpsc jobq channel and celery algos to communicating between actors' threads (send and receive tasks and messages between their worker threadpools)  
        • it has a batch handler which means it can take a batch of tasks and publish them to the producers from the queue
        • receiving only a specific message on a specific topic (receivers can only subscribe to a specific topic)
        • schduling a message to be sent later using a task queue handler
        • schduling a message to be received at a specific condition using a task queue handler
        • sending and broadcasting message only to specific receivers 
        • handle (send and receive) tasks and messages asyncly inside a threadpool
        • buffering messages inside a queue to send them once the receiver gets backed online





        

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                  lopin rmq setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈      
    ////
    ////         publisher/subscriber app (rust or js code) 
    ////                      |
    ////                       ---- tcp socket
    ////                                       |
    ////                              rpc broker channels
    ////                                       |
    ////                                        --------- exchange
    ////                                                     |
    ////                             routing key ------- |binding| ------- routing key
    ////                                                     |
    ////                                             jobq queue buffer
    ////                                                     |
    ////                                                      --------- worker threadpool 
    ////
    //// ➔ publishers (rust or js code) which is connected to the mq broker can publish messages to a channel 
    ////    from there (inside the broker channels) messages will be buffered inside a specific queue.
    //// ➔ subscribers (rust or js code) want to subscribe to a specific message in which they must talk to a channel
    ////    then the channel will talk to the broker to get the message from a specific queue.
    //// ➔ rabbitmq uses queues instead of topics means that we can get all messages from a specific queues 
    ////    instead of subscribing to a specific topic by doing this all consumers can subscribe to a specific queue.  
    //// ➔ there might be multiple channels each of which are able to talk to a specific queue to get the buffered message from there.

    let coiniXerr_account_id = Uuid::new_v4().to_string();
    let mut account = RMQAccount::new(
                                    &ampq_addr,
                                    2, 
                                    coiniXerr_account_id
                                ).await;
                                
                                
                                
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////         making queues, publish and subscribe
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    account //// making the hoop queue for publishing and subscribing process
        .make_queue("hoop")
        .await;
        
    account //// the publisher could be another app written in another lang
    .publish(10, "", "hoop") //// publishing 10 times on the passed in queue
        .await;
        
    account //// the subscriber could be another app written in another lang
        .subscribe("hoop") //// subscribing to the hoop queue
        .await;
        








        
*/
        
// #[derive(Debug)]    
// pub enum Topic{
//     Hoop,
//     ReHoop,
//     Mention,
//     HashTag,
//     Like,
//     RMQAccountInfo,
// }   

// //// if Clone trait is not implemented for a type and that type is also a field of a structure we can't have &self in
// //   structure methods since using a shared reference requires Clone trait be implemented for all types of the structure 
// //   otherwise we can't share none of the fields of the structure and by calling a method of the structure on the instance
// //   the instance will be no longer valid and be moved.
// //// if the first param of methods was &self that means the instance is behind a shared reference
// //// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
// //// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all
// //// hence we can't have &self as the first param.
// pub struct RMQAccount{ //// RMQAccount is the user that can publish and subscribe to the messages
//     pub account_id: String, //// this is the _id of the account that wants to publish messages
//     pub channels: Vec<Channel>, //// rabbitmq channels
//     pub queues: Vec<Queue>, //// rabbitmq queues
// } 

// impl RMQAccount{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
//     //// this method will build the connection to the broker and rabbitmq channels to talk to publishers and subscribers
//     pub async fn new(broker_addr: &str, n_channels: u16, acc_id: String) -> Self{ 

//         // ----------------------------------------------------------------------
//         //                     CONNECTING TO RABBITMQ BROKER
//         // ----------------------------------------------------------------------
        
//         let conn = Connection::connect(&broker_addr, ConnectionProperties::default().with_default_executor(10)).await.unwrap();
//         info!("➔ 🟢 ⛓️ connected to the broker");
        
//         // ----------------------------------------------------------------------
//         //            CREATING RABBITMQ CHANNELS TO TALK TO THE BROKER
//         // ----------------------------------------------------------------------

//         let mut channels = Vec::<Channel>::new(); //// producers and consumers must talk to the channel first
//         for i in 0..n_channels{
//             channels.push(
//                 conn.create_channel().await.unwrap()
//             );
//         }
//         info!("➔ 🟢 🕳️ channels created susscessfully");
//         Self{ //// returning a new instance of the RMQAccount also is Self is the complete type of the RMQAccount<T> not just the constructor or RMQAccount
//             account_id: acc_id,
//             channels,
//             queues: Vec::new(), // or vec![]
//         }
//     }

//     //// this method will build the queue from the passed in name
//     pub async fn make_queue(&mut self, name: &str) -> Self{

//         // ----------------------------------------------------------------------
//         //             BUILDING THE HOOP QUEUE USING THE FIRST CHANNEL
//         // ----------------------------------------------------------------------

//         // let RMQAccount { account_id, channels, queues } = self; //// unpacking the self into the RMQAccount struct; by defining the self as mutable every field of the unpacked self will be mutable
        
//         //// consider the first one as the publisher channel and the second as the subscriber channel
//         let first_channel = self.channels[0].clone();
//         let mut queues = self.queues.clone();
//         queues.push(
//             first_channel.queue_declare(
//                             name, //// defining the queue with passed in name; this can be later used to subscribe messages to the buffer of this queue 
//                             QueueDeclareOptions::default(), 
//                             FieldTable::default(),
//                         ).await.unwrap()
//         );
        
//         info!("➔ 🟢🎣 queue created susscessfully");
        
//         Self{
//             account_id: self.account_id.clone(), //// cannot move out of `self.account_id` which is behind a mutable reference 
//             channels: self.channels.clone(), //// cannot move out of `self.channels` which is behind a mutable reference
//             queues,
//         }

    
//     }

//     //// this method will build the consumer from the second channel 
//     //// and wait for each message to be consumed from the specified queue
//     //// until all the message gets deliverred.
//     pub async fn subscribe(&self, queue: &str){

//         // -------------------------------------------------------------------------------------------------------------
//         //             BUILDING THE CONSUMER FROM THE SECOND CHANNEL TO SUBSCRIBE TO THE PUBLISHED MESSAGES  
//         // -------------------------------------------------------------------------------------------------------------

//         //// we're using Arc to clone the second_channel since Arc is to safe for sharing the type between threads 
//         info!("➔ 🟢📩 subscribing from the second channel to the published messages from the [{}] queue", queue);
//         let second_channel = self.channels[1].clone(); //// we've used the second channel to use its consumer to get all message deliveries
//         let consumer_channel = Arc::new(&second_channel); //// putting the borrowed form of second_channel inside the Arc (since we want to clone it later for ack processes) to prevent ownership moving since we want to consume messages inside a worker threadpool
//         let consumer = consumer_channel
//                             .clone()
//                             .basic_consume( //// it'll return the consumer which will be used to get all the message deliveries from the specified queue
//                                 queue, //// the quque that we've just built and want to get all messages which are buffered by the publisher 
//                                 format!("{} consumer", queue).as_str(),  
//                                 BasicConsumeOptions::default(),
//                                 FieldTable::default(),
//                             ).await.unwrap();

//         // ----------------------------------------------------------------------
//         //           GETTING ALL THE DELIVERIES OF THE CONSUMED MESSAGES
//         // ----------------------------------------------------------------------
//         let second_channel = second_channel.clone(); //// cloning the second channel to prevent ownership moving since we're moving the channel into the tokio spawn scope
//         tokio::spawn(async move{ //// spawning async task that can be solved inside the tokio green threadpool under the hood which in our case is consuming all the messages from the passed in queue buffer  
//             info!("➔ 🪢🛀🏽 consuming deliveries inside tokio worker green threadpool");
//             consumer
//                 .for_each(move |delivery|{ //// awaiting on each message delivery 
//                     let delivery = delivery.expect("Error in consuming!").1;
//                     second_channel
//                         .basic_ack(delivery.delivery_tag, BasicAckOptions::default()) //// acknowledging the messages using their delivery tags
//                         .map(|_| ())
//                 }).await
//         });

//     }

//     //// this method will build the producer from the first channel 
//     //// and produce payloads based on the passed in criteria to send them 
//     //// to the specified routing key which in this case is our queue name.
//     pub async fn publish(&self, criteria: u16, exchange: &str, routing_key: &str){

//         // -----------------------------------------------------------------------------------------------------------------
//         //             BUILDING THE PUBLISHER FROM THE FIRST CHANNEL TO PUBLISH MESSAGES TO THE SPECIFIED QUEUE  
//         // -----------------------------------------------------------------------------------------------------------------

//         info!("➔ 🟢🛰️ publishing messages from the first channel to the [{}] queue", exchange);
//         let first_channel = self.channels[0].clone();
//         for n in 0..criteria{ //// sending the payload `criteria` times
//             let message = format!("[{:?} ➔ {}-th]", Topic::Hoop, n); //// enum field first will be converted into String then into utf8 bytes
//             let payload = message.as_bytes(); //// converting the message to utf8 bytes
//             info!("➔ 🟢📦 iteration [{}], publishing payload", n);
//             let confirm = first_channel
//                                         .basic_publish(
//                                             exchange, //// exchange receives message from publishers and pushes them to queues by using binders and routing keys
//                                             routing_key, //// this is the routing key and is the address that the message must be sent to like the queue name in which the messages will be buffered inside  
//                                             BasicPublishOptions::default(),
//                                             payload.to_vec(), //// the payload that must be published
//                                             BasicProperties::default(),
//                                         )
//                                         .await.unwrap()
//                                         .await.unwrap();
//             assert_eq!(confirm, Confirmation::NotRequested);
//         }

//     }


// } 

// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                    RabbitMQ RMQAccount Stream Contains Publisher and Subscriber rabbitmq_stream_client crate
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

// #[derive(Debug)]    
// pub enum Topic{
//     Hoop,
//     ReHoop,
//     Mention,
//     HashTag,
//     Like,
//     RMQAccountInfo,
// }   

// //// if Clone trait is not implemented for a type and that type is also a field of a structure we can't have &self in
// //   structure methods since using a shared reference requires Clone trait be implemented for all types of the structure 
// //   otherwise we can't share none of the fields of the structure and by calling a method of the structure on the instance
// //   the instance will be no longer valid and be moved.
// //// if the first param of methods was &self that means the instance is behind a shared reference
// //// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
// //// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all
// //// hence we can't have &self as the first param.
// pub struct RMQAccount{ //// RMQAccount is the user that can publish and subscribe to the messages
//     pub account_id: String, //// this is the _id of the account that wants to publish messages
//     pub env: Environment, //// the rabbitmq environemt which is used to publish or subscribe
//     pub producer: Option<Producer<Dedup>>, //// Clone trait is not implemented for the DedUp thus we can't clone or copy this field
//     pub consumer: Option<Consumer>,
// } 

// impl RMQAccount{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
//     pub async fn new(env: Environment, acc_id: String) -> Self{
//         Self{
//             account_id: acc_id,
//             env,
//             producer: None,
//             consumer: None,
//         }
//     }

//     pub async fn build_producer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

//         info!("➔ 🟢 building hoopoe producer");

//         let prod = self.env
//                 .producer()
//                 .name("hoopoe_publisher")
//                 .build("hoopoe_producer_stream")
//                 .await
//                 .unwrap();
        
//         Self{
//             account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
//             env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
//             producer: Some(prod),
//             consumer: self.consumer, //// since self is not a shared reference thus we can move it into new scope
//         }

//     }

//     pub async fn build_consumer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

//         info!("➔ 🟢 building hoopoe consumer");

//         let cons = self.env
//                 .consumer()
//                 .build("hoopoe_consumer_stream")
//                 .await
//                 .unwrap();
        
//         Self{
//             account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
//             env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
//             producer: self.producer, //// since self is not a shared reference thus we can move it into new scope
//             consumer: Some(cons), 
//         }

//     }

//     pub async fn publish(producer: Option<Producer<Dedup>>, topic: Topic, message: String) -> Option<Producer<Dedup>>{ //// we're returning the producer for later calls since once the producer gets passed to this method it'll be moved and there will be no longer available 

//         //        to be executed from the hoops queue buffer until the consumer is backed on line
//         // ...
//         let body = match topic{
//             Hoop => format!("hooping: {}", message), 
//             ReHoop => format!("rehooping: {}", message), 
//             Mention => format!("Mentioning: {}", message),
//             HashTag => format!("Hashtaging: {}", message),
//             Like => format!("Liking: {}", message),
//         };

//         if let Some(mut prod) = producer{
//             info!("➔ 🟢 publishing");
//             prod
//                 .send(Message::builder().body(body).build(), |_| async move{})
//                 .await
//                 .unwrap();            
//             Some(prod)
//         } else{
//             None
//         }        

//     }

//     pub async fn subscribe(consumer: Option<Consumer>){

//         let mut consumer = consumer.unwrap(); //// defining the consumer as mutable since receiving and reading from the consumer is a mutable process and needs the futures::StreamExt trait to be imported 
//         tokio::spawn(async move{
//             info!("➔ 🟢 subscribing");
//             while let Some(delivery) = consumer.next().await{ //// streaming over the consumer to receive all the messages comming from the producer while there is some delivery
//                 info!("Received message {:?}", delivery);
//             }
//         });

//     }

//     pub async fn close_producer(producer: Option<Producer<Dedup>>){
//         if let Some(prod) = producer{
//             info!("➔ 🟢 closing hoopoe producer");
//             prod
//                 .close().await
//                 .unwrap();
//         }
//     }

//     pub async fn close_consumer(consumer: Option<Consumer>){
//         if let Some(cons) = consumer{
//             info!("➔ 🟢 closing hoopoe consumer");
//             let consumer_handler = cons.handle();
//             consumer_handler
//                     .close().await
//                     .unwrap();
//         }
//     }

// } 
