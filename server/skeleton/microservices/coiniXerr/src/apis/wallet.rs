






use crate::constants;
use crate::utils::response::ResponseBody;
use crate::schemas::{Block, Chain, Transaction};
use actix::{*, prelude::*}; //-- loading actix actors and handlers for threads communication using their address and defined events 
use actix_web::{web, get, post, Error, HttpRequest, HttpResponse};
use futures::StreamExt;








                                                            ////////////////////////////////////
                                                            ///////// TRANSACTION APIs /////////
                                                            //////////////////////////////////// 

/* -------------------------------------------------------------------------------------------------------------------------------------------
    NOTE - a transaction should be sent from auth transfer coin api to the coiniXerr network transaction api which is a stream of binary data 
           in from of utf8 bytes loaded into the memory, then it'll deserialize or map the binary from memory into 
           the Transaction struct for mining and consensus process, finally if a transaction was added to the blockchain, 
           its is_mined field will become true and then update coins algorithm in auth microservice transfer coin will be processed.
   ------------------------------------------------------------------------------------------------------------------------------------------- */    
#[post("/coiniXerr/transaction")] //-- the route for handling streaming of regular transactions in form of utf8 binary data 
async fn transaction(req: HttpRequest, mut body_payload: web::Payload, blockchain: web::Data<Chain>) -> Result<HttpResponse, Error>{
    let blockchain = blockchain.as_ref().clone();
    let ip = req.peer_addr().unwrap().ip();
    let port = req.peer_addr().unwrap().port();
    println!("[+] SERVER TIME : {} | TRANSACTION FROM PEER ::: {}:{} ", chrono::Local::now().naive_local(), ip, port);
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = body_payload.next().await { //-- extracting binary wallet data or utf8 bytes from incoming request - loading the payload into the memory
        bytes.extend_from_slice(&chunk?); //-- the web::Payload extractor already contains the decoded byte stream if the request payload is compressed with one of the supported compression codecs (br, gzip, deflate), then the byte stream is decompressed
    }
    println!("Transaction Body in Bytes {:?}!", bytes);
    let des_trans_union = Transaction::new(&bytes).unwrap(); //-- decoding process of incoming transaction - deserializing a new transaction bytes into the Transaction struct object using TransactionMem union
    let des_trans_serde = &mut serde_json::from_slice::<Transaction>(&bytes).unwrap(); //-- deserializing bytes into the Transaction struct object using serde from_slice method
    // ----------------------------------------------------------------------
    //                              MINING PROCESS
    // ----------------------------------------------------------------------
    // TODO - limit transaction inside a block by calculating the size of the block after adding an incoming transaction from the auth microservice
    // TODO - if the size of the current block was equal to 4 mb then we have to build another block for mining its transaction
    // TODO - do the mining and consensus process here then send back the mined transaction inside the response to where it's called
    // TODO - add mined block to the coiniXerr chain
    // blockchain.add(mined_block);
    // ...
    des_trans_union.signed = Some(chrono::Local::now().naive_local().timestamp()); // TODO - this should be update after a successful signed contract and mined process
    // ----------------------------------------------------------------------
    //               SENDING SIGNED TRANSACTION BACK TO THE USER
    // ----------------------------------------------------------------------
    Ok(
        HttpResponse::Ok().json(
            ResponseBody::new(
                constants::MESSAGE_FETCHED_SUCCESS,
                des_trans_union, //-- send the signed transaction back to the user
            )
        )
    )
}


pub fn routes(config: &mut web::ServiceConfig){
    config.service(transaction);
}