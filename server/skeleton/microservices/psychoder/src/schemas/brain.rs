





use serde::{Serialize, Deserialize};
use uuid::Uuid;




pub struct BrainContext<Neuron>(pub Vec<Neuron>, pub i64);
impl<Neuron> Default for BrainContext<Neuron>{
    fn default() -> Self{
        BrainContext(vec![], chrono::Local::now().naive_local().timestamp())
    }
}

pub trait Synapse{
    fn communicate() -> Self; //-- this is not object safe trait cause it's returning an associated type which is Self
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neuron{
    pub id: Uuid,
    pub name: String,
    pub time: i64,
}

impl Synapse for Neuron{
    fn communicate() -> Self{
        Neuron{
            id: Uuid::new_v4(),
            name: "Genesis".to_string(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}

impl Default for Neuron{
    fn default() -> Self{
        Neuron{
            id: Uuid::new_v4(),
            name: "AJG7$%".to_string(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}
