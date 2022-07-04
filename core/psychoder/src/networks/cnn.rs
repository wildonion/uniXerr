






use std::sync::{Arc, mpsc::channel};
use actix::prelude::*;
use crate::schemas::brain::Neuron;
use crate::schemas::brain::Synapse; //-- based on the orphan rule this should be used in here cause the communication() method of each neuron is implemented inside the Synapse trait - items from traits can only be used if the trait is in scope





pub struct Conv2d{
    pub neural_circuit: Vec<Neuron>, //-- neurons are features or columns of our input data matrix
}

impl Conv2d{
    
    pub async fn forward(&self, x_train: Arc<Vec<Vec<f64>>>) -> f64{
        let mut cnn_neural_circuit = self.neural_circuit.iter();
        cnn_neural_circuit.next().unwrap().communicate(cnn_neural_circuit.next()); //-- communicate method through synapse trait
        let loss = 0.3535;
        loss
    }

    pub async fn backward(&self, loss: f64){
        //-- without &mut self would be an associated function not a method
        // TODO - 
        // ...
    }
}


impl Actor for Conv2d{
    type Context = Context<Conv2d>;
}