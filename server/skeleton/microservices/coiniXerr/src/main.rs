



mod schemas;
mod handlers;
mod utils;
use std::env;
use dotenv::dotenv;
use crate::utils::gen_coin;
use liby::lib_func_sample;







#[tokio::main] //-- await is only allowd inside an async function due to this reason we're using the tokio as a runtime to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    
    
    


    // NOTE - when we put the async keyword behind a block or function means we're implementing Future<Output = ()> trait for them and turn them into a Future object and the outpute depends on the return type of that function or block
    // NOTE - .await suspend execution until the result of a Future is ready and async return a Future instead of blocking the current thread to run it
    // NOTE - async_block is a future object and will not be run immediately but can be solved by blocking the current thread using .await
    // NOTE - .awaiting a future will suspend the current function's execution until the executor has run the future to completion and can only be used just inside async function
    let heavy_number = 2;
    let async_block = async move {
        heavy_number + 3 * 274
    };




    
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
    handlers::socket::whisper::produce(&brokers).await; //-- passing brokers String by taking a reference to it, by doing this we're coercing it into &str - &String is &str

    


    Ok(())



}
