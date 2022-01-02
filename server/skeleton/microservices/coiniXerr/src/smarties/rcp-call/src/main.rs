














//-- The error type for errors that can never happen, since this enum has no variant, a value of this type can never actually exist. 
//-- this can be useful for generic APIs that use [Result] and parameterize the error type, to indicate that the result is always [Ok] 
use std::{convert::Infallible, net::SocketAddr}; //-- Infallible and ! are the same type and expressions with type ! will coerce into any other type
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn}; //-- required for making a service from a function for the hyper server
use hyper::server::conn::AddrStream;
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;








#[derive(Clone)]
pub struct AppContext{
    pub id: u32,
} 




async fn api(context: AppContext, add: SocketAdd, req: Request<Body>) -> Result<Response<Body>, Infallible>{ //-- we have no error at all so we use Infallible
    
    let mut response = Response::new(Body::empty()); //-- empty &[u8] of buffer bytes
    match (req.method(), req.uri().path()){
        (&Method::GET, "/") => {
            // TODO - access level closure
            // TODO - check access level in req.headers
            // ...
        },
        (&Method::POST, "/user") => {
            // TODO - access level closure
            // TODO - check access level in req.headers
            // ...
            let full_body_chunk = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = full_body_bytes
                .map_ok(|chunk|{ //-- chunk of bytes is a set of bytes
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase()) //-- iterating through all chunks to convert each of their bytes to ASCII uppercase
                        .collect::<Vec<u8>>()
                });
                *response.body_mut() = Body::wrap_stream(mapping);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    
    Ok(response)
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
    
    
    


    
    
    
    let context = AppContext{
        id: 0001110,
    };

    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_service = make_service_fn(move |conn: &AddrStream| async{
        let context = context.clone(); //-- cloning context to move it between hyper threads - if your data doesn't implement Clone trait we have to use Arc
        let addr = conn.remote_addr();
        let service = service_fn(move |req|{
            api(context.clone(), addr, req)
        });
        Ok::<_, Infallible>(service)
    });
    

    
    let server = Server::bind(&addr).serve(make_service);
    println!("[+] server is listening...");
    if let Err(e) = server.await{
        eprintln!("server error: {}", e);
    }


    


    







    Ok(())


}
