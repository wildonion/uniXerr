





// TODO - https://github.com/lucrussell/kafka-blockchain


use crate::schemas::block::Transaction;
use std::time::SystemTime;
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use liby;






pub async fn produce(brokers: &str){
    
    



    // NOTE - kafka => multiple cluster (datacenter or VPS) <-has-> nodes(multiple instances of kafka brokers or servers) <-has-> topics <-has-> partition replicas for each topic <-has-> buck of events inside each partition
    // NOTE - three replicas in kafka means there are three copies of each topics' partitions (buck of events) in each node (kafka broker)
    // NOTE - kafka partitions are created based on the hash of each event and events with similar hash will be inside a same partition so a topic is divided into one or more partitions
    // NOTE - the default number of partitions in kafka for each topic is 10.
    let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("⚠️ producer creation error");
    
    
    
    
    let producer = producer.clone(); //-- we're clonning the producer cause we want to move it between tokio::spawn() threads thus according to rust ownership we have to take a reference to the producer using clone() cause trait Copy is not imeplemented for that
    tokio::spawn(async move{ //-- tokio is based on event loop - tokio::spawn() takes a task of type future and shares it between multiple threads using its job queue channel protocol, so every type in the task must be Send + Sync and cloneable
        let mut i = 0_usize;
        loop {
            let transaction_event = Transaction::default();
            let topic = transaction_event.id.to_string(); //-- every transaction is a topic
            let transaction_event_json = serde_json::to_string_pretty(&transaction_event).expect("⚠️ failed to serialize transaction event"); //-- serializing the struct into json
            let transaction_data: Transaction = serde_json::from_str(&transaction_event_json).expect("⚠️ failed to deserialize transaction json"); //-- deserializing the json into struct
            let key = &i.to_string(); //-- setting the key for this event
            let devlivery_status = producer.send_result( //-- we're using FutureRecord for sending the message or the event asynchoronously to all consumers cause send_result() method takes a FutureRecord to send a message
            FutureRecord::to(&topic)
                        .key(key)
                        .payload(&transaction_event_json) //-- we can send serde json inside the payload
                        .headers(OwnedHeaders::new().add("wo_header_key", "wo_header_value"))
                        .timestamp(
                            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
                                Ok(n) =>  n.as_secs() as i64,
                                Err(_) => { panic!("SystemTime before UNIX EPOCH!") },
                            }
                        )
            );
            println!("[+] Delivery status for Player data with imei {} inside iteration key {} received", topic, i);
            match devlivery_status{ //-- devlivery_status is a Result of delivery future and in order to solve it we have to await on it 
                Ok(delivery) => {
                    let solved_delivery = delivery.await.unwrap().unwrap();
                    info!("[+] Delivery solved {:?}", solved_delivery);
                },
                Err(e) => {
                    error!("[!] Delivery error {:?}", e);
                }
            }

            i += 1;
        }
    });





}
