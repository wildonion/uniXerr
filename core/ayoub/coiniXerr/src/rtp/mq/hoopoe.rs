





// ---------------------------
//// hoopoe rabbitmq streams
// ---------------------------





use crate::*;
use utils::api; // macro apis for communicating with the ayoub hyper server hoopoe service like storing in db
use rtp::{
    rpc::server as rpc_server,
    wrtc::server as wrtc_server,
        ws::server as ws_server, // for chatapp
        socks::server as socks_server,
        p2p::udp::app as p2p_app,
    };
        













    
pub enum Topic{
    Hoop,
    ReHoop,
    Mention,
    HashTag,
    Like,
    AccountInfo,
}   

//// if Clone trait is not implemented for a type and that type is also a field of a structure we can't have &self in
//   structure methods since using a shared reference requires Clone trait be implemented for all types of the structure 
//   otherwise we can't share none of the fields of the structure and by calling a method of the structure on the instance
//   the instance will be no longer valid and be moved.
//// if the first param of methods was &self that means the instance is behind a shared reference
//// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
//// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all
//// hence we can't have &self as the first param.
pub struct Account{ //// Account is the user that can publish and subscribe to the messages
    pub account_id: String, //// this is the _id of the account that wants to publish messages
    pub env: Environment, //// the rabbitmq environemt which is used to publish or subscribe
    pub producer: Option<Producer<Dedup>>, //// Clone trait is not implemented for the DedUp thus we can't clone or copy this field
    pub consumer: Option<Consumer>,
} 

impl Account{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
    pub async fn new(env: Environment, acc_id: String) -> Self{
        Self{
            account_id: acc_id,
            env,
            producer: None,
            consumer: None,
        }
    }

    pub async fn build_producer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

        info!("➔ 🟢 building hoopoe producer");

        let prod = self.env
                .producer()
                .name("hoopoe_publisher")
                .build("hoopoe_producer_stream")
                .await
                .unwrap();
        
        Self{
            account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
            env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
            producer: Some(prod),
            consumer: self.consumer, //// since self is not a shared reference thus we can move it into new scope
        }

    }

    pub async fn build_consumer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

        info!("➔ 🟢 building hoopoe consumer");

        let cons = self.env
                .consumer()
                .build("hoopoe_consumer_stream")
                .await
                .unwrap();
        
        Self{
            account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
            env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
            producer: self.producer, //// since self is not a shared reference thus we can move it into new scope
            consumer: Some(cons), 
        }

    }

    pub async fn publish(producer: Option<Producer<Dedup>>, topic: Topic, message: String) -> Option<Producer<Dedup>>{ //// we're returning the producer for later calls since once the producer gets passed to this method it'll be moved and there will be no longer available 

        // TODO - ayoub server api calls maybe! for storing in db 
        // TODO - schedule old and new ones (from the last offline time) 
        //        to be executed from the hoops queue buffer until the consumer is backed on line
        // ...
        let body = match topic{
            Hoop => format!("hooping: {}", message), 
            ReHoop => format!("rehooping: {}", message), 
            Mention => format!("Mentioning: {}", message),
            HashTag => format!("Hashtaging: {}", message),
            Like => format!("Liking: {}", message),
        };

        if let Some(mut prod) = producer{
            info!("➔ 🟢 publishing");
            prod
                .send(Message::builder().body(body).build(), |_| async move{})
                .await
                .unwrap();            
            Some(prod)
        } else{
            None
        }        

    }

    pub async fn subscribe(consumer: Option<Consumer>){

        let mut consumer = consumer.unwrap(); //// defining the consumer as mutable since receiving and reading from the consumer is a mutable process and needs the futures::StreamExt trait to be imported 
        tokio::spawn(async move{
            info!("➔ 🟢 subscribing");
            while let Some(delivery) = consumer.next().await{ //// streaming over the consumer to receive all the messages comming from the producer while there is some delivery
                info!("Received message {:?}", delivery);
            }
        });

    }

    pub async fn close_producer(producer: Option<Producer<Dedup>>){
        if let Some(prod) = producer{
            info!("➔ 🟢 closing hoopoe producer");
            prod
                .close().await
                .unwrap();
        }
    }

    pub async fn close_consumer(consumer: Option<Consumer>){
        if let Some(cons) = consumer{
            info!("➔ 🟢 closing hoopoe consumer");
            let consumer_handler = cons.handle();
            consumer_handler
                    .close().await
                    .unwrap();
        }
    }

} 