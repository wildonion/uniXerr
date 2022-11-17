





//// onion actor




use crate::*;




pub struct Actor; // https://ryhl.io/blog/actors-with-tokio/
    
impl Actor{

    pub fn schedule<T>(){

        todo!() // ➔ schedule attack every 40 seconds after any error

    }


    pub fn broadcast(){
        
        todo!() // ➔ use tokio::sync::broadcast
    
    }
    
    pub fn run(){

        todo!()

    }

    fn handle_message(){

        todo!()

    }

    fn communicate(){ // each actor (neuron in uniXerr brain schema) can communicate with each other through some kinda channel (synapse)

        todo!() // ➔ use the jobqs the one inside jobq.rs for communicating between actors

    }


}