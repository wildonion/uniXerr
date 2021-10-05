




// example command of training an async and multithreading version of mlp network
// CLI COMMAND : ./psychoder --network mlp --dataset /path/to/dataset --device cpu --epochs 200 --batch-size 64 --output-neurons 10



use utilser::Info;
use crate::schemas::brain::{Neuron, MetaData};




impl Info for MetaData {
    fn who(&self) -> String{
        format!("Neuron [{}] fired at time [{}] with id [{}]", self.time, self.neuron_name.to_string(), self.id.to_string())
    }
}


pub struct Linear{
    pub neurons: Vec<Neuron>, //-- neurons are features or columns of our input data matrix
}