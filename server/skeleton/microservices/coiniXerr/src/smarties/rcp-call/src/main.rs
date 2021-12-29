









// TODO - add features to a module to include it in Cargo when installing that module like { version = "0.14", features = ["full"] }
// ...
#![feature(never_type)]
impl Debug for ! {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        *self
    }
}





use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn}; //-- required for making a service from a function for the hyper server







//-- The error type for errors that can never happen, since this enum has no variant, a value of this type can never actually exist. 
//-- this can be useful for generic APIs that use [Result] and parameterize the error type, to indicate that the result is always [Ok] 
pub type Empty = ();
pub type Infallible = !; //-- define the associated type of never type using Infallible - Infallible and ! are the same type and expressions with type ! will coerce into any other type
pub type Error = Infallible; //-- an error which never happen






async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible>{ //-- we have no error at all so we use Infallible
    Ok(Response::new("Hello, world".into()));
}





#[tokio::main]
async fn main(){

    
    
    
    // https://docs.rs/hyper/0.14.16/hyper/server/index.html
    // http://zderadicka.eu/hyper-websocket/
    // an RPC call to our smart contract by seding a new transaction to our deployed contract_program to change its state using defined instructions and the instruction_data field
    // TODO - all incoming binary streaming RPC commands will be decoded in here using the hyper http server
    // TODO - design pattern syntax for api calls ====> api!(endpoint, data, access(2), method)
    // ...
    


    let x: ! = {
        return 123; //-- here ! will be coerced to i32 
    };




    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async{
        println!("inside the service function..");
        Ok::<_, Infallible>(service_fn(handle)) //-- it'll never return error on Ok cause we've used Infallible type
    });



    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await{
        eprintln!("server error: {}", e);
    }

    





}
