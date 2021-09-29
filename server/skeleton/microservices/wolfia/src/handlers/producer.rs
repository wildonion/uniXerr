






///////////// ===============================================================================================================================================================================================================================================
///////////// we are pinning the pointer of the Future object into memory cause we want to await on it later thus it shouldn't move from the memory by replacing with and pointing to a new value of a new variable
///////////// this is a wrapper around a kind of pointer which makes that pointer "pin" its value in place(stack or heap), preventing the value referenced by that pointer from being moved unless it implements Unpin.
///////////// EXAMPLE => type PFuture = Pin<Box<dyn Future<Output = Result<ServiceResponse, Error>>>>; 
/////////////
/////////////
///////////// we can’t just pass the receiver between multiple threads cause trait Clone which is a super trait of Copy is not implemented for the receiver thus we can’t clone it cause if a type is Copy its Clone needs to return *self.
///////////// the receiver of tokio mpsc channel is shareable between tokio::spawn() threads so we don’t need to take an atomic reference and put it inside the Mutex.
///////////// generally can't clone a data structure unless the trait Clone is implemented for that otherwise in order to move it between threads we have to clone it using Arc.
/////////////
/////////////
///////////// multiple producer means multiple threads own the receiver and single consumer means only one of them can mutate and get the job or task from the receiver at a time.
///////////// to fix the issue we have to take an atomic reference from the receiver using Arc in order to clone it for passing between multiple threads and for mutating it we have to 
///////////// put it inside a Mutex to insure that only one thread can change the content of the receiver at a time. this is done by waiting on the receiver until a job or task becomes 
///////////// available to the down side of the channel then locking on the receiver to acquire the mutex.
/////////////
/////////////
///////////// thread safe coding is about to putting the shareable receiver (cloned with Arc) inside a Mutex in order to lock on the incoming task from the sender to prevent other threads from mutating the task at a time.
///////////// clone data structure if you want to move them between threads so trait Clone must be implemented for them otherwise clone them using Arc.
///////////// every Copy type is also required to be Clone and if T: Copy, x: T, and y: &T, then let x = y.clone(); is equivalent to let x = *y;
///////////// when we derive a Copy implementation Clone is also required cause it's a super trait of Copy.
///////////// if a type imeplements trait Copy means we can clone it (cause trait Clone is a super trait of Copy) and also assign the variable into another one without losing the ownership of our variable
///////////// ===============================================================================================================================================================================================================================================



// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-unsafer-rs
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-garbage-rs


use crate::handlers::db::cass::schemas::player::Chat;
use crate::handlers::db::cass::establish as cass;
use std::time::SystemTime;
use log::{error, info};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use wolfielper;


pub async fn produce(brokers: &str){


    
    // NOTE - cloning db connections using Arc cause trait Copy and Clone is not implemented for them and they are not Sync and Send and safe to move between threads thus Arc do these things for us
    let cass_session = cass::connection().await.expect("⚠️ can't establish cassandra connection!"); //-- making cassandra pool of connections for selected node
    let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("⚠️ producer creation error");
    
    
    
    

    let cass_session = cass_session.clone(); //-- cloning the immutable cassandra session so we can share its ownership between multiple threads
    Chat::init(cass_session.clone()).await; //-- it'll create player_data column family if there is not any
    let producer = producer.clone(); //-- we're clonning the producer cause we want to move it between tokio::spawn() threads thus according to rust ownership we have to take a reference to the producer using clone() cause trait Copy is not imeplemented for that
    tokio::spawn(async move{ //-- tokio::spawn() takes a task of type future and shares it between multiple threads using its job queue channel protocol, so every type in the task must be Send + Sync and cloneable
        let mut i = 0_usize; // it's the default size of integer in rust which is i32
        loop {
            let player_event = Chat::last(cass_session.clone()).await; //-- getting the last data inserted into cassandra player_data column family
            let topic = player_event.id.to_string(); //-- getting its imei to set it as the topic for this event
            let player_event_json = serde_json::to_string_pretty(&player_event).expect("⚠️ failed to serialize player event"); //-- serializing the struct into json
            let player_data: Chat = serde_json::from_str(&player_event_json).expect("⚠️ failed to deserialize player json"); //-- deserializing the json into struct
            let key = &i.to_string(); //-- setting the key for this event
            let devlivery_status = producer.send_result( //-- we're using FutureRecord for sending the message or the event asynchoronously to all consumers cause send_result() method takes a FutureRecord to send a message
            FutureRecord::to(&topic)
                        .key(key)
                        .payload(&player_event_json) //-- we can send serde json inside the payload
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
