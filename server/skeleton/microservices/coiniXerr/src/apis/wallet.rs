






use crate::constants;
use crate::utils::response::ResponseBody;
use crate::schemas::{Transaction, Block};
use actix_web::{web, get, post, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use liby;








/* 
    NOTE - a transaction should be sent from auth transfer coin api to the coiniXerr network transaction api which is a stream of binary data 
           in from of utf8 bytes loaded into the memory, then it'll deserialize or map the binary from memory into the Transaction struct for mining and consensus process,
           finally if a transaction was added to the blockchain, its is_mined field will become true and then update coins algorithm in auth microservice transfer coin will be processed.
*/
#[post("/uniXerr/api/coiniXerr/transaction")] //-- the route for handling streaming of transactions in form of utf8 binary data 
async fn transaction(req: HttpRequest, mut body_payload: web::Payload) -> Result<HttpResponse, Error>{
    let ip = req.peer_addr().unwrap().ip();
    let port = req.peer_addr().unwrap().port();
    println!("[+] SERVER TIME : {} | TRANSACTION FROM PEER ::: {}:{} ", chrono::Local::now().naive_local(), ip, port);
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = body_payload.next().await { //-- extracting binary wallet data or utf8 bytes from incoming request - loading the payload into the memory
        bytes.extend_from_slice(&chunk?); //-- actix automatically decodes chunked encoding, the web::Payload extractor already contains the decoded byte stream if the request payload is compressed with one of the supported compression codecs (br, gzip, deflate), then the byte stream is decompressed
    }
    println!("Transaction Body in Bytes {:?}!", bytes);
    let des_trans_union = Transaction::new(&bytes).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes into the Transaction struct object using our TransactionMem union
    let mut des_trans_serde = serde_json::from_slice::<Transaction>(&bytes).unwrap(); //-- deserializing bytes into the Transaction struct object using serde from_slice method
    // TODO - limit transaction inside a block by calculating the size of the block in every incoming transaction from the auth microservice
    // TODO - if the size of the current block was equal to 4 mb then we have to build another block for mining its transaction
    // TODO - do the mining and consensus process here then send back the mined transaction inside the response to where it's called
    // ...
    des_trans_union.signed = Some(chrono::Local::now().naive_local().timestamp()); // TODO - this should be update after a successful signed contract and mined process
    Ok(
        HttpResponse::Ok().json(
            ResponseBody::new(
                constants::MESSAGE_FETCHED_SUCCESS,
                des_trans_union,
            )
        )
    )
}


pub fn routes(config: &mut web::ServiceConfig){
    config.service(transaction);
}