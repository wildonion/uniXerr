







#[macro_use]
extern crate diesel;
mod utils;
mod entities;
mod handlers;
mod schema;
mod services;
mod constants;
mod controllers;
mod middlewares;
use serde::Serialize;
use actix_session::CookieSession;
use actix_web::{App, web, HttpServer, HttpRequest, Responder};
use actix_web::middleware::Logger;
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;
use self::controllers::adminer::ACL::set_access_level;
use self::controllers::auth::auth_init_service; // to configure auth apis
use self::entities::{
    users::api::user_init_service,
    login_history::api::login_history_init_service,
    user_friend::api::user_friend_init_service,
};






#[derive(Serialize)]
struct Greet{
    name: String,
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    web::Json(Greet{name: name.to_string()})
}










#[actix_web::main]
async fn main() -> std::io::Result<()> { //-- return type is an empty Result object - std::io::Result is broadly used across std::io for any operation which may produce an error.
    
        
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
        env_logger::init();
        dotenv().expect("⚠️ .env file not found");
        let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
        let host = env::var("HOST").expect("⚠️ please set host in .env");
        let port = env::var("AUTH_PORT").expect("⚠️ please set port in .env");






        let args: Vec<String> = env::args().collect();
        if args.len() == 3{
            let un: Option<&String>;
            un = Some(&args[1]);
            let al: Option<&i16>;
            let parsed_access_level = &args[2].parse::<i16>().unwrap();
            al = Some(parsed_access_level);
            if environment == "dev" && un != None && al != None{
                match set_access_level(un, al).await{
                    Ok(user) => {
                        println!("\n[+] Successfully update access level - {:?}\n", user);
                    },
                    Err(err) => {
                        println!("\n[!] Error updating access level - {}\n", err);
                    }
                }
            } else{
                println!("\n[?] Warning - username and access level are empty or environment is not set to dev\n");
            }
        } else{
            println!("\n[?] Warning - Empty arguments\n");
        }







        let mut listenfd = ListenFd::from_env();
        let mut server = 
            HttpServer::new(move || {
                App::new() // NOTE - we can build the pg pool in here and pass its clone through the .data() actix method
                    .route("/", web::get().to(greet))
                    .route("/{name}", web::get().to(greet))
                    .configure(login_history_init_service)
                    .configure(user_init_service)
                    .configure(auth_init_service)
                    .configure(user_friend_init_service)
                    .wrap(CookieSession::signed(&[0; 32]).secure(false))
                    .wrap(Logger::default())
            });
        
        




        
        server = match listenfd.take_tcp_listener(0)?{
            Some(listener) => server.listen(listener)?,
            None => {
                server.bind(format!("{}:{}", host, port))?
            }
        };
        server.run().await

}