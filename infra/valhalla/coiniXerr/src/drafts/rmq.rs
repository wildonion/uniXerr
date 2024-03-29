// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                          RabbitMQ RMQAccount Stream Contains Publisher and Subscriber using lopin crate
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

/*


    mq clients in rust and js
        | 
        -------- coiniXerr mq producer and consumer actor streamer -------- conse hyper server 
                        |                             |
                        |                             -------- mongodb
                        |
                        <---tcp socket--> |broker actor streamer on VPS <---routing channel exchange--> job or task queue buffer| 
                                                                                                            |
                                                                                                            |
                                                                                                            |
                                                                                                            <---mpsc channel---> worker threadpools




    https://www.cloudamqp.com/blog/part1-rabbitmq-for-beginners-what-is-rabbitmq.html#exchanges


    • Producer: Application that sends the messages.
    • Consumer: Application that receives the messages.
    • Queue: Buffer that stores messages.
    • Message: Information that is sent from the producer to a consumer through RabbitMQ.
    • Connection: A TCP connection between your application and the RabbitMQ broker.
    • Channel: A virtual connection inside a connection. When publishing or consuming messages from a queue - it's all done over a channel.
    • Exchange: Receives messages from producers and pushes them to queues depending on rules defined by the exchange type. To receive messages, a queue needs to be bound to at least one exchange.
    • Binding: A binding is a link between a queue and an exchange.
    • Routing key: A key that the exchange looks at to decide how to route the message to queues. Think of the routing key like an address for the message.
    • AMQP: Advanced Message Queuing Protocol is the protocol used by RabbitMQ for messaging.



    mq is actually a tcp socket channel based on actor desing pattern that will send and receive buffers like any other socket channels
    but the only difference between others is it can manage incoming payloads in a specific manner like:
        • it uses an async job or task queue like mpsc jobq channel and celery algos to communicating between actors' threads (send and receive tasks and messages between their worker threadpools)  
        • it has a batch handler which means it can take a batch of tasks and publish them to the producers from the queue
        • receiving only a specific message on a specific topic (receivers can only subscribe to a specific topic)
        • schduling a message to be sent later using a task queue handler
        • schduling a message to be received at a specific condition using a task queue handler
        • sending and broadcasting message only to specific receivers 
        • handle (send and receive) tasks and messages asyncly inside a threadpool
        • buffering messages inside a queue to send them once the receiver gets backed online





        

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                  lopin rmq setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈      
    ////
    ////         publisher/subscriber app (rust or js code) 
    ////                      |
    ////                       ---- tcp socket
    ////                                       |
    ////                              rpc broker channels
    ////                                       |
    ////                                        --------- exchange
    ////                                                     |
    ////                             routing key ------- |binding| ------- routing key
    ////                                                     |
    ////                                             jobq queue buffer
    ////                                                     |
    ////                                                      --------- worker threadpool 
    ////
    //// ➔ publishers (rust or js code) which is connected to the mq broker can publish messages to a channel 
    ////    from there (inside the broker channels) messages will be buffered inside a specific queue.
    //// ➔ subscribers (rust or js code) want to subscribe to a specific message in which they must talk to a channel
    ////    then the channel will talk to the broker to get the message from a specific queue.
    //// ➔ rabbitmq uses queues instead of topics means that we can get all messages from a specific queues 
    ////    instead of subscribing to a specific topic by doing this all consumers can subscribe to a specific queue.  
    //// ➔ there might be multiple channels each of which are able to talk to a specific queue to get the buffered message from there.

    let coiniXerr_account_id = Uuid::new_v4().to_string();
    let mut account = RMQAccount::new(
                                    &ampq_addr,
                                    2, 
                                    coiniXerr_account_id
                                ).await;
                                
                                
                                
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////         making queues, publish and subscribe
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    account //// making the hoop queue for publishing and subscribing process
        .make_queue("hoop")
        .await;
        
    account //// the publisher could be another app written in another lang
    .publish(10, "", "hoop") //// publishing 10 times on the passed in queue
        .await;
        
    account //// the subscriber could be another app written in another lang
        .subscribe("hoop") //// subscribing to the hoop queue
        .await;
        








        
*/
        
// #[derive(Debug)]    
// pub enum Topic{
//     Hoop,
//     ReHoop,
//     Mention,
//     HashTag,
//     Like,
//     RMQAccountInfo,
// }   

// //// if Clone trait is not implemented for a type and that type is also a field of a structure we can't have &self in
// //   structure methods since using a shared reference requires Clone trait be implemented for all types of the structure 
// //   otherwise we can't share none of the fields of the structure and by calling a method of the structure on the instance
// //   the instance will be no longer valid and be moved.
// //// if the first param of methods was &self that means the instance is behind a shared reference
// //// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
// //// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all
// //// hence we can't have &self as the first param.
// pub struct RMQAccount{ //// RMQAccount is the user that can publish and subscribe to the messages
//     pub account_id: String, //// this is the _id of the account that wants to publish messages
//     pub channels: Vec<Channel>, //// rabbitmq channels
//     pub queues: Vec<Queue>, //// rabbitmq queues
// } 

// impl RMQAccount{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
//     //// this method will build the connection to the broker and rabbitmq channels to talk to publishers and subscribers
//     pub async fn new(broker_addr: &str, n_channels: u16, acc_id: String) -> Self{ 

//         // ----------------------------------------------------------------------
//         //                     CONNECTING TO RABBITMQ BROKER
//         // ----------------------------------------------------------------------
        
//         let conn = Connection::connect(&broker_addr, ConnectionProperties::default().with_default_executor(10)).await.unwrap();
//         info!("➔ 🟢 ⛓️ connected to the broker");
        
//         // ----------------------------------------------------------------------
//         //            CREATING RABBITMQ CHANNELS TO TALK TO THE BROKER
//         // ----------------------------------------------------------------------

//         let mut channels = Vec::<Channel>::new(); //// producers and consumers must talk to the channel first
//         for i in 0..n_channels{
//             channels.push(
//                 conn.create_channel().await.unwrap()
//             );
//         }
//         info!("➔ 🟢 🕳️ channels created susscessfully");
//         Self{ //// returning a new instance of the RMQAccount also is Self is the complete type of the RMQAccount<T> not just the constructor or RMQAccount
//             account_id: acc_id,
//             channels,
//             queues: Vec::new(), // or vec![]
//         }
//     }

//     //// this method will build the queue from the passed in name
//     pub async fn make_queue(&mut self, name: &str) -> Self{

//         // ----------------------------------------------------------------------
//         //             BUILDING THE HOOP QUEUE USING THE FIRST CHANNEL
//         // ----------------------------------------------------------------------

//         // let RMQAccount { account_id, channels, queues } = self; //// unpacking the self into the RMQAccount struct; by defining the self as mutable every field of the unpacked self will be mutable
        
//         //// consider the first one as the publisher channel and the second as the subscriber channel
//         let first_channel = self.channels[0].clone();
//         let mut queues = self.queues.clone();
//         queues.push(
//             first_channel.queue_declare(
//                             name, //// defining the queue with passed in name; this can be later used to subscribe messages to the buffer of this queue 
//                             QueueDeclareOptions::default(), 
//                             FieldTable::default(),
//                         ).await.unwrap()
//         );
        
//         info!("➔ 🟢🎣 queue created susscessfully");
        
//         Self{
//             account_id: self.account_id.clone(), //// cannot move out of `self.account_id` which is behind a mutable reference 
//             channels: self.channels.clone(), //// cannot move out of `self.channels` which is behind a mutable reference
//             queues,
//         }

    
//     }

//     //// this method will build the consumer from the second channel 
//     //// and wait for each message to be consumed from the specified queue
//     //// until all the message gets deliverred.
//     pub async fn subscribe(&self, queue: &str){

//         // -------------------------------------------------------------------------------------------------------------
//         //             BUILDING THE CONSUMER FROM THE SECOND CHANNEL TO SUBSCRIBE TO THE PUBLISHED MESSAGES  
//         // -------------------------------------------------------------------------------------------------------------

//         //// we're using Arc to clone the second_channel since Arc is to safe for sharing the type between threads 
//         info!("➔ 🟢📩 subscribing from the second channel to the published messages from the [{}] queue", queue);
//         let second_channel = self.channels[1].clone(); //// we've used the second channel to use its consumer to get all message deliveries
//         let consumer_channel = Arc::new(&second_channel); //// putting the borrowed form of second_channel inside the Arc (since we want to clone it later for ack processes) to prevent ownership moving since we want to consume messages inside a worker threadpool
//         let consumer = consumer_channel
//                             .clone()
//                             .basic_consume( //// it'll return the consumer which will be used to get all the message deliveries from the specified queue
//                                 queue, //// the quque that we've just built and want to get all messages which are buffered by the publisher 
//                                 format!("{} consumer", queue).as_str(),  
//                                 BasicConsumeOptions::default(),
//                                 FieldTable::default(),
//                             ).await.unwrap();

//         // ----------------------------------------------------------------------
//         //           GETTING ALL THE DELIVERIES OF THE CONSUMED MESSAGES
//         // ----------------------------------------------------------------------
//         let second_channel = second_channel.clone(); //// cloning the second channel to prevent ownership moving since we're moving the channel into the tokio spawn scope
//         tokio::spawn(async move{ //// spawning async task that can be solved inside the tokio green threadpool under the hood which in our case is consuming all the messages from the passed in queue buffer  
//             info!("➔ 🪢🛀🏽 consuming deliveries inside tokio worker green threadpool");
//             consumer
//                 .for_each(move |delivery|{ //// awaiting on each message delivery 
//                     let delivery = delivery.expect("Error in consuming!").1;
//                     second_channel
//                         .basic_ack(delivery.delivery_tag, BasicAckOptions::default()) //// acknowledging the messages using their delivery tags
//                         .map(|_| ())
//                 }).await
//         });

//     }

//     //// this method will build the producer from the first channel 
//     //// and produce payloads based on the passed in criteria to send them 
//     //// to the specified routing key which in this case is our queue name.
//     pub async fn publish(&self, criteria: u16, exchange: &str, routing_key: &str){

//         // -----------------------------------------------------------------------------------------------------------------
//         //             BUILDING THE PUBLISHER FROM THE FIRST CHANNEL TO PUBLISH MESSAGES TO THE SPECIFIED QUEUE  
//         // -----------------------------------------------------------------------------------------------------------------

//         info!("➔ 🟢🛰️ publishing messages from the first channel to the [{}] queue", exchange);
//         let first_channel = self.channels[0].clone();
//         for n in 0..criteria{ //// sending the payload `criteria` times
//             let message = format!("[{:?} ➔ {}-th]", Topic::Hoop, n); //// enum field first will be converted into String then into utf8 bytes
//             let payload = message.as_bytes(); //// converting the message to utf8 bytes
//             info!("➔ 🟢📦 iteration [{}], publishing payload", n);
//             let confirm = first_channel
//                                         .basic_publish(
//                                             exchange, //// exchange receives message from publishers and pushes them to queues by using binders and routing keys
//                                             routing_key, //// this is the routing key and is the address that the message must be sent to like the queue name in which the messages will be buffered inside  
//                                             BasicPublishOptions::default(),
//                                             payload.to_vec(), //// the payload that must be published
//                                             BasicProperties::default(),
//                                         )
//                                         .await.unwrap()
//                                         .await.unwrap();
//             assert_eq!(confirm, Confirmation::NotRequested);
//         }

//     }


// } 

// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
//                    RabbitMQ RMQAccount Stream Contains Publisher and Subscriber rabbitmq_stream_client crate
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

// #[derive(Debug)]    
// pub enum Topic{
//     Hoop,
//     ReHoop,
//     Mention,
//     HashTag,
//     Like,
//     RMQAccountInfo,
// }   

// //// if Clone trait is not implemented for a type and that type is also a field of a structure we can't have &self in
// //   structure methods since using a shared reference requires Clone trait be implemented for all types of the structure 
// //   otherwise we can't share none of the fields of the structure and by calling a method of the structure on the instance
// //   the instance will be no longer valid and be moved.
// //// if the first param of methods was &self that means the instance is behind a shared reference
// //// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
// //// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all
// //// hence we can't have &self as the first param.
// pub struct RMQAccount{ //// RMQAccount is the user that can publish and subscribe to the messages
//     pub account_id: String, //// this is the _id of the account that wants to publish messages
//     pub env: Environment, //// the rabbitmq environemt which is used to publish or subscribe
//     pub producer: Option<Producer<Dedup>>, //// Clone trait is not implemented for the DedUp thus we can't clone or copy this field
//     pub consumer: Option<Consumer>,
// } 

// impl RMQAccount{ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 
    
//     pub async fn new(env: Environment, acc_id: String) -> Self{
//         Self{
//             account_id: acc_id,
//             env,
//             producer: None,
//             consumer: None,
//         }
//     }

//     pub async fn build_producer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

//         info!("➔ 🟢 building hoopoe producer");

//         let prod = self.env
//                 .producer()
//                 .name("hoopoe_publisher")
//                 .build("hoopoe_producer_stream")
//                 .await
//                 .unwrap();
        
//         Self{
//             account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
//             env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
//             producer: Some(prod),
//             consumer: self.consumer, //// since self is not a shared reference thus we can move it into new scope
//         }

//     }

//     pub async fn build_consumer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

//         info!("➔ 🟢 building hoopoe consumer");

//         let cons = self.env
//                 .consumer()
//                 .build("hoopoe_consumer_stream")
//                 .await
//                 .unwrap();
        
//         Self{
//             account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
//             env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
//             producer: self.producer, //// since self is not a shared reference thus we can move it into new scope
//             consumer: Some(cons), 
//         }

//     }

//     pub async fn publish(producer: Option<Producer<Dedup>>, topic: Topic, message: String) -> Option<Producer<Dedup>>{ //// we're returning the producer for later calls since once the producer gets passed to this method it'll be moved and there will be no longer available 

//         //        to be executed from the hoops queue buffer until the consumer is backed on line
//         // ...
//         let body = match topic{
//             Hoop => format!("hooping: {}", message), 
//             ReHoop => format!("rehooping: {}", message), 
//             Mention => format!("Mentioning: {}", message),
//             HashTag => format!("Hashtaging: {}", message),
//             Like => format!("Liking: {}", message),
//         };

//         if let Some(mut prod) = producer{
//             info!("➔ 🟢 publishing");
//             prod
//                 .send(Message::builder().body(body).build(), |_| async move{})
//                 .await
//                 .unwrap();            
//             Some(prod)
//         } else{
//             None
//         }        

//     }

//     pub async fn subscribe(consumer: Option<Consumer>){

//         let mut consumer = consumer.unwrap(); //// defining the consumer as mutable since receiving and reading from the consumer is a mutable process and needs the futures::StreamExt trait to be imported 
//         tokio::spawn(async move{
//             info!("➔ 🟢 subscribing");
//             while let Some(delivery) = consumer.next().await{ //// streaming over the consumer to receive all the messages comming from the producer while there is some delivery
//                 info!("Received message {:?}", delivery);
//             }
//         });

//     }

//     pub async fn close_producer(producer: Option<Producer<Dedup>>){
//         if let Some(prod) = producer{
//             info!("➔ 🟢 closing hoopoe producer");
//             prod
//                 .close().await
//                 .unwrap();
//         }
//     }

//     pub async fn close_consumer(consumer: Option<Consumer>){
//         if let Some(cons) = consumer{
//             info!("➔ 🟢 closing hoopoe consumer");
//             let consumer_handler = cons.handle();
//             consumer_handler
//                     .close().await
//                     .unwrap();
//         }
//     }

// }