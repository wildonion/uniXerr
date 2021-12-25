




// program instruction logic and processors




use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg, 
};




#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract<T>{
    pub sign: T,
}



impl<T> Contract<T>{
    pub fn new(&self) -> Option<&T>{ //-- we need to define the first argument as &self which is an immutable pointer to all Contract struct fields to satisfy the return type of new() method which is Option<&T> 
        Some(self.sign) //-- returning a pointer to the sign field inside Option by using self which is &self in first argument of the new() method
    }
}



// =======================================
//  ....... SMART CONTRACT LOGIC
// =======================================
pub fn contract_program( //-- this program keeps track of the number of times the contract_program() program has been called for a given account - every program belongs to a instruction and every instruction belongs to an entrypoint loader and all accounts are owned by a specific program
    program_id: &Pubkey, //-- this is the public key of the account this program was loaded into and containing this program - a program is read-only or stateless and contains just program logic
    accounts: &[AccountInfo], //-- accounts to say hello to which interact with this program store data related to program interaction like saving infos about uploaded metadata files for this account - account contains data and owner to save data inside an account for a specific owner
    instruction_data: &[u8]
) -> ProgramResult { //-- the return type is ProgramResult  


    
    msg!("processing instruction: {} on {} accounts with incoming data={:?}", program_id, accounts.len(), instruction_data);
 
    
    
    //-- accounts are marked as executable during a successful program deployment process by the loader that owns the account means if the current program loader is the owner of this account then that account will be marked as executable
    //-- when a program is deployed to the execution engine (BPF deployment) the loader determines that the bytecode in the account's data is valid, if so, the loader permanently marks the program account as executable
    //-- an account's place in the array signifies its meaning, for example, when transferring lamports an instruction may define the first account as the source and the second as the destination
    let accounts_iter = &mut accounts.iter(); //-- an instruction will be done on one or more accounts in which the owner public key of each one must be owned by the program id public key of the instruction 
    let our_account = next_account_info(accounts_iter)?; //-- selecting this account to process the instructions of this program and read its data to deserialize it into the Contract struct on every call of this program
    
    
    if our_account.owner != program_id{ //-- account.owner is the program that ownes this account and is not controlled by a private key like other accounts cause accounts can only be owned by programs - the owner of this account which is called program derived account is the program id
        msg!("this account is not owned by this program");
        msg!("account data={:?}", our_account.data);
        return Err(ProgramError::IncorrectProgramId);
    }
    

    
    let mut contract_account_string_val = Contract::<String>::try_from_slice(&our_account.data.borrow())?; //-- Contract struct is taking a generic value of type String to deserialize our account data value into it - our_account.data is gaurded by the RefCell so we can call the borrow() method to borrow its ownership
    let mut contract_account = Contract::<u32>::try_from_slice(&our_account.data.borrow())?; //-- Contract struct takes a generic value of type T here is u32 - deserializing our account data into the Contract struct; we must define the deserialized account data as a mutable one in order to update its sign variable later - the data of an account can be a link to a picture art for nft based contracts
    
    
    contract_account.sign += 1; //-- keeps track of the number of times an account has sent a contract instruction to it
    contract_account_string_val.sign = format!("wildonion_sign-{}", contract_account.sign); //-- updating the sign with another instance of Contract struct with different type 
    
    
    let mut account_data = &mut our_account.data.borrow_mut()[..]; //-- mutable pointer to whole u8 bytes buffer of this account
    contract_account.serialize(&mut account_data)?; //-- serializing the contract into the utf8 bytes using the account data as the buffer
    msg!("Signed {} time(s)! with the name {}", contract_account.sign, contract_account_string_val.sign); //-- msg!() has lower runtime cost than the println!() 



    
    


    // client will send a transaction contains instruction to transfer lamports to another accounts 
    // we can be a middleware to catch all of these lamports and transfer 
    // them to another account like being a shity escrow.  
    // ...
    
    



    //-- at a specific time like every 2 or 3 days (epoch) votes will be transmited via the gossip protocol to the leader by every validators to form a slot (block) with morew than 2/3 of votes
    //-- votes are the hash of the computed state at that PoH tick count based on a greedy choice to maximize the reward
    //-- vote on 32 slots (blocks) over the past 12 seconds means 2 ** 32 slots (blocks) timeout in PoH
    //-- nft is a token with only one amount minted to an address which contains the url to the digital asset
    // TODO - other instructions on this program like changing the state of this program using incoming instruction_data when this program is called from the RPC client API
    // ...





    // traits, macros, Box<dyn Trait> &dyn Trait, mpsc job queue channel, ptr + len + cap, pointers and clone (deep copy) for borrowing instead of moving the ownership, 
    // lifetimes to prevent dangling pointers from returning, (de)serializing from or (d)ecoding into utf8 bytes or mapping using union, borsh and serde codec,  
    // from and into raw parts
    




    Ok(())




}