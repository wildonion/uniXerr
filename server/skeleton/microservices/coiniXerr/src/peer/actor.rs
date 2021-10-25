





// NOTE - actix actors are used for sending messages and events through their address (Addr object) instead of blocking the local thread for mutex acquisition using mpsc channel
// NOTE - all actix actors are on top of tokio in which every future task like actors communication events and messages will be handled by mpsc job queue channel and multithreading patterns
// NOTE - mpsc channel can be used to communicate between threads while we're using a thread pool to mutate a data structure by locking on the data and blocking the local thread to acquires the mutex and prevent other thread from mutating and locking it at a same time to avoid being in dead lock situation
// NOTE - the sender of mpsc channel can be owned by multiple threads but the receiver can only be owned by only one thread at a time, that's because it's called multi producer and single consumer  
// NOTE - we have to clone the receiver for passing between multiple threads and for mutating what's in it we have to put it inside a Mutex to insure that only one thread can change the content of the receiver at a time
// NOTE - mutex acquisition is done by waiting on the receiver until a job or task becomes available to down side of the channel then locking on the receiver to acquire the mutex
// NOTE - multiple producers or workers own the receiver (Ac<T>) and single consumer or worker get the job at a time from the receiver (Mutex<T>)
// NOTE - we'll spawn some threads inside a pool like four threads for every process like socket connection to schedule and solve all its incoming tasks 
// NOTE - a job or a task coming from a process must be solved inside a thread which is selected from the pool based on the availability of threads
// NOTE - tasks or jobs of a process can be a massive computational data or a bytes of a file from evey connection 
// NOTE - tasks or jobs of a process can be solved simultaneously inside one of the opened threads using a queue channel like mpsc
// NOTE - tasks or jobs of a process can be sent to multiple threads using sender of mpsc channel and only one thread can solve it at a time using the receiver of mpsc job q channel
// NOTE - if a thread was busy another thread will be spawned to handle new task or job coming from the process
// NOTE - task scheduler is done through threads communication using either job queue channel or actors message passing to avoid dead lock and race condition






use actix::prelude::*; //-- loading actix actors and handlers
use std::time::Duration;
use liby;



#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct Miner {
    counter: usize,
    name: String,
    recipient: Recipient<Ping>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
}

// simple message handler for Ping message
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

pub async fn run() {
    let mut system = System::new();

    // To get a Recipient object, we need to use a different builder method
    // which will allow postponing actor creation
    let addr = system.block_on(async {
        Miner::create(|ctx| {
            // now we can get an address of the first actor and create the second actor
            let addr = ctx.address();

            let addr2 = Miner {
                counter: 0,
                name: String::from("Miner 2"),
                recipient: addr.recipient(),
            }
            .start();

            // let's start pings
            addr2.do_send(Ping { id: 10 });

            // now we can finally create first actor
            Miner {
                counter: 0,
                name: String::from("Miner 1"),
                recipient: addr2.recipient(),
            }
        });
    });

    system.run();
}
