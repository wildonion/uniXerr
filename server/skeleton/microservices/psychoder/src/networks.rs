








// EXAMPLE - let network = model!(mlp_1(20) -> mlp_2(10) -> cnn(3, 16, 2, 5, 1) -> mlp_3(10))
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1






pub mod cnn;
pub mod mlp;
pub mod graph;
pub mod transformers;
pub mod lstm;
use futures::{executor::block_on, join};
use std::thread;
use std::sync::{Arc, mpsc::channel};
use std::sync::mpsc::Receiver;





pub struct Model<N>{
    pub networks: Vec<N>,
    pub is_training: bool,
}

impl<N> Model<N>{

    // pub async fn train<S, L>(recv: Receiver::<S, L>){ // NOTE - S is sample or mini_bactch generic type and L is labels generic type;
        

    //     // while let Some((mini_batch, label)) = recv.recv().await{ // TODO - get input data from a source like receiver
    //     // while let Some((mini_batch, label)) = batches.next().await{ // TODO - (mini_batch, label) is coming from a source in a live streaming manner

    //         // let cloned_model = model.clone();
    //         // let (sender, receiver) = channel();
    //         // let cloned_receiver = Arc::clone(receiver);
    //         // let mut children = Vec::new();
    //         // =======================================================================
    //         // tokio::spawn(async move{ // NOTE - passing a future into the tokio::spawn() which is based on job queue channel and multithreading concepts
    //         //     for e in 0..epochs{
    //         //         // cloned_model.lock.unwrap().train(mini_batch).await; 
    //         //     }
    //         // });
            
            
    //         // block_on(async move{ // NOTE - passing a future into the block_on() to solve the future by blocking the current thread
    //         //     for e in 0..epochs{
    //         //         let cloned_sender = sender.clone(); //-- clone() trait is implemented for sender object of mpsc channel
    //         //         let cloned_model = model.clone();
    //         //         children.push(pool.execute(move ||{ // NOTE - thread::spawn() takes closure as its parameter not an async move || {} cause async closures are not stable cause they don't have fixed size to pin them in memory to be a future object
    //         //             // TODO - run a closure task in each epoch to train the model inside a thread pool and send the calculated loss to down side of the channel 
    //         //             // ...
    //         //             // let training_loss = cloned_model.lock.unwrap().train(mini_batch).await;
    //         //             // cloned_sender.send(training_loss).unwrap();
    //         //         }));
    //         //     }
    //         // });
    //         // =======================================================================

    //     // }
    
    // }
    
    
    pub async fn predict(){}
}




