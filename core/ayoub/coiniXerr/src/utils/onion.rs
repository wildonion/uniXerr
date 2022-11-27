





/*

                ONION ACTOR TASKS

    - build an ssl (obfuscated connection) based (reverse) proxy server/client like nginx, pingora, v2ray protocols, softether and tor snowflake using hyper and tokio on layer4 based on tcp/udp/grpc/websocket and riker actors to bypass the cencorship using cryptography (zero proof of knowledge) and networking (ipatables) concepts on top of binary address transmition data protocol like onionary://0001010101:2349
    - build a load balancer and its config file engine (build the config file engine using macro syntax in utils.rs) runtime like k8s using cpu task (thread) scheduling algos (runtime scheduler will decide what task can be executed right now or later) like round robin dns, vector clock, event loop, simd vectorization and weighted round robin for ayoub using hyper and tokio on layer 1 and 2 (tcp/udp/grpc/websocket) based on and riker actors (every load balancer instance is an actor which can manage the server load instance inside their threadpool and every server instance can handle 144000 requests per second so with our load balance we must be able to spread the requests between each server node efficiently based on dns based round robin algo)
    - build something like ngrok using ssh tunnels and ipatables for rerouting traffics
    - object storage like MinIO and S3
    - download/upload manager like IDM/tus.io


*/


//// onion actor




use crate::*;






#[derive(Clone)]
pub struct Message;


#[derive(Clone)]
pub struct Publish{ //// use to publish message to the built channel
    pub msg: Message,
    pub topic: String,
} 


#[derive(Clone)]
pub struct Subscribe{ //// use to subscribe to a specific topic which has published by another actor
    pub actor: Actor,
    pub topic: String,
} 


#[derive(Clone)]
pub struct Actor; // https://ryhl.io/blog/actors-with-tokio/
    
impl Actor{

    pub async fn schedule(){

        todo!() // ➔ schedule attack every 40 seconds after any error

    }


    pub async fn broadcast(){
        
        todo!() // ➔ use tokio::sync::broadcast
    
    }
    
    pub async fn run(){

        todo!()

    }

    pub async fn handle_message(){

        // use jobq.rs algos to put the incoming tasks and messages inside the queue
        // ...

        todo!()

    }

    pub async fn communicate(){ // each actor (neuron in uniXerr brain schema) can communicate with each other through some kinda channel (synapse)

        todo!() // ➔ use the jobqs algos the ones inside jobq.rs for communicating between neuron actors

    }


}