





// ==========--------------==========--------------==========--------------==========--------------==========--------------
//                  A CONSENSUS ALGORITHM BASED ON SSL MODELS LIKE VAE AND A PROOF OF STAKE PATTERN 
// ==========--------------==========--------------==========--------------==========--------------==========--------------

use crate::schemas::Block;


pub fn can_be_valid(mut block: Block) -> Result<Block, bool>{ //-- define block as mutable cause we want to update its is_valid field - it's not a mutable borrower (pointer) to the block
    println!("-> {} - mining process has been started for block [{}]", chrono::Local::now().naive_local(), block.id);

    
    /*
        NOTE - all coiniXerr coin holders are uniXerr community members
        NOTE - staking is the process of actively participating in transaction validation
        NOTE - there is no reward or losing stakes for creating or attesting to a malicious block and downtime issue cause we have a weekly AI based coin generation engine for all peers based on their positions  
        NOTE - a transaction has finality when it's part of a block that can't change
        NOTE - if there is a network with 100 stakers, each who have staked $1 million dollars, 
               then the total amount of stake present is $100 million dollars. 
               if there are two blocks proposed at the same height, say B and B',
               and 66% of stakers vote on B ($66 million) and 66% vote on B' ($66 million), 
               then at least 33% of stakers were malicious meaning the loss of at least $33 million.
        TODO - confirm all valid transactions (mempool) inside a block using a proof of stake consensus algorithm based on uniXerr peers' (stakers holders' or validators') features and coins and VAE algorithm
        TODO - a NFA and DFA based algorithm using peers' positions for choosing validators to create blocks and lock thir coins for staking
        TODO - an encoder in the client side is responsible to encode each transaction
        TODO - all encoded transactions will be stored inside a block inside the server cause the're valid transactions (mempool)
        TODO - a decoder in the server side is responsible to decode all of them from the latent space of VAE
        TODO - if the decoder was successful at decoding each transactin and no anomal transaction detected means all transactins are valid which cause the block be a created one
    */
    
    
    let is_valid = true; // TODO - checks the block's hash to ensure it's legitimate
    if is_valid{
        block.is_valid = true; // TODO - 
        Ok(block)
    } else{
        Err(false)
    }


}
