












use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case




///// NOTE - 32 bytes means 256 bits and 64 characters cause every 4 bits in one byte represents one digit in hex thus 00000000 means 0x00 which is 2 characters in hex and 32 bytes hex string means 64 characters
///// NOTE - borsh like codec ops : Box<[u8]> (automatic lifetime) or &'a [u8] <-> vec[u8] <-> struct
///// NOTE - &[u8] bytes to str using str::from_utf8() -> parse it and build the key:value hashmap -> build the struct instance from the hashmap
///// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value ----serde_json::to_string()----> json string or &str ----serde_json::from_str()----> struct
///// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value or json!({}) ----serde_json::from_value()---->  struct
///// NOTE - deserialization using slice       : &[u8] buffer ----serde_json::from_slice()----> struct
///// NOTE - serializing                       : struct instance ----serde_json::to_vec()----> Vec<u8> which will be coerced to &[u8] at compile time
///// NOTE - serializing                       : struct instance ----serde_json::to_string()----> json string will be coerced to &str at compile time 
///// NOTE - serializing                       : serde_json::json!({})----> Value ----serde_json::to_string()----> json string or &str ----serde_json::from_str()----> struct








//// since dynamic types like Vec, String and traits are on the heap, 
//// by passing them into new scopes they will be moved by rust compiler 
//// from the heap in order to free the memory location that they've just 
//// allocated to free up some huge space at runtime and this why rust doesn't
//// have garbage collector and use borrowing and ownership rule instead of that 
//// hence due to this nature it'll let us have a safe concurrency and 
//// it'll change the way of coding a little bit since you have to use 
//// tokio job queue channels to pass and share Arc<Mutex<T>>: Send + Sync + 'static
//// between threads in order to access them later outside the threads in other 
//// scopes also if we move the type the lifetime of that will be dropped 
//// due to not having garbage collector feature;
//// the solution is to borrow the ownership of them from where they are stored 
//// either by cloning and move that clone between other scopes also 
//// which is expensive or by taking a reference to them using
//// as_ref() method or putting & behind them to create a pointer which will 
//// point to the location of their heap area and is good to know that their 
//// pointers are fat ones since extra bytes which has been dedicated to their 
//// length inside the heap are in their pointers also their pointers must have 
//// valid lifetime across scopes and threads in order to avoid dangling pointer 
//// issue since we can't return a pointer from a scope which is owned by a that scope 
//// to fix this we can either by defining a lifetime in struct, enum 
//// fields or function signatur or by putting them inside the Box 
//// (with dyn keyword for trait) which is a smart pointer and 
//// have a valid lifetime in itself; as_ref() will convert the type into a shared 
//// reference by returning the T as &T which we can't move out of it when it's being 
//// used by other scopes and threads thus we have to dereferene it either by * or 
//// cloning if the Clone trait is implemented for that, otherwise we CAN'T 
//// dereference or move it at all because Clone is a supertrait 
//// of the Copy trait; also we MUST know this that inside a scope multiple 
//// immutable references of a type or instance can be there but only one 
//// mutable reference must be used for that instance for example inside a method
//// struct we can have multiple immutable reference of the self but only one mutable 
//// reference of the self can be used, this rule allows rust to have safe concurreny 
//// and thread safe channels like mpsc in which we can move a shareable data like 
//// Arc<Mutex<T>>: Send + Sync + 'static (the type must be cloneable, lockable and bounded 
//// to Send, Sync traits and have 'static lifetime to be valid across threads) between 
//// threads that can be read by multiple producer or multiple threads (immutable references) 
//// at the same time but only a single consumer or a single thread (mutable reference) 
//// can use that data also the receiver side of the channel is not shareable since Clone 
//// trait is not implemented for that but the sender side 
//// can be cloned and shared between threads.










//// most important issues in p2p is to solve the 
//// NAT issue, peer routing or finding and peer 
//// dialing; also the pub/sub pattern stack 
//// can be over either tokio TCP or UDP; 
//// first step is to define our p2p network behaviour 
//// to start the swarm based on that which in our case 
//// scaffolded by the gossipsub and kademlia protocols 
//// for peers communication and discovery over the internet 
//// respectively, the second step is to define all p2p events 
//// which can be happened across the network and is done 
//// inside the `P2PAppBehaviourEvent` enum also we MUST impl 
//// `From` trait for each variant of the `P2PAppBehaviourEvent` 
//// enum to return an specific variant based on the passed 
//// in event into the `from()` method, the final step is 
//// to define the event loop structure which is responsible 
//// for selecting async I/O event from the event loop
//// that might be happened across our network for execution.



#[derive(Debug, Serialize, Deserialize)] //// we'll use serde serialization and deserialization traits for json ops
pub struct P2PChainResponse{ //// local chain response from other peer - used for if someone sends us their local blockchain and use to send them our local chain
    pub blocks: Vec<Block>, //// blocks from other peers
    pub receiver: String, //// the receiver node (peer_id) of the incoming chain or blocks
}

//// if we send a LocalChainRequest with the peer_id 
//// of another node in the system, this will trigger 
//// that they send us their chain back.
#[derive(Debug, Serialize, Deserialize)]
pub struct P2PLocalChainRequest{ //// local chain request from a specific peer
    pub from_peer_id: String, //// a peer sends a request to get the local chain from other peers
}

//// since we provide a custom `out_event` thus we must implement `From` trait 
//// for each of the event types generated by the struct members to 
//// return an specific variant based on the passed in event.  
#[derive(Debug)]
#[allow(clippy::large_enum_variant)] //// this will allow large size differences between enum variants
pub enum P2PAppBehaviourEvent{
    Kademlia(KademliaEvent),
    Gossipsub(GossipsubEvent),
}

impl From<GossipsubEvent> for P2PAppBehaviourEvent{
    fn from(event: GossipsubEvent) -> Self{ //// Self refers to the `P2PAppBehaviourEvent` in which we only return the Gossipsub variant 
        P2PAppBehaviourEvent::Gossipsub(event)
    }
}

impl From<KademliaEvent> for P2PAppBehaviourEvent{
    fn from(event: KademliaEvent) -> Self{ //// Self refers to the `P2PAppBehaviourEvent` in which we only return the Kademlia variant 
        P2PAppBehaviourEvent::Kademlia(event)
    }
}

//// NetworkBehaviour trait defines the behaviour of the local node on the network
//// in contrast to Transport which defines how to send bytes on the network, 
//// NetworkBehaviour defines what bytes to send, to whom and how! which in our case 
//// is based on gossipsub and kademlia protocols. 
//
//// to find other hosts and nodes on the local network we can either
//// use mDNS algorithm which will send a multicast UDP message on port 5353
//// announcing its presence a DHT kademlia based algorithm can be used as a 
//// replacement for mDNS which works over the internet.
#[derive(NetworkBehaviour)] //// implementing the NetworkBehaviour trait for the `P2PAppBehaviour` struct
#[behaviour(out_event = "P2PAppBehaviourEvent")] //// exporting our network behaviour the `P2PAppBehaviour` struct as `P2PAppBehaviourEvent`
pub struct P2PAppBehaviour{
    pub gossipsub: Gossipsub, //// gossipsub protocol for p2p pub/sub: https://docs.libp2p.io/concepts/pubsub/overview/
    // pub mdns: mdns::tokio::Behaviour, //// mDNS protocol is used to discover other nodes on the local network that support libp2p
    pub kademlia: Kademlia<MemoryStore>, //// kademlia DHT protocol to find peers on the internet
}

impl P2PAppBehaviour{

    pub async fn new() -> Self{ //// it'll return a new app behaviour 

        let message_id_fn = |message: &GossipsubMessage|{
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string()) //// we can take the hash of message and use it as an id
        };

        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) //// hearbeat every 10 seconds
            .validation_mode(ValidationMode::Strict) //// this requires the message author to be a valid PeerId and to be present as well as the sequence number aslo all messages must have valid signatures
            //// message_id_fn is a user-defined function allowing the user to specify 
            //// the message id of a gossipsub message, the default value 
            //// is to concatenate the source peer id with a sequence number, 
            //// setting this parameter allows the user to address packets arbitrarily, 
            //// one example is content based addressing, where this function may be 
            //// set to hash(message). This would prevent messages of the same content 
            //// from being duplicated.
            //
            //// the function takes a GossipsubMessage as input and outputs a String 
            //// to be interpreted as the message id.
            .message_id_fn(message_id_fn)
            .build()
            .unwrap();

        //// building the network behaviour
        //// based on gossipsub and kademlia protcols.
        let mut behaviour = Self{
            gossipsub: Gossipsub::new( //// building a gossipsub network behaviour
                MessageAuthenticity::Signed(
                    KEYS.clone()), //// since the Clone trait is implemented for Topic struct hence we can clone the KEYS to dereference it   
                    gossipsub_config).unwrap(),
            // mdns: mdns::Behaviour::new(mdns::Config::default()).unwrap(), //// building a default mdns
            kademlia: Kademlia::new(PEER_ID.clone(), MemoryStore::new(PEER_ID.clone()))
        };

        //// in p2p every peer is a subscriber and publisher
        //// at the same time thus we must subscribe to topics
        //// that has been published by other peers below we're 
        //// subscribing to chain, network stat, transaction 
        //// and all slot operations topic using gossipsub protocol.
        behaviour.gossipsub.subscribe(&CHAIN_TOPIC.clone()).unwrap();
        behaviour.gossipsub.subscribe(&NETWORK_STAT.clone()).unwrap();
        behaviour.gossipsub.subscribe(&TRANSACTION_TOPIC.clone()).unwrap();
        behaviour.gossipsub.subscribe(&SLOT_TOPIC.clone()).unwrap();

        behaviour

    }

}


//// in order to handle async I/O task events 
//// streaming we need an event loop structure
//// which can handle all the async events 
pub struct P2PSwarmEventLoop{
    //// following channels will be used to send and receive
    //// node initialization signal in which once we receive
    //// an initialization signal we'll publish a local chain 
    //// request to the whole network to all peers to get 
    //// the latest chain of peers who subscribe to the
    //// topic, in the mean while every node will send its  
    //// local chain to the `P2PChainResponse` mpsc channel 
    //// using the `response_sender` and once we received 
    //// the local chain response on the downside of the 
    //// channel we'll publish the chain topic to the 
    //// whole network, the `P2PChainResponse` mpsc channel 
    //// will be used to send and receive local chain responses 
    //// that has been requested by a node inside the init channel, 
    //// between different parts of the app like differen threads
    //// of the swarm events. 
    //
    //// oneshot channel is a single producer 
    //// and single consumer job queue channel
    //// in which a single value can be sent and
    //// a single receiver can use it, in our case
    //// the type of the channel is a Result with 
    //// () as the Ok arm and Box<dyn std::error::Error + Send + Sync + 'static>
    //// as the Err arm which might be caused during 
    //// the app life cycle if there was any std error.
    pub init_receiver: mpsc::Receiver<bool>,
    pub response_sender: mpsc::Sender<P2PChainResponse>,
    pub response_receiver: mpsc::Receiver<P2PChainResponse>,
    pub swarm: Swarm<P2PAppBehaviour>,
    pub parachain: ActorRef<ParachainMsg>, //// parachain actor is the back bone of the coiniXerr node
    pub actor_sys: ActorSystem,
    pub nodes: Vec<PeerId>,
    pub pending_dial: HashMap<PeerId, oneshot::Sender<Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>>,
    pub pending_start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    pub pending_get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
}

impl P2PSwarmEventLoop{

    pub fn new(swarm: Swarm<P2PAppBehaviour>, 
                init_receiver: mpsc::Receiver<bool>,
                parachain: ActorRef<ParachainMsg>,
                actor_sys: ActorSystem) -> Self{
        let buffer_size = daemon::get_env_vars().get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();            
        let (response_sender, mut response_receiver) = mpsc::channel::<P2PChainResponse>(buffer_size);
        Self{
            init_receiver,
            response_sender,
            response_receiver,
            swarm,
            parachain,
            actor_sys,
            nodes: Vec::default(),
            pending_dial: Default::default(),
            pending_start_providing: Default::default(),
            pending_get_providers: Default::default(),
        }
    }

    pub async fn run(&mut self){ //// swarm.select_next_some() is a mutable method thus we must define the self as mutable

        info!("➔ ⌨️ enter messages via STDIN and they will be sent to connected peers using Gossipsub");
        let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
        loop{
            //// inside select! macro we can have multiple async tasks and 
            //// once an async task inside the select! macro gets completed
            //// remaining tasks will be canceled and the selected task will
            //// be poped out of the queue for execution inside the function
            //// execution stack. 
            tokio::select!{
                //// event_name = async_expression => {} 
                //// event_name = (async_expression), if a.is_none() => {}
                //// - the async_expression must be a future 
                //// - an event inside the loop can be either stdin, 
                ////    local chain response comming from the mpsc channel, 
                ////    receiving init signal or swarm events.
                line = stdin.next_line() => { //// //// in here the async task is reading from the stdin; handling the stdin event if there was some input
                    match line.unwrap().unwrap() {
                        cmd if cmd.starts_with("ls p") => self.print_peers().await,
                        cmd if cmd.starts_with("ls c") => self.print_chain().await,
                        _ => error!("🛑 unknown command"),
                    }                 
                },
                //// once we received the init signal from the channel 
                //// we'll publish a chain request to the network to get
                //// the latest chain from other peers.
                local_chain_request = self.init_receiver.recv() => { //// in here the async task is receiving from the init mpsc channel
                    let peers = self.list_peers().await;
                    info!("⚛️ connected nodes {}", peers.len());
                    if !peers.is_empty(){
                        let chain_request = P2PLocalChainRequest{
                            from_peer_id: peers
                                .iter()
                                .last() //// getting the last peer_id to send the request with
                                .unwrap()
                                .to_string(),
                        };
                        let json_request = serde_json::to_string(&chain_request).unwrap();
                        self.swarm
                            .behaviour_mut() //// it'll return a mutable reference to the swarm behaviour
                            .gossipsub
                            .publish(CHAIN_TOPIC.clone(), json_request.as_bytes()) //// publishing requested chain from selected peer to the p2p network using gossipsub protocol   
                            .unwrap(); 
                    }
                },
                //// in here we're contantly receiving from the `P2PChainResponse`
                //// mpsc channel and if there was a local chain response has sent to 
                //// the channel we'll publish it to the whole network to all peers
                //// so they can get the response and update their chain with the new one.
                //// usually the new chain will be sent to this channel when we're handling
                //// GossipsubEvent::Message of the swarm event by decoding the published 
                //// CHAIN_TOPIC through the network when another peer send a local chain 
                //// request to update its chain (if it's passed the update criteria) 
                //// with the latest one from a peer.        
                local_chain_response = self.response_receiver.recv() => { //// in here the async task is receiving from the response mpsc channel 
                    //// converting the received response into the json string, 
                    //// we'll publish its bytes through the whole network later 
                    let json_response = serde_json::to_string(&local_chain_response).unwrap(); 
                    self.swarm
                            .behaviour_mut() //// it'll return a mutable reference to the swarm behaviour
                            .gossipsub
                            .publish(CHAIN_TOPIC.clone(), json_response.as_bytes()) //// publishing recieved chain response from other peers to the p2p network using gossipsub protocol 
                            .unwrap(); 
                },
                swarm_event = self.swarm.select_next_some() => { //// //// in here the async task is the swarm event; handling the swarm events if there was some
                    info!("🤌 handling a swarm event {:?}", swarm_event);
                    self.handle_event(swarm_event).await; //// calling handle_event() here to handle all swarm events
                },
            }
       }

    }

    async fn handle_event(&mut self, event: SwarmEvent<P2PAppBehaviourEvent, EitherError<GossipsubHandlerError, std::io::Error>>){
        //// following we're taking care of the swarm events 
        //// both kademlia and gossipsub events which is one of 
        //// the variant inside the `P2PAppBehaviourEvent`.
        match event{ 
            //// in here our node will subscribe to all topics that has been published
            //// using gossipsub protocol like subscribing to CHAIN_TOPIC to send 
            //// and receive latest local chains.
            /* 
                
                A NOTE ABOUT ENUM MATCHING ON STRUCT FIELD
                ------------------------------------------

                //// using its own fields' names 
                //// for unpacking on struct arm
                GossipsubEvent::Message{ 
                    propagation_source, 
                    message_id, 
                    message,
                }

                //// we can also give another names to 
                //// the current struct fields using `:` 
                //// for unpacking on struct arm
                GossipsubEvent::Message{ 
                    propagation_source: peer_id, 
                    message_id: id, 
                    message,
                }
            
            */
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Gossipsub(GossipsubEvent::Message{ //// the encoded message that we've received from other peers
                propagation_source, //// this is the peer_id
                message_id, //// this is the message_id
                message, //// this is the message itself
            })) => {
                //// deserializing the message.data related to CHAIN_TOPIC
                //// that has been published through the network, if we could 
                //// decode it into the `P2PChainResponse` structure means that
                //// we're receiving a local chain from `propagation_source` or a peer.
                if let Ok(chain_response) = serde_json::from_slice::<P2PChainResponse>(&message.data){ //// decode incoming message that has been published inside the event loop
                    if chain_response.receiver == PEER_ID.to_string(){ //// the message receiver must be this peer 
                        info!("🗨️ a chain response from {:?}", message.source);
                        chain_response.blocks.iter().for_each(|b| info!("{:?}", b)); //// logging each received block
                        let mut parachain_data = self.get_parachain_data().await;
                        let Some(mut blockchain) = parachain_data.blockchain else{ //// choose_chain() method is mutable thus blockchain must be mutable 
                            panic!("⛔ no chain at all :/");
                        };
                        //// blockchain lifetime is accessible from here
                        //// of course if there was some :)
                        let choosen = blockchain.choose_chain(chain_response.blocks); //// choosing the right chain between the local chain and the received chain
                        blockchain.blocks = choosen;
                        parachain_data.blockchain = Some(blockchain);
                    }
                } 
                //// deserializing the message.data related to CHAIN_TOPIC
                //// that has been published through the network, if we could 
                //// decode it into the `P2PLocalChainRequest` structure means that
                //// we're sending a local chain to `propagation_source` or a peer.
                else if let Ok(chain_request) = serde_json::from_slice::<P2PLocalChainRequest>(&message.data){
                    let sender_peer_id = chain_request.from_peer_id;
                    if PEER_ID.clone().to_string() == sender_peer_id{
                        let mut parachain_data = self.get_parachain_data().await;
                        //// sending a local chain response to the peer
                        //// that has initialized the request to downside 
                        //// of the `P2PChainResponse` mpsc job queue channel,
                        //// we'll publish the response to the whole network 
                        //// once we received the `P2PChainResponse` instance
                        //// from the channel which will be handled asyncly 
                        //// inside the tokio::select! event loop since sending 
                        //// and receiving between different part of the app 
                        //// will be done asyncly by tokio channels. 
                        if let Err(e) = self.response_sender.send(P2PChainResponse{ 
                            blocks: parachain_data.blockchain.unwrap().blocks, 
                            receiver: message.source.unwrap().to_string(), 
                        }).await{
                            error!("⛔ error sending p2p chain response to the job queue channel caused by: {}", e);
                        }
                    }
                }

                // TODO - decode other topic messages to their struct
                // ...

            },
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed{ 
                    id, 
                    result: QueryResult::StartProviding(_), 
                    .. //// rest of the fields of `OutboundQueryProgressed` struct variant that we don't care about
                },
            )) => {
                let sender: oneshot::Sender<()> = self
                    .pending_start_providing
                    .remove(&id)
                    .expect("❌ key with this id MUST be exists");
                let _ = sender.send(());    
            },
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed{ 
                    id, 
                    result: QueryResult::GetClosestPeers(result), //// naming the value of the GetClosestPeers variant with result
                    .. //// rest of the fields of `OutboundQueryProgressed` struct variant that we don't care about
                },
            )) => {
                match result{
                    Ok(ok) => {
                        if !ok.peers.is_empty(){
                            self.nodes = ok.peers.clone();
                            info!("⚛️ query finished closest peers {:#?}", ok.peers);
                        } else{
                            info!("😧 query finished with no closest peers");
                        }
                    },
                    Err(GetClosestPeersError::Timeout { peers, .. }) => { //// in unpacking Timeout we only care about the peers and the rest of fields can be filled with `..`
                        if !peers.is_empty(){
                            info!("⚛️⌛ query timed out with closest peers {:#?}", peers);
                        } else{
                            info!("😧⌛ query timed out with no closest peers {:#?}", peers);
                        }
                    }
                }
            },
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed{ 
                    id, 
                    result: QueryResult::GetProviders(
                        Ok(
                            GetProvidersOk::FoundProviders{ 
                            providers,
                            .. //// rest of the fields of `FoundProviders` struct variant that we don't care about
                        })), 
                    .. //// rest of the fields of `OutboundQueryProgressed` struct variant that we don't care about
                },
            )) => {
                if let Some(sender) = self.pending_get_providers.remove(&id){
                    sender.send(providers).expect("❌ the receiver MUST not be dropped");
                    self.swarm
                            .behaviour_mut()
                            .kademlia
                            .query_mut(&id) //// the query id
                            .unwrap()
                            .finish() //// finishes the query asap, without waiting for the regular termination conditions.
                }                
            },
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed{ 
                    id, 
                    result: QueryResult::GetProviders(
                        Ok(
                            GetProvidersOk::FinishedWithNoAdditionalRecord{ .. },        
                        )), 
                    .. //// rest of the fields of `OutboundQueryProgressed` struct variant that we don't care about
                },
            )) => {},
            SwarmEvent::Behaviour(P2PAppBehaviourEvent::Kademlia(_)) => {}, //// other kademlia events that we don't care about
            SwarmEvent::ConnectionEstablished{ //// if the dialing attempt succeeds
                    peer_id, 
                    endpoint, 
                    .. //// we can use .. to cover all or the remaining struct fields
            } => {
                if endpoint.is_dialer(){
                    if let Some(sender) = self.pending_dial.remove(&peer_id){ //// remove will return the value of the removed key
                        let _ = sender.send(Ok(()));
                    }
                }                
            },
            SwarmEvent::OutgoingConnectionError{ //// if the dialing attempt filed
                    peer_id, 
                    error, 
                    .. 
            } => {
                if let Some(peer_id) = peer_id{
                    if let Some(sender) = self.pending_dial.remove(&peer_id){ //// remove will return the value of the removed key
                        let _ = sender.send(Err(Box::new(error)));
                    }
                }
            },
            SwarmEvent::NewListenAddr{ address, .. } => {
                let local_peer_id = *self.swarm.local_peer_id(); //// dereferencing the self to get the local peer id of the current swarm
                info!("local node is listening on {:?}", address.with(Protocol::P2p(local_peer_id.into())));
            },
            SwarmEvent::ConnectionClosed{ .. } => {}, 
            SwarmEvent::IncomingConnection{ .. } => {},
            SwarmEvent::IncomingConnectionError{ .. } => {},
            SwarmEvent::Dialing(peer_id) => info!("📞 dialing {}", peer_id),
            e => panic!("⛔ {e:?}"),
        }
    }

    async fn list_peers(&self) -> Vec<String>{
        let mut unique_peers = HashSet::new();
        info!("⚛️ discovered nodes {}", self.nodes.len());
        //// cannot move out of `self.nodes` which is behind a shared reference
        //// thus we MUST dereference it by getting a clone of it and move 
        //// that clone between other scopes.
        for peer in self.nodes.clone(){
            unique_peers.insert(peer);
        }
        unique_peers.iter().map(|p| p.to_string()).collect() //// iterate over all nodes to convert each peer_id to string then collect them as Vec<String>
    }

    async fn print_peers(&self){
        let peers = self.list_peers().await;
        peers.iter().for_each(|p| info!("🎡 peer : {}", p));
    }

    async fn print_chain(&self){
        info!("🔗🧊 local blockchain");
        let parachain_data = self.get_parachain_data().await;
        let Some(chain) = parachain_data.blockchain else{ //// in later scopes we can access the chain
            panic!("⛔ no chain is available inside the parachain");
        };
        let blockchain_pretty_json = serde_json::to_string_pretty(&chain).unwrap();
        info!("{}", blockchain_pretty_json);
    }

    async fn get_parachain_data(&self) -> Parachain{
        info!("🔗🌉 getting the data of the parachain actor");
        let fetch_parachain_remote_handle: RemoteHandle<Parachain> = ask(&self.actor_sys, &self.parachain, ParachainCommunicate{id: Uuid::new_v4(), cmd: ParachainCmd::GetSelf}); //// we're asking the parachain actor to send us the parachain data
        let parachain_instance = fetch_parachain_remote_handle.await;
        parachain_instance
    }

    pub async fn get_providers(&mut self, parachain_id: Uuid) -> HashSet<PeerId>{
        //// performs a lookup for providers 
        //// of a value to the given chain_id
        //// returning the QueryId of the initial query 
        //// that announces the local node as a provider.
        //
        //// we'll create a result based oneshot job queue channel just 
        //// to make sure that either everything went ok or wrong 
        //// between different parts of the app whenever we want to 
        //// get all the providers of the passed in parachain id.
        let (sender, receiver) = oneshot::channel();
        let query_id = self
            .swarm
            .behaviour_mut()
            .kademlia
            .get_providers(parachain_id.to_string().into_bytes().into());
        self.pending_get_providers.insert(query_id, sender);
        //// receiving the value which is either Ok() or Err() 
        //// sent by the sender inside the swarm event loop
        receiver.await.expect("❌ sender MUST not be dropped")
    }

    pub async fn start_providing(&mut self, parachain_id: Uuid){
        //// establishes the local node as a provider 
        //// of a value for the given chain_id, this operation 
        //// publishes a provider record with the given 
        //// chain_id and identity of the local node to the peers 
        //// closest to the passed in chain_id, thus establishing the 
        //// local node as a provider; returns Ok if a 
        //// provider record has been stored locally, 
        //// providing the QueryId of the initial query 
        //// that announces the local node as a provider.
        //
        //// we'll create a result based oneshot job queue channel just 
        //// to make sure that either everything went ok or wrong 
        //// between different parts of the app whenever we want to 
        //// advertising by being a provider for the passed in parachain id.
        let (sender, receiver) = oneshot::channel();
        let query_id = self
            .swarm
            .behaviour_mut()
            .kademlia
            .start_providing(parachain_id.to_string().into_bytes().into())
            .expect("❌ there MUST be no kademlia store error");
        self.pending_start_providing.insert(query_id, sender);
        //// receiving the value which is either Ok() or Err() 
        //// sent by the sender inside the swarm event loop 
        receiver.await.expect("❌ sender MUST not be dropped"); 
    }

    //// following method will be used if the user
    //// passed an address from the terminal to 
    //// dial to a peer with.
    pub async fn dial(&mut self, peer_id: PeerId, peer_addr: Multiaddr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{
        let (sender, receiver) = oneshot::channel();
        //// if there was no entry with this peer_id was inside 
        //// the map simply we'll add its address to the DHT
        if let hash_map::Entry::Vacant(e) = self.pending_dial.entry(peer_id){ 
            self.swarm
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, peer_addr.clone()); //// adding a known listen address of a peer participating in the DHT to the routing table
            match self
                .swarm
                .dial(peer_addr.with(Protocol::P2p(peer_id.into()))){
                    Ok(()) => {
                        e.insert(sender); //// add the entry with this peer_id into the map
                    },
                    Err(e) => {
                        let _ = sender.send(Err(Box::new(e)));
                    }
                }
        } else{
            info!("📞 already dialing peer, not need to insert into  the map");
        }
        receiver.await.expect("❌ sender MUST not be dropped")
    }
}


#[derive(Debug, Clone)]
pub struct Staker{
    pub id: Uuid,
    pub deposit: i32,
    pub owner: Validator, //// delegator or owner or staker is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voter{
    pub parachain_id: Uuid, //// voter will vote in this parachain using delegator stakes
    pub owner: Validator, //// owner is a Validator
    pub rewards: Option<i32>,
    pub signed_at: Option<i64>,
    pub staker_id: Option<Uuid>, //// delegator id who staked his/her money for this voter
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Slot{ //// pool of validators for slot auctions
    pub id: Uuid,
    pub name: String,
    pub voters: Vec<Voter>, //// auction voters for this slot
    pub epoch: u32, //// one epoch is the time taken to process 600k blocks which might takes 1 day or less depends on the coiniXerr network performence, after end of each epoch a new slot auction process will be started 
} 

impl Slot{

    //// we've cloned the self.validators and current_validators to prevent ownership moving
    pub fn get_validator(&self, validator_peer_id: String) -> Option<Validator>{
        let current_voters = self.voters.clone();
        let index = current_voters.iter().position(|v| v.owner.peer_id == validator_peer_id); //// this user has already participated in this event
        if index != None{
            Some(current_voters[index.unwrap()].clone().owner) //// returning the validator of the passed in socket address
        } else{
            None
        }
    }

    pub fn add_validator(&mut self, pid: Uuid, validator_peer_id: String) -> Self{
        
        //// building a new voter to push into the voters 
        let new_voter = Voter{
            parachain_id: pid,
            owner: Validator{
                peer_id: validator_peer_id,
                recent_transaction: None, //// it must be filled inside the stream channel the receiver side once the his/her incoming transaction gets signed
                mode: ValidatorMode::Mine,
                ttype_request: None, //// it must be filled inside the transaction mempool channel the receiver side once the transaction arrived
            },
            rewards: Some(0),
            signed_at: Some(chrono::Local::now().naive_local().timestamp()),
            staker_id: None,
        };

        self.voters.push(new_voter);
        Self { id: self.id, name: self.name.clone(), voters: self.voters.clone(), epoch: self.epoch }
    }

}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InsertParachainInfo{
    pub id: Uuid,
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub next_parachain_id: Option<Uuid>, //// next parachain uuid
    pub current_block: Option<Block>,
}

#[async_trait]
impl StorageModel for InsertParachainInfo{

    type AppStorage = Option<Arc<Storage>>; //// the type of the AppStorage GAT is the Arc-ed Storage inside the Option since we don't know the exact engine in runtime 

    async fn save(&self) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>{ 
        let data = InsertParachainInfo{ //// building the instance from self since insert_one() method gets T not &T
            //// we must clone each field to prevent the self ownership from moving 
            //// since Copy is not implemented for InsertParachainInfo struct.
            id: self.id,
            slot: self.slot.clone(),
            blockchain: self.blockchain.clone(),
            next_parachain_id: self.next_parachain_id.clone(),
            current_block: self.current_block.clone(),
        };
        let unwrapped_storage = APP_STORAGE.clone().unwrap(); //// unwrapping the app storage to create a db instance
        let db_instance = unwrapped_storage.get_db().await.unwrap(); //// getting the db inside the app storage; it might be None
        let parachains = db_instance.clone().database(daemon::get_env_vars().get("DB_NAME").unwrap()).collection::<schemas::InsertParachainInfo>("parachains");
        match parachains.insert_one(data.clone(), None).await{ //// serializing the user doc which is of type RegisterRequest into the BSON to insert into the mongodb
            Ok(insert_result) => Ok(insert_result),
            Err(e) => Err(e)
        }
    } 

    //// we can either implement traits for the type by using them inside the crate 
    //// and use `impl for` them or bound the type in function signature to those 
    //// traits using where clause.
    async fn fetch(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized{

        todo!()

    }

    async fn filter(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized{

        todo!()

    }

}


#[derive(Serialize, Deserialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Chain{
    pub branch_id: Uuid, //// chain id
    pub branch_name: String,
    pub blocks: Vec<Block>,
}

impl Chain{
    
    pub fn default() -> Self{
        Chain{
            branch_id: Uuid::new_v4(),
            branch_name: format!("cc-{}", rand::thread_rng().gen::<u32>().to_string()),
            blocks: vec![Block::default()],
        }
    }

    pub fn new(branch_id: Uuid, branch_name: String, blocks: Vec<Block>) -> Self{ //// constructor of Chain struct - creating another branch or fork
        Chain{
            branch_id,
            branch_name,
            blocks,
        }
    }
    
    pub fn add(&mut self, block: Block) -> Self{ //// the first param is a mutable pointer to every field of the struct - self takes a copy of all fields and &mut borrow the ownership of those fields for mutating them
        self.blocks.push(block);
        Chain{
            branch_id: self.branch_id,
            branch_name: self.branch_name.clone(), //// Copy trait is not implemented for String thus we have to clone it to return from the function
            blocks: self.blocks.clone(), //// Copy trait is not implemented for blocks thus we have to clone it to return from the function
        }
    }

    pub fn get_genesis(&self) -> Block{
        let genesis = self.blocks[0].clone(); //// cloning the self.blocks[0] to prevent ownership moving since &self is an immutable reference to self which is a shared reference (means it has a valid lifetime and is being used by other methods) and can't be moved  
        genesis
    }

    pub fn build_raw_block(&self, prev_block: &Block) -> Block{ //// this method get an immutable pointer to the block (borrowed) as its second argument 
        Block{
            id: Uuid::new_v4(),
            index: prev_block.clone().index + 1, //// we have to clone the prev_block cause Block struct doesn't implement the Copy trait
            is_genesis: false,
            prev_hash: prev_block.clone().hash, //// first block inside the chain is the genesis block - we have to clone the prev_block cause Block struct doesn't implement the Copy trait 
            hash: None,
            merkle_root: None, 
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![],
            is_valid: false,
        }
    }

    fn is_chain_valid(&mut self, chain: &[Block]) -> bool{
        for i in 0..chain.len(){
            if i == 0{ 
                continue; //// go to the next iteration since we want to get the block before i
            }
            let first_block = chain
                                        .get(i - 1) //// getting the first block
                                        .expect("❌ first block MUST be exits");
            let next_block = chain
                                        .get(i)
                                        .expect("❌ next block MUST be exits");
            
            //// since the is_block_valid() is a mutable method
            //// we have to call it on a borrowed mutable instance 
            //// of the Block hence we have to first dereference 
            //// the next_block which is of type &Block using to_owned() 
            //// method then convert it into a &mut Block by calling 
            //// the into_mut_ref() on the instance method.
            let mut next_block_owned = next_block.to_owned(); //// converting the next_block which is &Block into Block  
            let next_block_mut_ref = next_block_owned.into_mut_ref(); //// borrowing the owned next block as mutable
            if !next_block_mut_ref.is_block_valid(first_block){ //// we can't borrow as mutable if it's borrowed as immutable
                //// since we have only one if block 
                //// we must return something using the 
                //// `return` keyword only.
                return false;
            }
        }
        true
    }

    //// always choose the longest chain,
    //// remote is the incoming chain from 
    //// other peers and local is the chain 
    //// inside this peer.
    //
    //// we can't borrow self as mutable when
    //// we're borrowing it as immutable.
    pub fn choose_chain(&mut self, remote: Vec<Block>) -> Vec<Block>{
        let local_chain = &self.blocks as &[Block]; //// this is an immutable borrow which will be used in an `if` condition 
        let is_local_valid = self.is_chain_valid(&local_chain);
        let is_remote_valid = self.is_chain_valid(&remote);
        if is_local_valid && is_remote_valid {
            if local_chain.len() >= remote.len(){
                local_chain.to_vec() //// we'll choose the local chain as the longest chain by converting it back to the Vec
            } else{
                remote //// we'll choose the incoming chain from other peers as the longest chain
            }
        } else if is_remote_valid && !is_local_valid{
            remote //// we must replace the current blocks in this peer with the incoming one since the remote chain is a valid chain
        } else if !is_remote_valid && is_local_valid{
            local_chain.to_vec() //// we must keep the current blocks by converting it back to the Vec in this peer same as before since the remote chain is an invalid chain
        } else{
            error!("⛔ local and remote chain are both invalid");
            panic!("⛔ local and remote chain are both invalid"); //// when we use panic there is no need to return something else
        }
    }

}


#[derive(Serialize, Deserialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Block{
    pub id: Uuid,
    pub index: u32,
    pub is_genesis: bool,
    pub prev_hash: Option<String>, //// 
    pub hash: Option<String>,
    pub merkle_root: Option<String>, //// hash of all transactions in the form of a binary tree-like structure called merkle tree such that each hash is linked to its parent following a parent-child tree-like relation
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //// valid transactions (came through mempool channel) waiting to be confirmed and signed by the node time - can't implement the Copy trait for Vec thus can't bound it to the Block structure 
    pub is_valid: bool,
}

impl Block{

    pub fn push_transaction(&mut self, transaction: Transaction) -> Self{ //// the first param is a mutable pointer to every field of the struct
        self.transactions.push(transaction);
        Block{ //// don't return &self when constructing the struct cause we'll face lifetime issue for struct fields - &mut T is not bounded to Copy trait due to ownership and borrowing rules which we can't have multiple mutable pointer at the same time
            id: self.id,
            index: self.index,
            is_genesis: self.is_genesis,
            prev_hash: Some(self.prev_hash.clone().unwrap()), //// self.prev_hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            hash: Some(self.hash.clone().unwrap()), //// self.hash is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            merkle_root: Some(self.clone().merkle_root.unwrap()), //// self.merkle_root is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for String thus we have to clone it
            timestamp: self.timestamp,
            transactions: self.transactions.clone(), //// self.transactions is behind a mutable reference (&mut self in function param) which doesn't implement Copy trait (can't have a multiple mutable pointer a time) for Vec thus we have to clone it 
            is_valid: self.is_valid,
        }
    }

    //// this method will be used for generating the hash of the 
    //// block from the json string of the block 
    //// or the instance of the Block struct.
    fn serialize_block(&self) -> String{ 
        //// all the block data except hash must 
        //// be convert to the string first
        //// in order to generate its hash.
        let block_data = serde_json::json!({ //// making a serde Value from the block data except the hash field since we want to fill it later
            "id": self.id,
            "index": self.index,
            "is_genesis": self.is_genesis,
            "prev_hash": self.prev_hash,
            "merkle_root": self.merkle_root,
            "timestamp": self.timestamp,
            "transactions": self.transactions,
            "is_valid": self.is_valid,
        });
        serde_json::to_string(&block_data).unwrap()
    }

    pub fn generate_hash(&mut self){
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let json_string_serialized_block = self.serialize_block(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        let block_hash_bytes = json_string_serialized_block.as_bytes(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        let hash = argon2::hash_encoded(block_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap();
        self.hash = Some(Self::cut_extra_bytes_from(hash)); //// cutting the extra byte (the first 27 bytes) from the argon2 hash
    }

    pub fn generate_genesis_hash(&mut self){
        let salt = daemon::get_env_vars().get("GENESIS_SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let json_string_serialized_block = self.serialize_block(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        let block_hash_bytes = json_string_serialized_block.as_bytes(); //// we're generating a longer lifetime since in self.serialize_block().as_bytes(); the self.serialize_block() will be dropped before we calling as_bytes() method;
        let hash = argon2::hash_encoded(block_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap();
        self.hash = Some(Self::cut_extra_bytes_from(hash)); //// cutting the extra byte (the first 27 bytes) from the argon2 hash
    }

    fn cut_extra_bytes_from(hash: String) -> String{
        //////////////
        // SAMPLE HASH
        //////////////
        // $argon2i$v=19$m=16,t=2,p=1$d2lsZG9uaW9uMjAxNw$+KaF8vyVKUdY/NqU9wf2Pg
        let hash = &hash[27..]; //// d2lsZG9uaW9uMjAxNw$+KaF8vyVKUdY/NqU9wf2Pg
        hash.to_string()
    }
    
    pub fn is_block_valid(&mut self, prev_block: &Block) -> bool{ //// since the generate_hash() method is a mutable method hence we have to define the self as mutable
        
        //// when se use .as_ref() instead of .clone() `self.hash` will 
        //// be borrowed as immutable also `self` param in this method 
        //// is behind a mutable reference thus self.generate_hash() 
        //// is borrowed as mutable which rust doesn't allow to 
        //// borrow as mutable and immutable at the same time which we 
        //// fix this by cloning the `self.hash`
        let current_block_hash = self.hash.clone(); 
        self.generate_hash(); //// this will calculate the hash of the current block again

        //// since we have separated if blocks 
        //// we must return something using the 
        //// `return` keyword only.
        if self.prev_hash != prev_block.hash{
            return false;
        } 

        if self.hash.clone().unwrap() != current_block_hash.unwrap(){ //// if the calculated block hash wasn't equal to the current hash we simply return false  
            return false;
        }
        
        if self.index != prev_block.index + 1{
            return false;
        } 
        
        //// the block must be validated using consensus algorithms first in order to be a valid
        //// since the is_valid flag MUST be set to true in there hence we have to just check 
        //// the flag again in here.
        if !self.is_valid && !prev_block.is_valid{ 
            return false;
        } 
        
        true

    }

    pub fn generate_merkle_root_hash(&mut self) -> Self{

        // TODO - generate merkle root hash of this block
        // TODO - update self.merkle_root field
        // ...

        todo!()

    }

}

impl Default for Block{
    fn default() -> Self{ //// this must be used to generate the genesis block so some field are None
        
        let prev_hash = Some(String::from("genesis"));
        let mut block = Block{ //// we got an instance of the block which can call Bloc methods on it
            id: Uuid::new_v4(),
            index: 0,
            is_genesis: true,
            prev_hash,
            hash: None, //// we'll fill this later
            merkle_root: None, //// must be filled later in consensus process
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![Transaction::default()],
            is_valid: true,
        };

        block.generate_genesis_hash(); //// updating the hash field of the genesis block
        block

    }
}


//// Rc is a smart pointer used for counting the incoming references to the type which shared its ownership using &
//// and see how many owners the borrowed type has in its entire scope as long as its lifetime is not dropped also 
//// it has nothing to do with the garbage collection rule cause rust doesn't have it.
//
//// all transactions inside a block will be stored in form of a merkle tree and since 
//// it'll chain transaction hash together is a useful algorithm for proof of chain.
#[derive(Debug)]
pub struct MerkleNode{
    pub id: Uuid,
    pub data: Transaction, //// in each node data is the transaction hash; if the data is of type Transaction just call the data.hash since the hash of the transaction has been generated in different TLPs
    pub parent: RefCell<Weak<Node>>, //// we want to modify which nodes are parent of another node at runtime, so we have a RefCell<T> in parent around the Vec<Weak<Node>> - child -> parent using Weak to break the cycle, counting immutable none owning references to parent - weak pointer or none owning reference to a parent cause deleting the child shouldn't delete the parent node
    pub children: RefCell<Vec<Rc<Node>>>, //// we want to modify which nodes are children of another node at runtime, so we have a RefCell<T> in children around the Vec<Rc<Node>> - parent -> child, counting immutable references or borrowers to childlren - strong pointer to all children cause every child has a parent which the parent owns multiple node as its children and once we remove it all its children must be removed
}

impl MerkleNode{

    pub fn is_leaf(&mut self) -> bool{
        todo!();
    }

    pub fn add_child(&mut self, node: Node){
        self.children.borrow_mut().push(Rc::new(node)); //// in order to push into the self.children field we have to borrow it as mutable at runtime since it has wrapped around the RefCell
    }

    pub fn children(&mut self, node: Node) -> Result<Vec<Rc<Self>>, String>{ //// &mut self means we're borrowing Node fields using a mutable pointer which is a shallow copy of the fields (if we change the pointer value the actual object will be changed) for updaing the desired field
        if node.children.borrow_mut().len() != 0{ //// first borrow the ownership of the self.children field at runtime then check its length
            Ok(node.children.borrow_mut().to_vec()) //// we have to borrow the ownership of the self.children field at runtime and convert it to vector to return it cause it's inside RefCell
        } else{
            Err(format!("node -{}- has no children", node.id).to_string())
        }
    }

    pub fn generate_hash(&mut self, right_node: Node) -> String{

        // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
        // https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
        // https://nomicon.io/DataStructures/MerkleProof
        // TODO - use argon2 to generate the hash of self.data.hash + right_node.data.hash
        // ...

        todo!()

    }
}


//// `Ed25519` is being used instead of `ECDSA` as the public-key 
//// (asymmetric) digital signature encryption to generate the 
//// wallet address keypair the public and private keys; the hash 
//// of the public key is being used to generate the wallet address 
//// also we **MUST** use the public key for transaction signature 
//// verification; the private key on the other hand will be used 
//// to sign transaction data (tx data **MUST** first serialized 
//// to an array of `utf8` bytes) finally for the transaction, 
//// block and `merkle` root hash and generating wallet address 
//// (hash of the public key) `Argon2` is being used as the `KDF` method.
//// I've dropped the first 27 bytes from the generated Argon2 hash 
//// to generate a real hash for the transaction, block and merkle root.
 
unsafe impl Send for TransactionMem {} //// due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads and avoid concurrent mutations.
//// all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
//// there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
//// enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself
union TransactionMem{ // https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    pub data: *mut self::Transaction, //// defining the data as a raw mutable pointer to a mutable Transaction object, changing the data will change the Transaction object and vice versa
    pub buffer: *const u8,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, Debug)] //// encoding or serializing process is converting struct object into utf8 bytes - decoding or deserializing process is converting utf8 bytes into the struct object
pub struct Transaction{
    #[borsh_skip] //// skip serializing this field since the BorshSerializer trait is not implemented for Uuid types
    pub id: Uuid,
    pub ttype: u8, //// 00000000 or 0x00 is one byte - every 4 bits in one byte is a hex number so 8 bits is 2 hex number in one byte representation bits and every 3 digit in one byte is a oct number
    pub amount: i32,
    pub public_key_bytes: Vec<u8>,
    pub from_address: String,
    pub to_address: String,
    pub issued: i64,
    pub signed: Option<i64>,
    pub signature: Option<Vec<u8>>,
    pub hash: Option<String>,
}

impl Default for Transaction{
    fn default() -> Self{ //// overriding the default method
        
        //// unwrap() method like into() and from() takes self not &self thus 
        //// it takes ownership of the `self`, which moves the value, means that 
        //// by calling the unwrap() method on COINIXERR_NODE_WALLET_KEYPAIR we 
        //// can't call other methods of the COINIXERR_NODE_WALLET_KEYPAIR instance 
        //// again since the instance will be moved, the solution is to use as_ref() 
        //// method on COINIXERR_NODE_WALLET_KEYPAIR instance to borrow its ownership
        //// the call the .unwrap() to prevent ownership moving by the first call.
        let coiniXerr_node_keypair = COINIXERR_NODE_WALLET_KEYPAIR.as_ref().unwrap(); 
        let public_key = coiniXerr_node_keypair.public_key().as_ref(); //// as_ref() converts to &[u8] which is an array or an slice of the Vec<u8> since Vec<u8> will be coerced to &[u8] at compile time
        let wallet_address = 
                Self::cut_extra_bytes_from( //// cutting the extra byte (the first 27 bytes) from the argon2 hash and use the rest as the wallet address
                    Self::generate_wallet_address_from(
                        public_key
                    ) //// generating wallet address from the public key
                );


        let mut tx = Transaction{ //// sending a transaction from to this coiniXerr node
            id: Uuid::new_v4(),
            ttype: 0x00, //// 0x00 means 0 in hex and a regular transaction - 0xFF or 1 (CRC21) and 0x02 or 2 (CRC20) and 0x03 or 3 (CRC22) in hex means smart contract transaction
            amount: 100,
            public_key_bytes: public_key.to_owned(), //// to_owned() will convert the borrowed data or &[u8] to owned data or Vec<u8>; Vec<u8> will be coerced to &[u8] at compile time to be stroed on the stack
            from_address: wallet_address.clone(), //// sender wallet address
            to_address: wallet_address, //// receiver wallet address
            issued: chrono::Local::now().naive_local().timestamp(),
            signed: Some(chrono::Local::now().naive_local().timestamp()),
            signature: None, //// transaction object needs to be signed using the sender's private key from walleXerr and this cryptographically proves that the transaction could only have come from the sender and was not sent fraudulently
            hash: None
        };


        //// we didn't save the Self::serialize_transaction_from(&tx)
        //// into a variable since the variable can't be used again
        //// to pass it to the Self::generate_hash_from() method
        //// to generate the hash because after updating signature field
        //// we must serialize the transaction data gain hence it's better
        //// to call Self::serialize_transaction_from(&tx) every time we need it!
        //  
        //// signing the data which is the serialized transaction bytes
        //// with the generated private key; the result is the transaction
        //// signature which must be verified inside the coiniXerr node.
        info!("➔ 🖊️ signing serialized transaction bytes using private key 🔑"); 
        let signature = coiniXerr_node_keypair
                                        .sign(
                                            //// at this moment when we serialize the tx
                                            //// the signature field is empty and we're 
                                            //// serializing with empty signature field
                                            //// to produce the signature.
                                            Self::serialize_transaction_from(&tx)
                                                    .as_bytes()
                                            ); //// if we call as_ref() method in here value before it will be dropped sooner thus we have to make a longer lifetime then call the as_ref() method
                                            
        let signature = signature.as_ref();
        
        //// updating the signature field before 
        //// generating the hash since in we need
        //// it to produce the hash of the tx.
        //
        //// to_owned() will convert the borrowed data or 
        //// &[u8] to owned data or Vec<u8>
        tx.signature = Some(signature.to_owned()); 
        tx.hash = Some(
                        Self::cut_extra_bytes_from( //// cutting the extra byte (the first 27 bytes) from the argon2 hash
                          Self::generate_hash_from(
                                    //// calling Self::serialize_transaction_from(&tx) again since we've updated the signature field
                                    Self::serialize_transaction_from(&tx)
                                )
                            )
                        );
        tx //// returning the default and hashed transaction 

    }
}

impl Transaction{
    
    //// a transaction decoder or deserializer using union
    pub fn new(buffer: &[u8]) -> Result<&mut Self, Box<dyn std::error::Error>>{ //// self is a copy to all values of the struct; &self is a pointer to those values means by doing this we will borrow ownership of all original values
        unsafe{ // NOTE - if neither Copy nor Clone is not implemented for the object by moving it into a function we loose the ownership of the value of that object; we can borrow the ownership by taking a pointer to it using &
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //// filling the buffer field will also fill the data cause thay have a same memory storage
            let deserialized_transaction = &mut *transaction.data; //// mutable pointer to the dereferenced transaction data - since the data inside the union is a raw pointer to a mutable Transaction object we have to dereference it to return a Transaction object; we also want to change the object later so we have to take a mutable pointer or reference (&mut) to the dereferenced object to borrow the ownership of the original object for mutation
            Ok(deserialized_transaction)
        }
    }

    //// this method will be used for generating the hash of the 
    //// transaction from the json string of the transaction 
    //// or the instance of the Transaction struct.
    fn serialize_transaction(&self) -> String{ //// we'll use this method to hash the signed transaction comming from the walleXerr
        //// all the transaction data except hash must 
        //// be convert to the string first
        //// in order to generate its hash.
        let tx_data = serde_json::json!({ //// making a serde Value from the transaction data except the hash field since we want to fill it later
            "id": self.id,
            "ttype": self.ttype,
            "amount": self.amount,
            "public_key_bytes": self.public_key_bytes,
            "from_address": self.from_address,
            "to_address": self.to_address,
            "issued": self.issued,
            "signed": self.signed,
            "signature": self.signature
        });
        serde_json::to_string(&tx_data).unwrap()
    }
     
    //// we'll use this method to serialize the passed in transaction
    //// to json string to produce an array of utf8 bytes (&[u8]) for signing it 
    //// with the private key and filling the hash field.  
    pub fn serialize_transaction_from(transaction: &Transaction) -> String{ 
        //// the return type of tx_data is Value
        //// which we'll convert it into the json string
        let tx_data = if let Some(_) = transaction.signature{ //// if the signature has some value means that we're serializing to generate the hash
            serde_json::json!({ //// this is the data that must be hashed
                "id": transaction.id,
                "ttype": transaction.ttype,
                "amount": transaction.amount,
                "public_key_bytes": transaction.public_key_bytes,
                "from_address": transaction.from_address,
                "to_address": transaction.to_address,
                "issued": transaction.issued,
                "signed": transaction.signature,
                "signature": transaction.signature
            })
        } else{ //// if the signature field was None means that we're serializing to sign the transaction with private key to generate the signature
            serde_json::json!({ //// this is the data that must be signed
                "id": transaction.id,
                "ttype": transaction.ttype,
                "amount": transaction.amount,
                "public_key_bytes": transaction.public_key_bytes,
                "from_address": transaction.from_address,
                "to_address": transaction.to_address,
                "issued": transaction.issued
            })
        };
        serde_json::to_string(&tx_data).unwrap()
    }

    pub fn generate_hash(&mut self){ //// this method must be called only inside the coiniXerr node
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let json_string_serialized_transaction = self.serialize_transaction(); //// we're generating a longer lifetime since in self.serialize_transaction().as_bytes(); the self.serialize_transaction() will be dropped before we calling as_bytes() method;
        let transaction_hash_bytes = json_string_serialized_transaction.as_bytes();
        let hash = argon2::hash_encoded(transaction_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap();
        self.hash = Some(Self::cut_extra_bytes_from(hash)); //// cutting the extra byte (the first 27 bytes) from the argon2 hash
    } 

    pub fn generate_hash_from(serialized_tx: String) -> String{ //// this method must be called only inside the coiniXerr node
        let salt = daemon::get_env_vars().get("SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        let transaction_hash_bytes = serialized_tx.as_bytes();
        argon2::hash_encoded(transaction_hash_bytes, salt_bytes, &argon2::Config::default()).unwrap()
    } 

    fn generate_wallet_address_from(pubk: &[u8]) -> String{ //// generating wallet address from the public key using Argon2
        let salt = daemon::get_env_vars().get("GENERATE_WALLET_ADDRESS_SECRET_KEY").unwrap().to_string();
        let salt_bytes = salt.as_bytes();
        argon2::hash_encoded(pubk, salt_bytes, &argon2::Config::default()).unwrap()
    }
    
    fn cut_extra_bytes_from(hash: String) -> String{
        //////////////
        // SAMPLE HASH
        //////////////
        // $argon2i$v=19$m=16,t=2,p=1$d2lsZG9uaW9uMjAxNw$+KaF8vyVKUdY/NqU9wf2Pg
        let hash = &hash[27..]; //// d2lsZG9uaW9uMjAxNw$+KaF8vyVKUdY/NqU9wf2Pg
        hash.to_string()
    }

    pub fn is_transaction_valid(&self) -> bool{

       self.verify_signature() 

    }

    fn verify_signature(&self) -> bool{
        info!("➔ 🏷️ verifying transaction signature using public-key (asymmetric) digital signature encryption based on Ed25519");
        match self.signature.clone(){ // https://cryptobook.nakov.com/digital-signatures | we'll use the self.signature later thus we have to clone it
            Some(tx_sig) => {                
                info!("➔ 🕵🏾‍♀️ verifying transaction signature using public key 🔑"); 
                let json_string_serialized_transaction = self.serialize_transaction(); //// we're generating a longer lifetime since in self.serialize_transaction().as_bytes(); the self.serialize_transaction() will be dropped before we calling as_bytes() method;
                let serialized_tx_bytes = json_string_serialized_transaction .as_bytes(); 
                if let None = self.signature{ //// we can use self.signature.is_none() also 
                    info!("➔ ⛔ null signature 🔑"); 
                    //// since we have separated if blocks 
                    //// we must return something using the 
                    //// `return` keyword only.
                    return false; 
                }
                if !self.public_key_bytes.is_empty() && self.signature.is_some(){ 
                    let peer_public_key = ring_signature::UnparsedPublicKey::new(&ring_signature::ED25519, self.public_key_bytes.clone()); //// generating the public key from the public key bytes 
                    let sig = &self.signature.clone().unwrap() as &[u8]; //// casting the &Vec<u8> to &[u8]
                    //// verifying the signature based on the 
                    //// serialized transaction bytes and the  
                    //// public key; public key will be used to 
                    //// find the private key of the signer 
                    //// which is inside the walleXerr to 
                    //// complete the verification process.
                    match peer_public_key.verify(serialized_tx_bytes, sig){ 
                        Ok(_) => true,
                        Err(_) => false
                    }
                } else{
                    false //// public address is empty which contains zero bytes
                }
            
            },
            None => false //// empty signature means false
        }
    }

}



#[derive(Clone, Debug)] //// can't bound Copy trait cause engine and url are String which are heap data structure 
pub struct Db{
    pub mode: Mode,
    pub engine: Option<String>,
    pub url: Option<String>,
    pub instance: Option<Client>,
}

impl Default for Db{
    fn default() -> Db {
        Db{
            mode: self::Mode::Off,
            engine: None,
            url: None,
            instance: None,
        }
    }
}

impl Db{
    
    pub async fn new() -> Result<Db, Box<dyn std::error::Error>>{
        Ok(
            Db{ //// building an instance with generic type C which is the type of the db client instance
                mode: Mode::On, //// 1 means is on 
                engine: None, 
                url: None,
                instance: None,
            }
        )
    }
    
    pub async fn GetMongoDbInstance(&self) -> Client{ //// it'll return an instance of the mongodb client - we set the first argument to &self in order to have the instance of the object later on after calling this method and prevent ownership moving
        Client::with_uri_str(self.url.as_ref().unwrap()).await.unwrap() //// building mongodb client instance
    }

}



#[derive(Clone, Debug)]
pub struct Storage{
    pub id: Uuid,
    pub db: Option<Db>, //// we could have no db at all
}

impl Storage{
    pub async fn get_db(&self) -> Option<&Client>{
        match self.db.as_ref().unwrap().mode{
            Mode::On => self.db.as_ref().unwrap().instance.as_ref(), //// return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client> or Option<&T>
            Mode::Off => None, //// no db is available cause it's off
        }
    }
}



#[derive(Copy, Clone, Debug)]
pub enum Mode{ //// enum uses 8 bytes (usize which is 64 bits on 64 bits arch) tag which is a pointer pointing to the current variant - the total size of this enum is 8 bytes tag + the largest variant size = 8 + 0 = 8 bytes; cause in our case On and Off variant both have 0 size
    On, //// zero byte size
    Off, //// zero byte size
}

 
