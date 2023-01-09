







use crate::*;






//// the backbone of the libp2p is something like ZMQ with pub/sub 
//// socket connections each of which is an actor communicate 
//// with each other using a socket or an RPC channels.
//
//// each lip2p node is a ZMQ or tokio socket which is an actor with concepts of
//// worker threadpool (like tokio::spawn() green based worker threadpool + 
//// tokio channels for sharing messages and tasks between threads), job or task queue for 
//// async task scheduling, pub/sub channels like socket, RPC (if actors are in not in same machine) 
//// or tokio channels (if actors are in same machine) for broadcasting async messages to other actors 
//// and mailbox to receive from other actor or outside of the actor system under the hood.
//
//// actors publisher can either broadcast topics through the RPC or socket 
//// to be subscribed by other actors inside other device or they can use 
//// tokio channels to broadcast to other actors inside the same machine.
//
//// the reason actors use RPC for communication is because with RPC we can directly call 
//// the method of an encoded object using cap'n proto or protobuf from the different devices.
//
//// in distributed networks like the one we build with libp2p, every node or socket is a pub/sub actor 
//// which will communicate with each other through message passing protocols like ZMQ or tokio sockets or RPC channels.
//// since each node is an actor object with pre defined methods encoded with a distributed object protocol 
//// like Cap'n Proto RPC or Protobuf gRPC hence to communicate with other node or actors 
//// and call each other methods directly on different machines without proxying they must use pub/sub 
//// channels through RPC like the one in chatroom, file sharing, twitter push update notifications.  
//// by using Cap'n Proto or Protobuf as the object serialization both pub/sub actors knwo the exact 
//// structure of the realtime request/response streaming between them and if they are on 
//// the same machine they can use tokio channels like mpsc, watch, oneshot and broadcast to
//// share an encoded, Send and Sync (Arc<Mutex<T>>) data between tokio workers' threadpool.
//
//// tokio channels will be used to share Arc<Mutex<T>> between multiple threads 
//// and ZMQ socket actors supports multiple connection types which can be used 
//// to communicate with other device socket actors. 
//
//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and consensus process.
pub async fn bootstrap(
        mempool_sender: broadcast::Sender<(Arc<Mutex<Transaction>>, Arc<Mutex<ActorRef<ValidatorMsg>>>, ActorSystem)>, //// we'll use this sender to send transactions to downside of the mempool channel for mining process
        storage: Option<Arc<Storage>>, 
        env_vars: HashMap<String, String>,
        current_slot: Slot, 
        validator_joined_channel: ChannelRef<ValidatorJoined>,
        default_parachain_uuid: Uuid,
        cloned_arc_mutex_runtime_info_object: Arc<Mutex<RafaelRt>>,
        meta_data_uuid: Uuid,
        cloned_arc_mutex_validator_update_channel: Arc<Mutex<ChannelRef<ValidatorUpdated>>>,
        cloned_arc_mutex_validator_actor: Arc<Mutex<ActorRef<ValidatorMsg>>>, //// the validator actor
        coiniXerr_sys: ActorSystem
    ){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////         libp2p pub/sub stream to broadcast actors' events and topics to the whole networks
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //// libp2p uses message queues and actors to handle the incoming data
    //// from other socket node actors inside a worker threadpool, also for 
    //// socket node actor communiactions and call their methods directly 
    //// it'll use RPC channels in a pub/sub manner with capnp
    //// or protobuf as the serialization protocol.
    //
    //// each instance of the socket connections or node or peer
    //// is an actor that it can handle incoming async packet from other
    //// nodes through the RPC pub/sub channel in a worker threadpool
    //// also it has a message queue like ZMQ which can schedule 
    //// the execution process of a packet inside other node
    //// using pub/sub broadcasting topic pattern.
    //
    //// topics are channels that will be broadcasted to the network
    //// using publishers so subscribers that are interested to those
    //// topics can subscribe to.

    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let swarm_addr = env_vars.get("SWARM_ADDR").unwrap().as_str();

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////         scaffolding p2p network stacks, services and requirements
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    // -----------------------------------------------------
    //         INITIALIZING MPSC JOB QUEUE CHANNELS 
    // -----------------------------------------------------
    //// since receiving is a mutable process
    //// we've defined the receiver as mutable.
    //
    //// passing data between threads can be done by using 
    //// tokio job queue channels like the following mpsc channels
    //// which can be used to broadcast network events like 
    //// local chain response comming from other nodes and 
    //// lazy initialization of the node between different 
    //// parts of the app like other threads and scopes.
    //
    //// share and schedule Arc<Mutex<T>>: Send + Sync + 'static between 
    //// tokio worker green threadpool using tokio channels.
    //// to avoid deadlock situations.
 

    info!("➔ 🎡 peer id for this node [{}]", PEER_ID.clone());
    let (response_sender, mut response_receiver) = mpsc::channel::<P2PChainResponse>(buffer_size);
    let (init_sender, mut init_receiver) = mpsc::channel::<bool>(buffer_size);
    
    // -------------------------------------------------------------------------------
    //         CREATING A SECURED MULTIPLEX PROTCOL USING TOKIO TCP AND NOISE   
    // -------------------------------------------------------------------------------
    //// creating a tokio-based TCP transport use noise for authenticated
    //// encryption based on keypairs and Mplex for multiplexing 
    //// of substreams on a TCP stream.
    //
    //// stream multiplexer protocol will be used to send 
    //// stream of information over a communication link 
    //// at the same time in the form of a 
    //// single and complex signal.
    //
    //// since dynamic type sizes are not specified at compile time in order 
    //// to pass them into other scopes and threads they must be referenced during the app
    //// either directly using & with a valid lifetime or by putting them 
    //// inside the Box which can handle the lifetime on its own like the 
    //// transport protocol which has been Boxed.

    info!("➔ 🔌🔐 building a secured transport protocol using noise and tokio TCP");
    let auth_keys = NoiseKeypair::<X25519Spec>::new()
                                                .into_authentic(&KEYS.clone())
                                                .unwrap();
    let transport = libp2pTCP::tokio::Transport::new(libp2pTCP::Config::default().nodelay(true))
                                                .upgrade(upgrade::Version::V1)
                                                .authenticate(NoiseConfig::xx(auth_keys).into_authenticated()) //// making tokio TCP mplex channel secure using generated public and private key based on noise protocol
                                                .multiplex(mplex::MplexConfig::new())
                                                .boxed(); //// put the setup config inside the Box which is a smart pointer to the config that handles the lifetime automatically

    
    // -----------------------------------------
    //      BUILDING THE NETWORK BEHAVIOUR   
    // -----------------------------------------

    info!("➔ 🔩 generating a new p2p app behaviour");
    let behaviour = P2PAppBehaviour::new().await;

    // -----------------------------------------------------------------------------------------------
    //        BUILDING THE SWARM MODULE FROM THE CREATED BEHAVIOUR, TRANSPORT AND THE PEER ID   
    // -----------------------------------------------------------------------------------------------
    //// creating a new SwarmBuilder based on tokio executor 
    //// from the given transport, behaviour and local peer_id.
    //
    //// swarm module will be the engine of the p2p network stacks
    //// which by starting it we can handle all the incoming events
    //// inside the node.
    
    info!("➔ 🌪️ building swarm module based on secured transport channel and the generated behaviour");
    let mut swarm = SwarmBuilder::with_tokio_executor(
                                                        transport, 
                                                        behaviour, 
                                            PEER_ID.clone()
                                            )
                                            .build();

    // --------------------------------------------------------
    //     READING FULL LINES FROM THE STDIN FOR USER INPUT   
    // --------------------------------------------------------

    info!("➔ ⌨️ enter messages via STDIN and they will be sent to connected peers using Gossipsub");
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    // -----------------------------------------
    //        STARTING THE CREATED SWARM 
    // -----------------------------------------
    
    swarm.listen_on(
        //// the /ip4/0.0.0.0 informs us that we want any 
        //// address of the IPv4 protocol, and /tcp/0 
        //// tells us we want to send TCP packets to any port.
        swarm_addr
                .parse()
                .unwrap(),
    ).unwrap();
    info!("➔ 👂 swarm is listening on {}", swarm_addr);
    
    //// spawning an async task or a future object thaT 
    //// can be handled in the background using 
    //// tokio worker green threadpool.
    tokio::spawn(async move{ 
        info!("➔ 📻 sending init event to downside of the mpsc channel");
        init_sender.send(true).await;
    });

    // -----------------------------------------
    //          SWARM EVENT LOOP PROCESS 
    // -----------------------------------------
    //// in event driven coding there is an event loop
    //// which constantly chanage the code flow of the app 
    //// when some async I/O events happen in which the pulled 
    //// async I/O event out of the queue will be placed onto 
    //// the function execution stack to be executed whenever 
    //// whenever the function stack becomes empty; this can be done
    //// using tokio::select! macro which waits on multiple 
    //// concurrent branches and async computation task events, 
    //// returning when the first branch or a single computation completes, 
    //// cancelling the remaining branches or async computations.
    //
    //// stream over events to read data from the wire constantly 
    //// and save then in a buffer inside the stack 
    //// then deserialize it.
    //
    //// tokio solves async tasks or future objects selected 
    //// from the event loop using tokio::select! in the background 
    //// inside worker green threadpool.
    //
    //// if we want to have concurrent and parallelism at the same time
    //// we can spawn each async expressin using tokio::spawn() and pass the
    //// returned join handle to select! macro.
    
    loop{ //// the event loop
        let event = {
            tokio::select!{



            }
        };
        
        if let Some(evt) = event{
            match evt{
                P2PBehaviourEvent::Init => {
    
                },
                P2PBehaviourEvent::LocalChainResponse(resp) => {
    
                },
                P2PBehaviourEvent::Input(line) => {
    
                },
            }
        }
    }

    
    





            
            
            
            
            
            
            

}
