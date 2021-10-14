






use crate::schemas::brain::Neuron;
use actix::prelude::*;





pub struct Conv2d{
    pub neurons: Vec<Neuron>, //-- neurons are features or columns of our input data matrix
}

impl Conv2d{
    pub async fn forward() -> f64{
        let loss = 0.3535;
        loss
    }
    pub async fn backward(loss: f64){}
}


impl Actor for Conv2d{
    type Context = Context<Conv2d>;
}