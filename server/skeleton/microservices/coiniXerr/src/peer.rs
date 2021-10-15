



// NOTE - peer can be an actix actor or a tcp or udp socket or even a kafka subscriber to transfer coins to other peers using their wallet address
// NOTE - every peer is also a miner cause we have an AI core which will generate credit score coins based on peer position clustering process
// ...




pub mod actor; //-- actor based peer protocol to send transaction infos
pub mod kafcriber; //-- kafka subscriber based peer protocol to send transaction infos
pub mod node; //-- p2p based protocol to send transaction infos
