




// example command of training an async and multithreading version of mlp network
// CLI COMMAND : ./psychoder --network mlp --dataset /path/to/dataset --device cpu --epochs 200 --batch-size 64 --output-neurons 10



use utilser::Info;
use crate::handlers::nn::MetaData;





impl Info for MetaData {
    fn who(&self) -> String{
        format!("Neuron {} Has id {}", self.neuron_name.to_string(), self.id.to_string())
    }
}