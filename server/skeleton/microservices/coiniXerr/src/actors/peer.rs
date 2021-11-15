


use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use uuid::Uuid;
use std::{net::SocketAddr, time::Duration};
use crate::schemas::{Transaction, MinerPool};
use crate::engine::contract::token::CRC20; //-- super is the root of the current directory (libs)









impl CRC20 for Miner{ //-- issuing a FT (fungible token) contract for a miner

    type TokenID = u8;
    type TokenName = String;
    type TotalSupply = u128;
    type Decimal = u8;
    type TokenAddress = String;
    type ValidTime = u64;

    fn mint(&mut self){
        //-- minting FT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
        Self::TokenAddress = self.transaction.unwrap().from_address;
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

    fn trade(&mut self){
        //-- do something after successfull token transfer

    } 
}











#[derive(Message)]
#[rtype(result = "()")] //-- result type of this event
pub struct Command {
    pub id: Uuid,
    pub cmd: String,
}

#[derive(Debug, Clone)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Miner {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub transaction: Option<Transaction>, //-- signed transaction
    pub recipient: Option<Recipient<Command>>, //-- recipient actor address
    pub pool: Option<String>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
    fn started(&mut self, ctx: &mut Self::Context){ //-- this function body will run once a miner has been started
        let addr = ctx.address(); //-- getting the address of the this miner actor
        print!("-> a miner has been started with address {:?}", self.addr);
    }
}

impl Handler<Command> for Miner { //-- implementing a Handler for Command event to send commands or messages to another miner actor like issuing a smart contract event
    type Result = ();
    fn handle(&mut self, msg: Command, ctx: &mut Context<Self>) -> Self::Result{
        println!("[{0}] command received {1}", self.id, msg.id);
        ctx.run_later(Duration::new(0, 100), move |act, _| { //-- wait 100 nanoseconds
            act.recipient.as_ref().unwrap().do_send(Command { id: Uuid::new_v4(), cmd: "balance".to_string() }); //-- sending a message to another miner is done through the miner address and defined Command event or message 
        });
    }
}
