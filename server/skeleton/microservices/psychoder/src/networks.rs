





// NOTE - declarative macros are written using macro_rules!
// NOTE - procedural macros are custom derive: #[derive(CustomDerive)], attribute-like: #[CustomAttribute], and function-like: custom!(...)
// NOTE - procedural macros enables other prgrammers to use our trait on our own struct
// NOTE - Fn trait is an object safe trait, because of unknown size at compile time it needs to be inside the Box<dyn Trait_Name>
// NOTE - macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. 
// NOTE - function gets called at runtime and a trait needs to be implemented at compile time.
// NOTE - for those types specially concrete types like traits which don't have size at compile time means they are not bounded to Sized trait, we have to point them using a pointer like Box<dyn Trait> or &dyn Trait
// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes





pub mod cnn;
pub mod mlp;
pub mod gnn;
pub mod transformers;
pub mod lstm;
pub mod gan;
pub mod vae;
use std::sync::Arc;
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

    pub async fn train(self, x_train: Vec<Vec<f64>>){ //-- `&self` has an anonymous lifetime `'_` because of unknown lifetime of enum networks field which contains multiple different type of networks 
        let arc_x_train = Arc::new(x_train);
        tokio::spawn(async move{ //-- spawning an async task in the background on a single thread using tokio green threads
            for network in self.networks{
                let cloned_x_train = Arc::clone(&arc_x_train);
                match network{
                    NetworkType::Linear(linear_net) => {
                        //-- TODO - training Linear layer
                        // ...
                        let loss = linear_net.forward(cloned_x_train).await;
                    },
                    NetworkType::Conv2d(conv2d_net) => {
                        //-- TODO - training Conv2d layer
                        // ...
                        let loss = conv2d_net.forward(cloned_x_train).await;
                    }   
                }
            }
        });
    }
    
    pub async fn predict(self){}
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
