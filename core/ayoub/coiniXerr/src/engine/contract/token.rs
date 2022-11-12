





// NOTE - all coiniXerr coin holders are uniXerr community members called participants known as validator actors
// NOTE - validator actors can issue contracts by staking some coins from their coiniXerr balance which can be deposited based on their smart contract tokens (CRC20) ExpTime field
// NOTE - validator actors get rewarded based on total values of their contracts and an AI based algorithm which is position clustering coin generation model
// NOTE - validator actors or voters will vote to a bidded or auctioned contract by staking their coins on the coiniXerr network and get rewarded based on the coiniXerr liquidity or token pool or every tax of the voted transaction of the contract
// TODO - compile coiniXerr contracts to wasm to run in browser through the loading process of the .wasm or .js compiled file or .so bpf bytecode


use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case



pub trait CRC20{ //-- dApp based smart contract for fungible tokens with a limited supply to invest on tokenized asstes like musics, notes, homes and flash loan for coiniXerr coins in which a token or the asset is a mapping between wallet addresses and their balances - a fungible token which is not unique is any token whose fundamental unit or characteristic is interchangeable with other tokens of the same set

    // ------------------
    //-- associated types
    // ------------------
    type TokenID;
    type TokenName;
    type TotalSupply; //-- total value or price or balance of this cryptocurrency token of the asset - sum of all addresses' balance must be equal to this; eg : an asset with total supply of 40 can be divided into 4 addresses 10 value for each means thos addresses must invest 10 coins to own this asset 
    type Decimal;
    type TokenAddress; //-- the address of this token which can be stored, sent, sold and bought using coiniXerr wallet address
    type ExpTime; //-- expiration time

    // -------------------
    //-- ownership methods
    // -------------------
    fn mint(&mut self); //-- minting FT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- the code that's executed by the contract's method can include calls to other contracts, these trigger more transactions that have the from field set to the contract's address - a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn owner_of(&mut self); //-- this function returns the address of the owner of a token. As each ERC-721 token is unique and non-fungible, they are represented on the blockchain by an ID,  other users, contracts, apps can use this ID to determine the owner of the token
    
    // ---------------
    //-- event methods
    // ---------------
    fn burn(&mut self); //-- burn some of the tokens
    
}

pub trait CRC21{ //-- dApp smart contract for none fungible tokens or digital assets in which a token links ownership to unique physical or digital items like musics, notes, homes and even coiniXerr gold coins 

    // ------------------
    //-- associated types
    // ------------------
    type AssetLink; //-- stored in IPFS or uniXerr network
    type TokenName;
    type TokenID;
    type TokenMetaData; //-- metadata schema in json format about the place of stored NFT
    type TokenAddress; //-- the address of this NFT which can be stored, sent, sold and bought using coiniXerr wallet address
    type ExpTime; //-- expiration time
    type Volume; //-- number of total NFTs of an asset; like there are total of 10 rust notes but with unique id 

    // -------------------
    //-- ownership methods
    // -------------------
    fn mint(&mut self); //-- minting NFT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- the code that's executed by the contract's method can include calls to other contracts, these trigger more transactions that have the from field set to the contract's address - a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn owner_of(&mut self); //-- this function returns the address of the owner of a token. As each ERC-721 token is unique and non-fungible, they are represented on the blockchain by an ID,  other users, contracts, apps can use this ID to determine the owner of the token

    // ---------------
    //-- event methods
    // ---------------
    fn burn(&mut self); //-- burn some of the tokens to zero address
}

pub trait CRC22{ //-- coiniXerr smart contract supports variety of tokens and standards like both FT and NFT types

    // ------------------
    //-- associated types
    // ------------------
    type TokenID;
    type TokenName;
    type TotalSupply;
    type TokenAddress; //-- the address of this token
    type AssetLink; //-- stored in IPFS or coiniXerr blockchain
    type TokenMetaData; //-- metadata schema in json format about the place of stored NFT
    type ExpTime; //-- expiration time
    type Volume; //-- number of total NFTs of an asset; like there are total of 10 rust notes but with unique id

    // -------------------
    //-- ownership methods
    // -------------------
    fn mint(&mut self); //-- minting FT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
    fn balance_of(&mut self); //-- query the deployer’s balance
    fn safe_transfer_from(&mut self); //-- transfer items to player accounts
    fn safe_batch_transfer_from(&mut self); //-- batch transfer items to player accounts and get the balance of batches using balance_of_batch() method
    fn balance_of_batch(&mut self); //-- balance of batches
    fn uri(&mut self, id: String); //-- the uri metadata can include the string {id} which clients must replace with the actual token ID, in lowercase hexadecimal (with no 0x prefix) and leading zero padded to 64 hex characters or 32 bytes; eg token ID : 0000000000000000000000000000000000000000000000000000000000000002
    fn owner_of(&mut self); //-- this function returns the address of the owner of a token. As each ERC-721 token is unique and non-fungible, they are represented on the blockchain by an ID,  other users, contracts, apps can use this ID to determine the owner of the token

    // ---------------
    //-- event methods
    // ---------------
    fn burn(&mut self); //-- burn some of the tokens
    
}
