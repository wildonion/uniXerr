





use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use uuid::Uuid;
use std::{net::SocketAddr, time::Duration};
use crate::schemas::{Slot, Chain, Block};






#[derive(Message)]
#[rtype(result = "()")] //-- response type
pub struct Message{ //-- parathread sends this message to a parachain
    pub id: Uuid,
    pub cmd: String,
}

#[derive(Message)]
#[rtype(result = "()")] //-- result type of this event
pub struct Connect {
    pub recipient: Recipient<Message>, //-- another parachain recipient actor address
}

#[derive(Debug, Clone)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Parachain {
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub paractor: Option<Addr<Parachain>>, //-- another parachain actor address
    pub current_block: Option<Block>,
}

impl Parachain{
    fn health(){}
}

impl Actor for Parachain {
    type Context = Context<Parachain>;
    fn started(&mut self, ctx: &mut Self::Context){ //-- this function body will run once a parachain has been started
        let addr = ctx.address(); //-- getting the address of the this parachain actor
        let mut chain = Chain::default(); //-- start the network by building a genesis block and a default transaction with 100 coins from the coiniXerr network wallet to the wildonion wallet - we have to define it as mutable cause we'll cal its add() method in which a new created block will be added to the chain
        self.blockchain = Some(chain); //-- we can update the blockchain field cause we passed &mut self (a mutable pointer to all fields) as first parameter to the started() method
        print!("-> a new parachain has been built with slot id {}", self.slot.as_ref().unwrap().id); //-- as_ref() converts &Option<T> to Option<&T> - we can also use clone() method in order to make a deep copy of the slot field to prevent the field from moving and loosing ownership 
        println!("-> {} - attaching genesis block to the default chain", chrono::Local::now().naive_local());
        let genesis_block = self.blockchain.as_ref().unwrap().get_genesis(); //-- returns a borrow or immutable pointer to the genesis block
        println!("-> {} - shaping a new block to add transactions", chrono::Local::now().naive_local());
        self.current_block = Some(self.blockchain.as_ref().unwrap().build_raw_block(genesis_block)); //-- passing the genesis block by borrowing it - we have to define it as mutable cause we'll cal its push_transaction() method in which a new transaction will be added to the block
    }
}

impl Handler<Message> for Parachain { //-- implementing a Handler for Message event to send commands or messages to another parachain actor like issuing a smart contract event
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Context<Self>) -> Self::Result{
        println!("-> {} - message info received {} - {}", chrono::Local::now().naive_local(), self.slot.as_ref().unwrap().id, msg.id);
        ctx.run_later(Duration::new(0, 100), move |act, _| { //-- wait 100 nanoseconds
            act.paractor.as_ref().unwrap().do_send(Message { id: Uuid::new_v4(), cmd: "communicating with another parachain".to_string() }); //-- as_ref() converts &Option<T> to Option<&T> - sending a message to another parachain in the background (unless we await on it) is done through the parachain address and defined Message event or message 
        });
    }
}
