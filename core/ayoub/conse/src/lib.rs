




//// conse test apis

use std::env;
use std::net::SocketAddr;
use hyper::{Uri, Server, Request, Response, Body, Error, Client, server::conn::AddrIncoming};
use routerify::{RouterService, Router};
use crate::routers::*;
mod routers;







pub async fn build_server(router: Router<Body, Error>) -> Server<AddrIncoming, RouterService<Body, Error>>{

    let host = env::var("HOST").expect("⚠️ no host variable set");
    let port = env::var("CONSE_PORT").expect("⚠️ no port variable set");
    let server_addr = format!("{}:{}", host, port).as_str().parse::<SocketAddr>().unwrap();
    let conse_service = RouterService::new(router).unwrap();
    let conse_server = Server::bind(&server_addr).serve(conse_service);
    conse_server

}





#[cfg(test)]
mod tests{


    use super::*;

    #[tokio::test]
    async fn get_all_in_going_events() -> Result<(), hyper::Error>{
        
        //// building the server for testing
        let host = env::var("HOST").expect("⚠️ no host variable set");
        let port = env::var("CONSE_PORT").expect("⚠️ no port variable set");
        let api = Router::builder()
                .scope("/auth", routers::auth::register().await)
                .scope("/event", routers::event::register().await)
                .scope("/game", routers::game::register().await)
                .build()
                .unwrap();
        let conse_server = self::build_server(api).await;

        //// sending get request to the stated conse server
        let uri = format!("http://{}:{}/event/get/all/in-going", host, port).as_str().parse::<Uri>().unwrap(); //-- parsing it to hyper based uri
        let client = Client::new();
        let Ok(res) = client.get(uri).await else{
            panic!("test failed");
        };

        

        Ok(())

    }

}