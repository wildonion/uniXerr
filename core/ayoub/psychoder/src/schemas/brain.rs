



// https://github.com/wildonion/uniXerr/blob/master/core/recognizer/helper_board
// TODO - implement the coiniXerr::utils::scheduler::_async::Actor as uniXerrActor for Neuron in here
// TODO - implement all cognitive neuroscience concepts and schemas
// TODO - every neuron can be an actor (or the column of our input matrix) to construct the GNN in an async and multithreading manner in such a way that every actor which is a neuron can communicate with each other to get the data of the next or the last neuron asyncly 
// NOTE - a brain structure can have multiple interfaces like being in void and illusion abstract situations which can be implemented using traits 
// NOTE - train on ayoub PaaS events' phases which will be used to create behavioural graph of each player inside the game 
//          to show the tips and tricks of the new game based on the history or the behavioural graph of the player.
// ...  




use serde::{Serialize, Deserialize};
use uuid::Uuid;













// ---------------
//   INTERFACES
// ---------------
pub trait Void{
    type Illusion;
    type Pain;
}

pub trait Illusion{
    fn VisualCortex(&self) -> () {
    
    }
}

pub trait Synapse{
    fn communicate(&self, n: Option<&Neuron>) -> Self; //-- this is not object safe trait cause it's returning an associated type which is Self
}

///////
/// an abstract trait which rebuild the whole brain network, neuron connections, destroy consciousness and renew the self
///////
pub trait Reconnect{ //-- the following method must be invoked on taking mushrooms for a long period of time
    fn rebuild(&self) -> Self{ //-- it'll return the type that this trait will be implemented for 
         todo!()
    }
}

///////
/// an abstract trait which can echo the feeling of pain through the neurons to the whole brain
///////
pub trait Pain;


///////
/// an abstract trait which can buffer (store them) the suspended, unsolved and unaddressed data inside neurons
///////
pub trait Suspend; //-- a buffer contains unaddressed issues, feelings, pains and etc..




// ---------------
//   STRUCTURES
// ---------------
pub struct BrainContext<Neuron>(pub Vec<Neuron>, pub i64);


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neuron{
    pub id: Uuid,
    pub name: String,
    pub timestamp: i64,
    pub data: Vec<f32>,
}


impl<Neuron> Default for BrainContext<Neuron>{
    fn default() -> Self{
        BrainContext(vec![], chrono::Local::now().naive_local().timestamp())
    }
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
