




// NOTE - declarative macros are written using macro_rules!
// NOTE - procedural macros are custom derive: #[derive(CustomDerive)], attribute-like: #[CustomAttribute], and function-like: custom!(...)
// NOTE - procedural macros enables other prgrammers to use our trait on our own struct
// NOTE - Fn trait is an object safe trait, because of unknown size at compile time it needs to be inside the Box<dyn Trait_Name>
// NOTE - macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. 
// NOTE - function gets called at runtime and a trait needs to be implemented at compile time.
// NOTE - for those types specially concrete types like traits which don't have size at compile time means they are not bounded to Sized trait, we have to point them using a pointer like Box<dyn Trait> or &dyn Trait
// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html





/* 

                        -----------------------------------------------------------------------------------
                                MINTING TOKEN BASED coiniXerr SMART CONTRACT FOR DIGITAL ASSETS
                        -----------------------------------------------------------------------------------



        ==========--------------==========--------------==========--------------==========--------------==========--------------
        | CRC20 and CRC21 (coiniXerr request for comment) traits are used to issue CRC20 and CRC21 FT and NFT based
        | smart contracts respectively to invest on tokenized asstes like musics, notes and even coiniXerr gold coins 
        | which can be stored, sent, sold and bought using coiniXerr wallet address to put them on CVM(coiniXerr Virtual Machine)
        | blockchain like regular transactions on top of the uniXerr network.
        |
        | https://medium.com/block-journal/introducing-substrate-smart-contracts-with-ink-d486289e2b59
        ==========--------------==========--------------==========--------------==========--------------==========--------------

*/




pub trait CRC21{ //-- smart contract for none fungible tokens or digital assets in which a token links ownership to unique physical or digital items

    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn trade(&mut self); //-- do something after successfull token transfer

}


pub trait CRC20{ //-- smart contract for fungible tokens or digital assets in which a token or the asset is a mapping between wallet addresses and their balances

    type TotalSupply; //-- total value of the asset
    type Decimal;

    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn trade(&mut self); //-- do something after successfull token transfer

}

