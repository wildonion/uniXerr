


use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use uuid::Uuid;
use std::{net::SocketAddr, time::Duration};
use crate::schemas::{Transaction, ValidatorPool};
use crate::engine::contract::token::CRC20; //-- super is the root of the current directory (libs)









impl CRC20 for Validator{ //-- issuing a FT (fungible token) contract for a validator

    type TokenID = u8;
    type TokenName = String;
    type TotalSupply = u128;
    type Decimal = u8;
    type TokenAddress = String;
    type ExpTime = u64;

    fn mint(&mut self){
        //-- minting FT is a transaction and means assigning a token or an asset value with a limited to a wallet address which can be issued by this contract
        let mint_address: Self::TokenAddress = self.transaction.clone().unwrap().from_address; //-- self is a mutable pointer to the Validator fields - for unwrapping the transaction we must clone it cause it's behind a shared reference which is &mut behind the self parameter
    }

    fn transfer_from(&mut self){
        //-- transfer token from a sender to a recipient

    } 

    fn balance_of(&mut self){
        //-- provides the number of tokens held by a given wallet address

    } 
    
    fn approve(&mut self){
        //-- the code that's executed by the contract's method can include calls to other contracts, these trigger more transactions that have the from field set to the contract's address - a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information

    }

    fn allowance(&mut self){
        //-- provides the number of tokens allowed to be transferred from a given wallet address by another given wallet address
        
    } 

    fn owner_of(&mut self){
        //-- this function returns the address of the owner of a token. As each ERC-721 token is unique and non-fungible, they are represented on the blockchain by an ID,  other users, contracts, apps can use this ID to determine the owner of the token
    }

    fn burn(&mut self){
        //-- burn some the tokens
    }
}










#[derive(Message)]
#[rtype(result = "()")] //-- result type of this event
pub struct Contract {
    pub id: Uuid,
    pub ttype: u8,
}

#[derive(Debug, Clone)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Validator {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub transaction: Option<Transaction>, //-- signed transaction
    pub recipient: Option<Recipient<Contract>>, //-- recipient actor address
    pub pool: Option<String>,
}

impl Actor for Validator {
    type Context = Context<Validator>;
    fn started(&mut self, ctx: &mut Self::Context){ //-- this function body will run once a validator has been started
        let addr = ctx.address(); //-- getting the address of the this validator actor
        print!("-> a validator has been started with address {:?}", self.addr);
    }
}

impl Handler<Contract> for Validator { //-- implementing a Handler for Contract event to send commands or messages to another validator actor like issuing a smart contract event
    type Result = ();
    fn handle(&mut self, msg: Contract, ctx: &mut Context<Self>) -> Self::Result{
        println!("-> {} - contract info received {} - {}", chrono::Local::now().naive_local(), self.id, msg.id);
        ctx.run_later(Duration::new(0, 100), move |act, _| { //-- wait 100 nanoseconds
            act.recipient.as_ref().unwrap().do_send(Contract { id: Uuid::new_v4(), ttype: 0x02 }); //-- as_reF() converts &Option<T> to Option<&T> - sending a message to another validator in the background (unless we await on it) is done through the validator address and defined Contract event or message 
        });
    }
}
