


mod constants;
mod schemas;
mod handlers;
mod utils;
mod actors;
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer, Error, HttpResponse, middleware};
use actix_session::CookieSession;
use futures::{executor, join, Stream, StreamExt};






async fn index(mut body: web::Payload) -> Result<HttpResponse, Error>{
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await { //-- extracting binary data or utf8 bytes from incoming request
        bytes.extend_from_slice(&item?);
    }

    format!("Body {:?}!", bytes);
    Ok(HttpResponse::Ok().finish())
}






#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the actix_web as a runtime on top of tokio to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    





    let heavy_number = 2;
    let async_block = async move { //-- this is a future object which is bounded to Future trait and it's not like the nodejs event loop
        heavy_number + 3 * 274
    };
    let blocked_thread = executor::block_on(async_block); //-- block_on will run a future to completion on the current thread and block the thread (execution gets stuck there) until the given future has completed (release the mutex) 
    // let suspend_execution = async_block.await; //-- .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
    // let joined_futures = join!(async_block); //-- we can only use join!() inside an async function or block - join!() complte multiple futures at the same time
    // let joined_tokio = tokio::join!(async_block); //-- join!() is like .await but can wait for multiple futures concurrently, completing multiple futures at the same time    
    



    


    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env::set_var("RUST_LOG", "librdkafka=trace,rdkafka::client=debug");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set coiniXerr http port in .env");
    let kafka_host = env::var("KAFKA_HOST").expect("⚠️ please set kafka host in .env");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let node1_port = env::var("KAFKA_NODE1_PORT").expect("⚠️ please set kafka node1 port in .env"); //-- broker 1
    let node2_port = env::var("KAFKA_NODE2_PORT").expect("⚠️ please set kafka node2 port in .env"); //-- broker 2
    let node3_port = env::var("KAFKA_NODE3_PORT").expect("⚠️ please set kafka node3 port in .env"); //-- broker 3





    
    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                .service(web::resource("/stream").route(web::get().to(index))) //-- the route for handling streaming of utf8 binary data
                .wrap(middleware::Logger::default())
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
        });
    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", host, coiniXerr_http_port))?
        }
    };
    server.run().await;







    let broker1 = format!("{}:{}", kafka_host, node1_port);
    let broker2 = format!("{}:{}", kafka_host, node2_port);
    let broker3 = format!("{}:{}", kafka_host, node3_port);
    let brokers = format!("{},{},{}", broker1, broker2, broker3);
    handlers::socket::kafka::produce(&brokers).await; //-- passing brokers String by taking a reference to it, by doing this we're coercing it into &str - &String is &str

    


    Ok(())



}
