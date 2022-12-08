




// https://stackoverflow.com/questions/50297252/actors-thread-based-vs-event-driven
// TODO - networking projects that must build with actors:
//      • traffic forwarding tools like ngrok using iptables
//      • proxy and all layers load balancer like pingora based on cpu task scheduling, weighted round robin dns, vector clock, event loop and simd vectorization 
//      • vpn like v2ray protocols with zero proof of knowledge  
//      • binary address transmition protocol like onionary://010101000001:2324 acts as a message broker like rmq, zmq, kafka, load balancer and proxy

pub mod peer;
pub mod parathread;
pub mod rafael;
pub mod unixerr;
