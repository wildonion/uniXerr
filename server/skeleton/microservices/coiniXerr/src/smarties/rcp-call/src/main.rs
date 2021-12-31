














use std::{convert::Infallible, net::SocketAddr}; //-- Infallible and ! are the same type and expressions with type ! will coerce into any other type
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn}; //-- required for making a service from a function for the hyper server





//-- The error type for errors that can never happen, since this enum has no variant, a value of this type can never actually exist. 
//-- this can be useful for generic APIs that use [Result] and parameterize the error type, to indicate that the result is always [Ok] 
pub type Empty = ();
pub type Error = Infallible; //-- an error which never happen




async fn cmd(req: Request<Body>) -> Result<Response<Body>, Infallible>{ //-- we have no error at all so we use Infallible
    Ok(Response::new("Hello, world".into()))
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    
    
    
    // https://docs.rs/hyper/0.14.16/hyper/server/index.html
    // http://zderadicka.eu/hyper-websocket/
    // an RPC call to our smart contract by seding a new transaction to our deployed contract_program to change its state using defined instructions and the instruction_data field
    // TODO - add features to a module to include it in Cargo when installing that module like { version = "0.14", features = ["full"] }
    // TODO - all incoming binary streaming RPC commands will be decoded in here using the hyper http server
    // TODO - design pattern syntax for api calls ====> api!(endpoint, data, access(2), method)
    // ...
    
    

    
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async{
        Ok::<_, Infallible>(service_fn(cmd))
    });
    

    
    let server = Server::bind(&addr).serve(make_svc);
    println!("[+] server is listening...");
    if let Err(e) = server.await{
        eprintln!("server error: {}", e);
    }

    

    // let access = |ac| {
    //     match ac {
    //         1 => {
    //             //-- superuser
    //             // ...
    //         }, 
    //         2 => {
    //             //-- admin
    //             // ...
    //         }, 
    //     }
    // };


    // api!("/user/all", method, access(2), token)


    Ok(())


}
