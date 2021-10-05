





// NOTE - in order to move all data through the socket or http protocol they must be encoded from struct and converted to &[u8] serde codec serialize 
// NOTE - in order to get the data from the socket or http protocol they must be decoded from &[u8] to struct using serde codec deserialize
// TODO - psychodec, a codec for streaming of binary data (like from a source contains training data) like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice or mapping struct into &[u8] bytes
// TODO - write proc macros for all nn variants
// EXAMPLE - let network = model!(mlp_1(20) -> mlp_2(10) -> cnn(3, 16, 2, 5, 1) -> mlp_3(10))
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1
// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
// https://stackoverflow.com/questions/2490912/what-are-pinned-objects
// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
// https://github.com/zupzup/warp-websockets-example
// https://github.com/tokio-rs/tokio/tree/master/examples
// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://danielkeep.github.io/tlborm/book/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://docs.rs/tokio/1.7.1/tokio/sync/index.html
// https://docs.rs/tokio-stream/0.1.7/tokio_stream/
// https://doc.rust-lang.org/std/pin/index.html
// https://doc.rust-lang.org/std/sync/struct.Arc.html
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/std/sync/struct.RwLock.html
// https://doc.rust-lang.org/std/cell/struct.RefMut.html
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// https://doc.rust-lang.org/std/rc/struct.Weak.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes
// https://github.com/wildonion/aravl/tree/master/microservices/device




use futures::{executor::block_on, join};
use std::thread;
use threadpool::ThreadPool;
use std::sync::{Arc, mpsc::channel};
use std::sync::mpsc::Receiver;



pub async fn async_training<S, L>(recv: Receiver::<S, L>){ // NOTE - S is sample or mini_bactch generic type and L is labels generic type
    

    // while let Some((mini_batch, label)) = recv.recv().await{ // TODO - get input data from a source like receiver
    // while let Some((mini_batch, label)) = batches.next().await{ // TODO - (mini_batch, label) is coming from a source in a live streaming manner

        // let cloned_model = model.clone();
        // let (sender, receiver) = channel();
        // let cloned_receiver = Arc::clone(receiver);
        // let mut children = Vec::new();
        // =======================================================================
        // tokio::spawn(async move{ // NOTE - passing a future into the tokio::spawn() which is based on job queue channel and multithreading concepts
        //     for e in 0..epochs{
        //         // cloned_model.lock.unwrap().train(mini_batch).await; 
        //     }
        // });
        
        
        // block_on(async move{ // NOTE - passing a future into the block_on() to solve the future by blocking the current thread
        //     for e in 0..epochs{
        //         let cloned_sender = sender.clone(); //-- clone() trait is implemented for sender object of mpsc channel
        //         let cloned_model = model.clone();
        //         children.push(pool.execute(move ||{ // NOTE - thread::spawn() takes closure as its parameter not an async move || {} cause async closures are not stable cause they don't have fixed size to pin them in memory to be a future object
        //             // TODO - run a closure task in each epoch to train the model inside a thread pool and send the calculated loss to down side of the channel 
        //             // ...
        //             // let training_loss = cloned_model.lock.unwrap().train(mini_batch).await;
        //             // cloned_sender.send(training_loss).unwrap();
        //         }));
        //     }
        // });
    //     // =======================================================================

    // }
    
}



pub mod cnn;
pub mod mlp;
pub mod graph;
pub mod transformers;
pub mod lstm;



use uuid::Uuid;



pub trait Synapse{
    fn communicate() -> Self;
}

pub trait Model{
    fn train();
    fn predict();
}



struct Neuron; //-- unit like struct
pub struct MetaData{
    pub id: Uuid,
    pub neuron_name: String,
}




impl Synapse for Neuron{ //-- it's like implementing a behaviour for a raw object without any meta data
    fn communicate() -> Self{ //-- this is not object safe trait cause it's returning an associated type which is Self
        Neuron
    }
}
impl Default for Neuron{
    fn default() -> Self{
        todo!()
    }
}


