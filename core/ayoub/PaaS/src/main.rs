




/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  




*/





// #![allow(unused)] //-- will let the unused vars be there - we have to put this on top of everything to affect the whole crate
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate 



use constants::MainResult;
use routerify::{RouterService, Router};
use std::{net::SocketAddr, sync::Arc, env};
use dotenv::dotenv;
use uuid::Uuid;
use log::{info, error};
use tokio::sync::oneshot;
use tokio::sync::Mutex; //-- async Mutex will be used inside async methods since the trait Send is not implement for std::sync::Mutex
use hyper::server::Server;
use self::contexts as ctx; // use crate::contexts as ctx;




mod middlewares;
mod utils;
mod constants;
mod contexts;
mod schemas;
mod controllers;
mod routers;
















#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> MainResult<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //-- generic types can also be bounded to lifetimes ('static in this case) and traits inside the Box<dyn ... > - since the error that may be thrown has a dynamic size at runtime we've put all these traits inside the Box (a heap allocation pointer) and bound the error to Sync, Send and the static lifetime to be valid across the main function and sendable and implementable between threads
    
    



    



    




    // -------------------------------- environment variables setup
    //
    // ---------------------------------------------------------------------
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let port = env::var("AYOUB_PORT").expect("⚠️ no port variable set");
    let sms_api_token = env::var("SMS_API_TOKEN").expect("⚠️ no sms api token variable set");
    let sms_template = env::var("SMS_TEMPLATE").expect("⚠️ no sms template variable set");
    let (sender, receiver) = oneshot::channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the oneshot channel
    let server_addr = format!("{}:{}", host, port).as_str().parse::<SocketAddr>().unwrap();
    
    







    

    
    

    

    // -------------------------------- app storage setup
    //
    // ---------------------------------------------------------------------
    let empty_app_storage = Some( //-- putting the Arc-ed db inside the Option
        Arc::new( //-- cloning app_storage to move it between threads
            ctx::app::Storage{ //-- defining db context 
                id: Uuid::new_v4(),
                db: Some(
                    ctx::app::Db{
                        mode: ctx::app::Mode::Off,
                        instance: None,
                        engine: None,
                        url: None,
                    }
                ),
            }
        )
    );
    let app_storage = if db_engine.as_str() == "mongodb"{
        info!("switching to mongodb");
        
        let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
        let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
        let db_username = env::var("MONGODB_USERNAME").expect("⚠️ no db username variable set");
        let db_password = env::var("MONGODB_PASSWORD").expect("⚠️ no db password variable set");
        
        let db_addr = if environment == "dev"{
            format!("{}://{}:{}", db_engine, db_host, db_port)
        } else if environment == "prod"{
            format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port)
        } else{
            "".to_string()
        };
        
        match ctx::app::Db::new().await{
            Ok(mut init_db) => {
                init_db.engine = Some(db_engine);
                init_db.url = Some(db_addr);
                let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
                Some( //-- putting the Arc-ed db inside the Option
                    Arc::new( //-- cloning app_storage to move it between threads
                        ctx::app::Storage{ //-- defining db context 
                            id: Uuid::new_v4(),
                            db: Some(
                                ctx::app::Db{
                                    mode: init_db.mode,
                                    instance: Some(mongodb_instance),
                                    engine: init_db.engine,
                                    url: init_db.url,
                                }
                            ),
                        }
                    )
                )
            },
            Err(e) => {
                error!("init db error - {}", e);
                empty_app_storage //-- whatever the error is we have to return and empty app storage instance 
            }
        }
    } else if db_engine.as_str() == "postgres"{
        
        let db_host = env::var("POSTGRES_HOST").expect("⚠️ no db host variable set");
        let db_port = env::var("POSTGRES_PORT").expect("⚠️ no db port variable set");
        let db_username = env::var("POSTGRES_USERNAME").expect("⚠️ no db username variable set");
        let db_password = env::var("POSTGRES_PASSWORD").expect("⚠️ no db password variable set");
        
        let db_addr = if environment == "dev"{
            format!("{}://{}:{}", db_engine, db_host, db_port)
        } else if environment == "prod"{
            format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port)
        } else{
            "".to_string()
        };

        // TODO 
        todo!();
    
    } else{
        empty_app_storage
    };







    









    // -------------------------------- update to dev access level
    //
    // ------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();
    let mut username_cli = &String::new(); //-- this is a mutable reference to the username_cli String location inside the heap since we want to mutate the content inside the heap using the pointer later
    let mut access_level_cli = &String::new(); //-- this is a mutable reference to the access_level_cli String location inside the heap since we want to mutate the content inside the heap using the pointer later
    if args.len() > 1{
        username_cli = &args[1];
        access_level_cli = &args[2];
    }
    if username_cli != &"".to_string() && access_level_cli != &"".to_string(){
        match utils::set_user_access(username_cli.to_owned(), access_level_cli.parse::<i64>().unwrap(), app_storage.clone()).await{
            Ok(user_info) => {
                info!("access level for user {} has been updated successfully", username_cli);
                info!("updated user {:?}", user_info);
            },
            Err(empty_doc) => {
                info!("no user found for updating access level");
            },
        }
    } else{
        info!("no username has passed in to the cli; pass updating access level process");
    }












    


    // -------------------------------- initializing the otp info instance
    //
    // ---------------------------------------------------------------------------------------
    let mut otp_auth = utils::otp::Auth::new(sms_api_token, sms_template); //// the return type is impl Otp trait which we can only access the trait methods on the instance - it must be defined as mutable since later we want to get the sms response stream to decode the content, cause reading it is a mutable process
    let otp_info = ctx::app::OtpInfo{
        otp_auth: Box::new(otp_auth), 
    };
    let arced_mutexd_otp_info = Arc::new( //// in order the OtpInput to be shareable between routers' threads it must be sendable or cloneable and since the Clone trait is not implemented for the OtpInput we're putting it inside the Arc
                                                        Mutex::new( //// in order the OtpInput to be mutable between routers' threads it must be syncable thus we have to put it inside the Mutex which based on mpsc rule means that only one thread can mutate it at a time 
                                                            otp_info
                                                        )
                                                    );
    











                                                    



    
    // -------------------------------- building the ayoub server from the router
    //
    //      we're sharing the db_instance state between routers' threads to get the data inside each api
    // --------------------------------------------------------------------------------------------------------
    let unwrapped_storage = app_storage.unwrap(); //-- unwrapping the app storage to create a db instance
    let db_instance = unwrapped_storage.get_db().await; //-- getting the db inside the app storage; it might be None
    let api = Router::builder()
        .data(db_instance.unwrap().clone()) //-- shared state which will be available to every route handlers is the db_instance which must be Send + Syn + 'static to share between threads
        .scope("/auth", routers::auth::register().await)
        .scope("/event", routers::event::register().await)
        .scope("/game", routers::game::register().await)
        .build()
        .unwrap();

    info!("running ayoub server on port {} - {}", port, chrono::Local::now().naive_local());
    let ayoub_service = RouterService::new(api).unwrap();
    let ayoub_server = Server::bind(&server_addr).serve(ayoub_service);
    let ayoub_graceful = ayoub_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
    if let Err(e) = ayoub_graceful.await{ //-- awaiting on the server to receive the shutdown signal
        unwrapped_storage.db.clone().unwrap().mode = ctx::app::Mode::Off; //-- set the db mode of the app storage to off
        error!("ayoub server error {} - {}", e, chrono::Local::now().naive_local());
    }







        
        
    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");
        
        
        
        
        
        
    Ok(())
    





}
