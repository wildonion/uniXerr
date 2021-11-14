



mod handlers;
use std::env;
use dotenv::dotenv;
use actix::*;
use actix_web::{middleware, App, HttpServer};
use actix_session::CookieSession;
use listenfd::ListenFd;
use crate::handlers::{
    db::cass::schemas::player::Chat,
    db::cass::establish as cass,
    proxies::chat::{balancer as chat_balancer, session::user_chat_sess_init},
};







#[actix_web::main] // NOTE - don't upgrade the actix to newer version cause it'll not work with the current websocket config
async fn main() -> std::io::Result<()> {
    






    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let port = env::var("SUPROXY_PORT").expect("⚠️ please set port in .env");
    let cass_session = cass::connection().await.expect("⚠️ can't establish cassandra connection!"); //-- making cassandra pool of connections for selected node
    Chat::init(cass_session.clone()); //-- it'll create user_chat column family if there is not any
    
    
    
    
    
    
    
    // NOTE - cloning db connections using Arc cause trait Copy and Clone is not implemented for them and they are not Sync and Send and safe to move between threads thus Arc do these things for us
    // NOTE - in order to start the ChatServer actor we have to define a simple Context for that
    // NOTE - based on orphan rule in order to call the start() method on our ChatServer actor we have to include the Actor trait in here cause start() method belongs to Actor trait
    let chat_server_address = chat_balancer::ChatServer::new(cass_session.clone()).start(); // start a new asynchronous actor, returning its address.
    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                .data(chat_server_address.clone())
                .data(cass_session.clone()) // NOTE - cloning db connections using Arc cause trait Copy and Clone is not implemented for them and they are not Sync and Send and safe to move between threads thus Arc do these things for us
                .wrap(middleware::Logger::default())
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
                .configure(user_chat_sess_init) // NOTE - websocket route configuration
        });
    
    






    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await




}
