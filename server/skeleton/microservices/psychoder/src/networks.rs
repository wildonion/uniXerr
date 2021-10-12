







// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
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
pub mod gan;
pub mod vae;





pub struct Model<N>{
    pub networks: Vec<N>,
    pub is_training: bool,
    pub epochs: u16,
    pub batch_size: u16,
    pub device: String,
}

impl<N> Model<N>{

    pub fn train(&self){
        // TODO - train the model using tokio::spawn()
        // ...
    }
    
    
    pub fn predict(){}
}

impl<N> Default for Model<N>{
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
