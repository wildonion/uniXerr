








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
    pub epochs: u16,
    pub batch_size: u16,
}

impl<N> Model<N>{

    pub fn train(&self){ // NOTE - S is sample or mini_bactch generic type and L is labels generic type;
        // TODO - train the model using tokio::spawn() or mpsc job queue channel from scratch
        // ...
    }
    
    
    pub fn predict(){}
}




