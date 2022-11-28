





//// ----------------------
//// Hoopoe Account Stream
//// ----------------------





use crate::*;












#[derive(Debug)]    
pub enum Topic{
    Hoop, //// hoops are the musiem playlist
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
    pub channels: Vec<Channel>, //// rabbitmq channels
    pub queues: Vec<Queue>, //// rabbitmq queues
} 

impl Account{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
    //// this method will build the connection to the broker and rabbitmq channels to talk to publishers and subscribers
    pub async fn new(broker_addr: &str, n_channels: u16, acc_id: String) -> Self{ 

        // ----------------------------------------------------------------------
        //                     CONNECTING TO RABBITMQ BROKER
        // ----------------------------------------------------------------------
        
        let conn = Connection::connect(&broker_addr, ConnectionProperties::default().with_default_executor(10)).await.unwrap();
        info!("➔ 🟢 ⛓️ hoopoe is now connected to the broker");
        
        // ----------------------------------------------------------------------
        //            CREATING RABBITMQ CHANNELS TO TALK TO THE BROKER
        // ----------------------------------------------------------------------
        
        let mut channels = Vec::<Channel>::new(); //// producers and consumers must talk to the channel first
        for i in 0..n_channels{
            channels.push(
                conn.create_channel().await.unwrap()
            );
        }
        info!("➔ 🟢 🕳️ hoopoe channels created susscessfully");
        Self{ //// returning a new instance of the Account also is Self is the complete type of the Account<T> not just the constructor or Account
            account_id: acc_id,
            channels,
            queues: Vec::new(), // or vec![]
        }
    }

    //// this method will build the queue from the passed in name
    pub async fn make_queue(&mut self, name: &str) -> Self{

        // ----------------------------------------------------------------------
        //             BUILDING THE HOOP QUEUE USING THE FIRST CHANNEL
        // ----------------------------------------------------------------------

        // let Account { account_id, channels, queues } = self; //// unpacking the self into the Account struct; by defining the self as mutable every field of the unpacked self will be mutable
        
        //// consider the first one as the publisher channel and the second as the subscriber channel
        let first_channel = self.channels[0].clone();
        let mut queues = self.queues.clone();
        queues.push(
            first_channel.queue_declare(
                            name, //// defining the queue with passed in name; this can be later used to subscribe messages to the buffer of this queue 
                            QueueDeclareOptions::default(), 
                            FieldTable::default(),
                        ).await.unwrap()
        );
        
        info!("➔ 🟢🎣 hoop queue created susscessfully");
        
        Self{
            account_id: self.account_id.clone(), //// cannot move out of `self.account_id` which is behind a mutable reference 
            channels: self.channels.clone(), //// cannot move out of `self.channels` which is behind a mutable reference
            queues,
        }

    
    }

    //// this method will build the consumer from the second channel 
    //// and wait for each message to be consumed from the specified queue
    //// until all the message gets deliverred.
    pub async fn subscribe(&self, queue: &str){

        // -------------------------------------------------------------------------------------------------------------
        //             BUILDING THE CONSUMER FROM THE SECOND CHANNEL TO SUBSCRIBE TO THE PUBLISHED MESSAGES  
        // -------------------------------------------------------------------------------------------------------------

        //// we're using Arc to clone the second_channel since Arc is to safe for sharing the type between threads 
        info!("➔ 🟢📩 subscribing from the second channel to the published messages from the [{}] queue", queue);
        let second_channel = self.channels[1].clone(); //// we've used the second channel to use its consumer to get all message deliveries
        let consumer_channel = Arc::new(&second_channel); //// putting the borrowed form of second_channel inside the Arc (since we want to clone it later for ack processes) to prevent ownership moving since we want to consume messages inside a worker threadpool
        let consumer = consumer_channel
                            .clone()
                            .basic_consume( //// it'll return the consumer which will be used to get all the message deliveries from the specified queue
                                queue, //// the quque that we've just built and want to get all messages which are buffered by the publisher 
                                "hoop_consumer",  
                                BasicConsumeOptions::default(),
                                FieldTable::default(),
                            ).await.unwrap();

        // ----------------------------------------------------------------------
        //           GETTING ALL THE DELIVERIES OF THE CONSUMED MESSAGES
        // ----------------------------------------------------------------------
        let second_channel = second_channel.clone(); //// cloning the second channel to prevent ownership moving since we're moving the channel into the tokio spawn scope
        tokio::spawn(async move{ //// spawning async task that can be solved inside the tokio green threadpool under the hood which in our case is consuming all the messages from the passed in queue buffer  
            info!("➔ 🪢🛀🏽 consuming deliveries inside tokio worker green threadpool");
            consumer
                .for_each(move |delivery|{ //// awaiting on each message delivery 
                    let delivery = delivery.expect("Error in consuming!").1;
                    second_channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default()) //// acknowledging the messages using their delivery tags
                        .map(|_| ())
                }).await
        });

    }

    //// this method will build the producer from the first channel 
    //// and produce payloads based on the passed in criteria to send them 
    //// to the specified routing key which in this case is our queue name.
    pub async fn publish(&self, criteria: u16, exchange: &str, routing_key: &str){

        // -----------------------------------------------------------------------------------------------------------------
        //             BUILDING THE PUBLISHER FROM THE FIRST CHANNEL TO PUBLISH MESSAGES TO THE SPECIFIED QUEUE  
        // -----------------------------------------------------------------------------------------------------------------

        info!("➔ 🟢🛰️ publishing messages from the first channel to the [{}] queue", exchange);
        let first_channel = self.channels[0].clone();
        for n in 0..criteria{ //// sending the payload `criteria` times
            let message = format!("[{:?} ➔ {}-th musiem]", Topic::Hoop, n); //// enum field first will be converted into String then into utf8 bytes
            let payload = message.as_bytes(); //// converting the message to utf8 bytes
            info!("➔ 🟢📦 iteration [{}], publishing payload", n);
            let confirm = first_channel
                                        .basic_publish(
                                            exchange, //// exchange receives message from publishers and pushes them to queues
                                            routing_key, //// this is the routing key and is the address that the message must be sent to like the queue name in which the messages will be buffered inside  
                                            BasicPublishOptions::default(),
                                            payload.to_vec(), //// the payload that must be published
                                            BasicProperties::default(),
                                        )
                                        .await.unwrap()
                                        .await.unwrap();
            assert_eq!(confirm, Confirmation::NotRequested);
        }

    }


} 