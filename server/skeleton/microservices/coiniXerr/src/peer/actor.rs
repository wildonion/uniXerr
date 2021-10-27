


use actix::prelude::*; //-- loading actix actors and handlers for threads communication using their address and defined events 
use std::time::Duration;
use liby;



#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping {
    pub id: usize,
}

#[derive(Debug)] //-- this is required for unwrapping the sender of mpsc channel which takes a stream and a cloned mutex runtime info
pub struct Miner {
    pub counter: usize,
    pub name: String,
    pub recipient: Recipient<Ping>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
}

impl Handler<Ping> for Miner {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}", self.name, msg.id);

            // wait 100 nanoseconds
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.recipient.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}