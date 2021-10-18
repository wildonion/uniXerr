







use serde::{Serialize, Deserialize};
use uuid::Uuid;




pub struct BrainContext<Neuron>(pub Vec<Neuron>, pub i64);
impl<Neuron> Default for BrainContext<Neuron>{
    fn default() -> Self{
        BrainContext(vec![], chrono::Local::now().naive_local().timestamp())
    }
}

pub trait Synapse{
    fn communicate(&self, n: Option<&Neuron>) -> Self; //-- this is not object safe trait cause it's returning an associated type which is Self
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neuron{
    pub id: Uuid,
    pub name: String,
    pub timestamp: i64,
    pub data: Vec<f32>,
}

impl Synapse for Neuron{
    fn communicate(&self, next_neuron: Option<&Self>) -> Self{
        let next_neuron = next_neuron.unwrap();
        let new_neuron_data: Vec<f32> = self.data.iter().zip(next_neuron.data.iter()).map(|(x, y)| x * y).collect();
        Neuron{
            id: Uuid::new_v4(),
            name: "Genesis-AJG7$%-12".to_string(),
            timestamp: chrono::Local::now().naive_local().timestamp(),
            data: new_neuron_data
        }
    }
}

impl Default for Neuron{
    fn default() -> Self{
        Neuron{
            id: Uuid::new_v4(),
            name: "Genesis-AJG7$%".to_string(),
            timestamp: chrono::Local::now().naive_local().timestamp(),
            data: vec![0.0, 0.0]
        }
    }
}
