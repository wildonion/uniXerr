



// TODO - every layer must be an actor to send its Arc<Mutex<output_data_matrix>> from its free thread to the next layer actor free thread asyncly through its mpsc channel to lock on it 
// TODO - every layer inside the defined network is an actor which can send its output of size (BATCH X NEXT_LAYER_NEURONS) as the input of the next layer actor through the mpsc channle by scheduling a future object (async message) to be executed later and calculate the output of the next layer actor asyncly  
// TODO - send borsh encoded training data asyncly through the mpsc channel of each layer actor using SIMD pattern
// ...


pub mod cnn;
pub mod mlp;
pub mod gnn;
pub mod lstm;
pub mod ssl;
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
