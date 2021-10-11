



mod schemas;
mod handlers;
mod utils;
use std::env;
use dotenv::dotenv;
use crate::utils::gen_coin;
use liby::lib_func_sample;
use futures::{executor, join};






#[tokio::main] //-- await is only allowd inside an async function due to this reason we're using the tokio as a runtime to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    
    
    

    // NOTE - rust is not based on event loop or green threads at all like nodejs - https://users.rust-lang.org/t/trying-to-understand-async-in-rust/39932/3
    let heavy_number = 2;
    let async_block = async move { //-- this is a future object which is bounded to Future trait 
        heavy_number + 3 * 274
    };
    let blocked_thread = executor::block_on(async_block); //-- block_on will run a future to completion on the current thread and block the thread (execution gets stuck there) until the given future has completed (release the mutex) 
    // let suspend_execution = async_block.await; //-- .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
    // let joined_futures = join!(async_block); //-- we can only use join!() inside an async function or block - join!() complte multiple futures at the same time
    // let joined_tokio = tokio::join!(async_block); //-- join!() is like .await but can wait for multiple futures concurrently, completing multiple futures at the same time





    
    env::set_var("RUST_LOG", "librdkafka=trace,rdkafka::client=debug");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("KAFKA_HOST").expect("⚠️ please set host in .env");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let node1_port = env::var("KAFKA_NODE1_PORT").expect("⚠️ please set kafka node1 port in .env"); //-- broker 1
    let node2_port = env::var("KAFKA_NODE2_PORT").expect("⚠️ please set kafka node2 port in .env"); //-- broker 2
    let node3_port = env::var("KAFKA_NODE3_PORT").expect("⚠️ please set kafka node3 port in .env"); //-- broker 3







    let broker1 = format!("{}:{}", host, node1_port);
    let broker2 = format!("{}:{}", host, node2_port);
    let broker3 = format!("{}:{}", host, node3_port);
    let brokers = format!("{},{},{}", broker1, broker2, broker3);
    handlers::socket::streamer::produce(&brokers).await; //-- passing brokers String by taking a reference to it, by doing this we're coercing it into &str - &String is &str

    


    Ok(())



}
