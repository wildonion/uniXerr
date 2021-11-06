


use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use uuid::Uuid;
use std::time::Duration;
use crate::schemas::Transaction;
use super::token::Contract; //-- super is the root of the current directory (libs)
use crate::schemas::Token::CRC20;




#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping {
    pub id: Uuid,
}

#[derive(Debug, Clone)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Miner {
    pub id: Uuid,
    pub transaction: Option<Transaction>,
    pub recipient: Option<Recipient<Ping>>,
    pub rewards: Option<i32>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
}

impl Handler<Ping> for Miner {
    type Result = ();
    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        println!("[{0}] Ping received {1}", self.id, msg.id);
        ctx.run_later(Duration::new(0, 100), move |act, _| { //-- wait 100 nanoseconds
            act.recipient.as_ref().unwrap().do_send(Ping { id: Uuid::new_v4()});
        });
    }
}

impl Contract for Miner{ //-- issuing a contract for a miner
    type Token = CRC20;

    fn transfer_from(&mut self){
        //-- transfer token from a sender to a recipient

    } 

    fn balance_of(&mut self){
        //-- provides the number of tokens held by a given address

    } 
    
    fn approve(&mut self){
        //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information

    }

    fn allowance(&mut self){
        //-- provides the number of tokens allowed to be transferred from a given address by another given address
        
    } 

    fn trade(&mut self){
        //-- do something after successfull token transfer

    } 
}