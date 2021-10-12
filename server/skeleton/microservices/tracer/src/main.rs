


mod constants;
mod schemas;
mod api;
mod utils;
use std::env;
use dotenv::dotenv;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};
use listenfd::ListenFd;
use api::device::init_routes as device_init_routes;



#[actix_web::main] //-- await is only allowd inside an async function due to this reason we're using the tokio to make the main() function as a runtime executor
async fn main() -> std::io::Result<()>{
    



    
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ please set host in .env");
    let port = env::var("TRACER_PORT").expect("⚠️ please set port in .env");

    
    
    

    let mut listenfd = ListenFd::from_env();
    let mut server = 
        HttpServer::new(move || {
            App::new()
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
                .wrap(middleware::Logger::default())
                .configure(device_init_routes)
        });
    
    






    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await



}
