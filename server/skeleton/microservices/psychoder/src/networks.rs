





pub mod cnn;
pub mod mlp;
pub mod graph;
pub mod transformers;
pub mod lstm;



use uuid::Uuid;



pub trait Synapse{
    fn communicate() -> Self;
}

pub trait Model{
    fn train();
    fn predict();
}



struct Neuron; //-- unit like struct
pub struct MetaData{
    pub id: Uuid,
    pub neuron_name: String,
}




impl Synapse for Neuron{ //-- it's like implementing a behaviour for a raw object without any meta data
    fn communicate() -> Self{ //-- this is not object safe trait cause it's returning an associated type which is Self
        Neuron
    }
}
impl Default for Neuron{
    fn default() -> Self{
        todo!()
    }
}


