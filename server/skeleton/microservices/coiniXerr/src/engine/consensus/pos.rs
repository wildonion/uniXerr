








use crate::schemas::Block;


pub fn can_be_mined(mut block: Block) -> Result<Block, bool>{ //-- define block as mutable cause we want to update its is_mined field
    println!("-> mining process has been started for block [{}]", block.id);
    // TODO - if there is a network with 100 stakers, each who have staked $1 million dollars, 
    //        then the total amount of stake present is $100 million dollars. 
    //        if there are two blocks proposed at the same height, say B and B',
    //        and 66% of stakers vote on B ($66 million) and 66% vote on B' ($66 million), 
    //        then at least 33% of stakers were malicious meaning the loss of at least $33 million.
    // TODO - confirm all valid transactions (mempool) inside a block using a proof of stake or NFA and DFA based consensus algorithm based on uniXerr peers' (stakers) features and coins and VAE algorithm
    // TODO - an encoder in the client side is responsible to encode each transaction
    // TODO - all encoded transactions will be stored inside a block inside the server cause the're valid transactions (mempool)
    // TODO - a decoder in the server side is responsible to decode all of them from the latent space of VAE
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
