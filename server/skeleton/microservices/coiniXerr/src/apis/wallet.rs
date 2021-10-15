





use crate::constants;
use crate::utils::response::ResponseBody;
use actix_web::{web, get, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use liby;




#[get("/uniXerr/api/coiniXerr")] //-- the route for handling streaming of utf8 binary data
async fn index(req: HttpRequest, mut body: web::Payload) -> Result<HttpResponse, Error>{
    let ip = req.peer_addr().unwrap().ip();
    let port = req.peer_addr().unwrap().port();
    println!("[+] SERVER TIME : {} | REQUEST FROM PEER ::: {}:{} ", chrono::Local::now().naive_local(), ip, port);
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await { //-- extracting binary wallet data or utf8 bytes from incoming request
        bytes.extend_from_slice(&item?);
    }
    println!("Wallet Body {:?}!", bytes);
    Ok(
        HttpResponse::Ok().json(
            ResponseBody::new(
                constants::MESSAGE_FETCHED_SUCCESS,
                constants::EMPTY,
            )
        )
    )
}


pub fn routes(config: &mut web::ServiceConfig){
    config.service(index);
}