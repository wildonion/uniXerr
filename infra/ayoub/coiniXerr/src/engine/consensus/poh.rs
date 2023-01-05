





// proof of healing

// TODO - use STEM project functions and structure which has been compiled to wasm to do the AI logics
// TODO - anomal and malicious validators and blocks detection using VAE
// TODO - runtime healing on malicious nodes' attacks using VAE and transformers 
// TODO - proof of consensus using self supervised learning techniques like VAE for block transactions 

use crate::*;


pub async fn consensus_on(block: &mut Block) -> bool{

    // TODO - current_block.generate_merkle_root_hash() 
    // TODO - current_block.generate_hash()
    // ...
    
    block.is_valid = true; //// set this to true if nodes reached a consensus on this block  

    todo!()
    
}