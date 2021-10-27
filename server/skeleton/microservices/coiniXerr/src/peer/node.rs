



// https://github.com/wildonion/aravl/tree/master/microservices/device/src
// https://github.com/actix/examples/blob/master/websockets/tcp-chat/src/codec.rs
// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
// https://stackoverflow.com/questions/2490912/what-are-pinned-objects
// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
// https://github.com/zupzup/warp-websockets-example
// https://github.com/tokio-rs/tokio/tree/master/examples
// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://danielkeep.github.io/tlborm/book/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://docs.rs/tokio/1.12.0/tokio/sync/index.html
// https://docs.rs/tokio-stream/0.1.7/tokio_stream/
// https://doc.rust-lang.org/std/pin/index.html
// https://doc.rust-lang.org/std/sync/struct.Arc.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/std/sync/struct.RwLock.html
// https://doc.rust-lang.org/std/cell/struct.RefMut.html
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// https://doc.rust-lang.org/std/rc/struct.Weak.html







use std::net::{TcpStream, Shutdown}; //-- these structures are not async; to be async in reading and writing from and to socket we must use tokio::net 
use actix::{*, prelude::*}; //-- loading actix actors and handlers for threads communication using their address and defined events 
use crate::peer::actor::{Miner, Ping};
use std::io::{Read, Write}; //-- Read and Write are traits which are implemented for an object of type TcpStream and based on orphan rule we must use them here to use the read() and write() method implemented for the object of TcpStream
use crate::schemas::{MetaData, RuntimeInfo};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;





pub async fn start_miner_actor(mut receiver: mpsc::Receiver::<(TcpStream, Arc<Mutex<RuntimeInfo>>)>){
    tokio::spawn(async move { //-- spawning a green thread based task of type future in the background on a single thread
        let mut transaction_buffer_bytes = [0 as u8; 1024];
        while let Some((mut stream, run_time_info)) = receiver.recv().await{ //-- receiving the stream and the cloned mutex runtime info into down side of channel to start a miner actor - stream must be mutable for reading and writing from and to socket  
            while match stream.read(&mut transaction_buffer_bytes){ //-- keep socket always open
                Ok(size) if size == 0 => false, //-- socket closed
                Ok(size) => {
                    // ---------------------------------------------------------------------------------------------
                    // ---------------------------------------------------------------------------------------------
                    // NOTE - in order to move all data through the socket or http protocol they must be encoded from struct and converted to &[u8] serde codec serialize 
                    // NOTE - in order to get the data from the socket or http protocol they must be decoded from &[u8] to struct using serde codec deserialize
                    // TODO - build miner actor with incoming transaction_buffer_bytes
                    // TODO - a codec like web::Payload and ws::Message for streaming of binary data like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice or mapping struct into &[u8] bytes based on big or little endian
                    // TODO - use a codec like serde_json::from_slice or Transaction struct (TransactionMem) to map and deserialize utf8 bytes from memory into the defined object
                    // ...  
                    stream.write(&transaction_buffer_bytes[0..size]).unwrap();
                    let miner = Miner::create(|ctx| {
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
                        let miner = Miner {
                            counter: 0,
                            name: String::from("Miner 1"),
                            recipient: addr2.recipient(),
                        };
                        miner
                    });
                    // ---------------------------------------------------------------------------------------------
                    // ---------------------------------------------------------------------------------------------
                    run_time_info.lock().unwrap().add(
                        MetaData{
                            address: stream.peer_addr().unwrap(),
                            buffer: transaction_buffer_bytes[0..size].to_owned(), //-- to_owned() creates owned data from borrowed data, usually by cloning
                            actor: miner,
                        }
                    );
                    true
                },
                Err(e) => {
                    println!("-> terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap(); //-- both the reading and the writing portions of the TcpStream should be shut down
                    false
                }
            } {} //-- it'll return true on its Ok() arm and false on its Err arm
        } 
    }); //-- awaiting on tokio::spawn() will block the current task which is running in the background
}





pub async fn subscribe(){
    // TODO - stream over incoming transactions from each kafka subscriber to mine and add them to the blockchain using coiniXerr consensus algorithm 
    // https://github.com/lucrussell/kafka-blockchain
    // ...
}