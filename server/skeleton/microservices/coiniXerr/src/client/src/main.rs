







use std;



#[tokio::main]
async fn main() -> std::io::Result<()>{ //-- await is only allowd inside an async function due to this reason we're using the actix_web as an event loop based runtime under the hood on top of tokio to make the main() function as an async one
    



    /////// ==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////                                          parachains and parathreads coiniXerr node
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    // NOTE - nodes' weights are their stakes which is their total coiniXerr balance
    // TODO - solve forking and reorgs issue for this model of blockchain by choosing the longest chain created by new() method of the blockchain object of the Chain struct 
    // TODO - save the whole chain state inside a db or a persistence storage in every peer of coiniXerr blockchain node
    // ...
    
    
    // TEST - web3 or polkadotjs <-p2p based gRPC or IPC or ws network-> store some transaction based assets in all blockchain nodes
    // TEST - run .wasm to call coiniXerr blockchain network methods and functions directly from js 


    Ok(())

}
