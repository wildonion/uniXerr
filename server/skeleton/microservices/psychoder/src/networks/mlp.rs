




// example command of training an async and multithreading version of mlp network
// CLI COMMAND : ./psychoder --network mlp --dataset /path/to/dataset --device cpu --epochs 200 --batch-size 64 --output-neurons 10



use utilser::Info;
use crate::schemas::brain::Neuron;
use actix::prelude::*;




impl Info for Neuron{
    fn what(&self) -> String{
        format!("Neuron [{}] fired at time [{}] with id [{}]", self.time, self.name.to_string(), self.id.to_string())
    }
}


pub struct Linear{
    pub neurons: Vec<Neuron>, //-- neurons are features or columns of our input data matrix
}

impl Linear{
    pub async fn forward() -> f64{
        let loss = 0.3535;
        loss
    }
    pub async fn backward(loss: f64){}
}


impl Actor for Linear{
    type Context = Context<Linear>;
}