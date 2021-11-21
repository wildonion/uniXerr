






use crate::schemas::Block;



pub fn can_be_valid(mut block: Block) -> Result<Block, bool>{ //-- define block as mutable cause we want to update its is_valid field - it's not a mutable borrower (pointer) to the block
    println!("-> {} - validating process has been started for block [{}]", chrono::Local::now().naive_local(), block.id);
    if let (Some(merkle_root), Some(block_hash)) = (block.clone().merkle_root, block.clone().hash){ //-- checking the block's hash and merkle_root hash for transactions finality
        block.is_valid = true;
        Ok(block)
    } else{
        Err(false)
    }



}
