








use crate::schemas::Block;


pub fn can_be_mined(mut block: Block) -> Result<Block, bool>{ //-- define block as mutable cause we want to update its is_mined field
    println!("-> mining process has been started for block [{}]", block.id);
    // TODO - confirm all valid transactions (mempool) inside a block using a proof of stake consensus algorithm based on uniXerr peers' features
    // TODO - an encoder in the client side is responsible to encode each transaction
    // TODO - all encoded transactions will be stored inside a block inside the server cause the're valid transactions (mempool)
    // TODO - a decoder in the server side is responsible to decode all of them from the latent space
    // TODO - if the decoder was successful at decoding each transactin and no anomal transaction detected means all transactins are valid which cause the block be a mined one 
    // ...
    let is_mined = true;
    if is_mined{
        block.is_mined = true; // TODO - 
        Ok(block)
    } else{
        Err(false)
    }



}