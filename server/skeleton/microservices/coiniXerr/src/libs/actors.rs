


use actix::prelude::*;
use uuid::Uuid; //-- loading actix actors and handlers for threads communication using their address and defined events 
use std::time::Duration;
use crate::schemas::Transaction;





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