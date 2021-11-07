


use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use uuid::Uuid;
use std::time::Duration;
use crate::schemas::{Transaction, Token::CRC20, MinerPool};
use super::token::Contract; //-- super is the root of the current directory (libs)







impl Contract for Miner{ //-- issuing a contract for a miner
    type Token = CRC20;

    fn transfer_from(&mut self){
        //-- transfer token from a sender to a recipient

    } 

    fn balance_of(&mut self){
        //-- provides the number of tokens held by a given wallet address

    } 
    
    fn approve(&mut self){
        //-- a token holder gives another wallet address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information

    }

    fn allowance(&mut self){
        //-- provides the number of tokens allowed to be transferred from a given wallet address by another given wallet address
        
    } 

    fn trade(&mut self){
        //-- do something after successfull token transfer

    } 
}









#[derive(Message)]
#[rtype(result = "()")]
pub struct Command {
    pub id: Uuid,
}

#[derive(Debug, Clone)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Miner {
    pub id: Uuid,
    pub transaction: Option<Transaction>,
    pub recipient: Option<Recipient<Command>>, //-- recipient address 
    pub rewards: Option<i32>,
    pub pool: Option<String>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
    fn started(&mut self, ctx: &mut Self::Context){ //-- this function body will run once a miner has been started
        let addr = ctx.address(); //-- getting the address of the actor
        print!("-> A miner has been started");
    }
}

impl Handler<Command> for Miner { //-- implementing a Handler for Command event to send commands or messages to another miner actor
    type Result = ();
    fn handle(&mut self, msg: Command, ctx: &mut Context<Self>) {
        println!("[{0}] Command received {1}", self.id, msg.id);
        ctx.run_later(Duration::new(0, 100), move |act, _| { //-- wait 100 nanoseconds
            act.recipient.as_ref().unwrap().do_send(Command { id: Uuid::new_v4()}); //-- sending a message to another miner is done through the miner address and defined Command event or message 
        });
    }
}