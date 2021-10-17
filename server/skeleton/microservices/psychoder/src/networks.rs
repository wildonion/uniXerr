





// NOTE - all layers or networks in deep learning must be an actor which will communicate with each other based on defined messages or events like their matrix multiplication from one layer to another layer
// NOTE - different kind of arguments passing structure with arbitrary numbers of them using macros 
// EXAMPLE - let network = model!(mlp_1(20) -> mlp_2(10) -> cnn(3, 16, 2, 5, 1) -> mlp_3(10))
// https://github.com/actix/actix
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes





pub mod cnn;
pub mod mlp;
pub mod graph;
pub mod transformers;
pub mod lstm;
pub mod gan;
pub mod vae;
use std::ops::Deref;

use crate::networks::mlp::Linear;
use crate::networks::cnn::Conv2d;




pub enum NetworkType{
    Linear(Linear),
    Conv2d(Conv2d),
}

pub struct Model{
    pub networks: Vec<NetworkType>,
    pub is_training: bool,
    pub epochs: u16,
    pub batch_size: u16,
    pub device: String,
}

impl Model{

    pub fn train(self){ //-- `&self` has an anonymous lifetime `'_` because of unknown lifetime of enum networks field which contains multiple different type of networks 
        for network in self.networks{
            match network{
                NetworkType::Linear(linear_net) => {
                    tokio::spawn(async move{
                        //-- TODO - training Linear layer
                        // ...
                        let loss = linear_net.forward().await;
                    });
                },
                NetworkType::Conv2d(conv2d_net) => {
                    tokio::spawn(async move{
                        //-- TODO - training Conv2d layer
                        // ...
                        let loss = conv2d_net.forward().await;
                    });
                }   
            }
        }
    }
    
    
    pub fn predict(self){}
}

impl Default for Model{
    fn default() -> Self{
        Model{
            networks: Vec::new(),
            is_training: false,
            epochs: 0,
            batch_size: 0,
            device: "cpu".to_string(),
        }
    }
}
