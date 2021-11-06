



/* 

        ==========--------------==========--------------==========--------------==========--------------==========--------------
        | CRC20 (coiniXerr request for comment) nft tokens are used to issue crc20 based smart contracts 
        | to invest on tokenized blockchain based asstes like musics, notes and even coiniXerr gold coins 
        | which can be stored and sent using coiniXerr address and transactions on the coiniXerr blockchain VM 
        |
        | https://medium.com/block-journal/introducing-substrate-smart-contracts-with-ink-d486289e2b59
        | https://www.wealdtech.com/articles/understanding-erc20-token-contracts/
        ==========--------------==========--------------==========--------------==========--------------==========--------------

*/




pub trait Contract{

    type Token;
    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn trade(&mut self); //-- do something after successfull token transfer

}