











#[tokio::main]
async fn main() -> std::io::Result<()>{ //-- await is only allowd inside an async function due to this reason we're using the actix_web as an event loop based runtime under the hood on top of tokio to make the main() function as an async one
    



    /////// ==========--------------==========--------------==========--------------==========--------------==========-------------- 
    ///////                                          parachains and parathreads coiniXerr node
    /////// ==========--------------==========--------------==========--------------==========--------------==========--------------
    // NOTE - web3 or polkadotjs <-grpc or wss api-> blockchain nodes
    // TODO - connect other coiniXerr full node blockchains from other instances of this server to build a parachains and parathreads based network for parallel transactions using scheduler, libp2p, protobuf and gRPC protocol over http or tcp socket
    // TODO - solve forking and reorgs issue for this model of blockchain by choosing the longest chain created by new() method of the blockchain object of the Chain struct 
    // TODO - even though it's possible for two rivaling chains to exist at the same time, soon one of the two chains will add another block and outgrow the other due to the time it takes to solve the mining algorithms,
    // TODO - save the whole chain state inside a db or a persistence storage using wasm 
    // ...









}


