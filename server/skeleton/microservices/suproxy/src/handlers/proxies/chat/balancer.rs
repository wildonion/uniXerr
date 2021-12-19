



use rand::{self, rngs::ThreadRng, Rng};
use uuid::Uuid;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use crate::handlers::{
    db::cass::schemas::player::Chat,
    db::cass::establish as cass,
};










/* --------------------------------------------------------------------------------------------------------
    [+] every event needs a handler which is a general way to handle incoming messages, streams, and future 
        from an actor to other actor through an Addr object and the Message trait bounded 
        to our event struct defines the result type for that event.

    [+] sending events and messages between actors' threads using their address (Addr object) is done through  
        something like mpsc job queue channel.
   -------------------------------------------------------------------------------------------------------- */
// -------------
// Connect event
// -------------
#[derive(Message)]
#[rtype(usize)] //-- the response type is usize
pub struct Connect{
    pub addr: Recipient<Message>, //-- the Recipient type allows to send one specific message to an actor - the addr field is the address of UserChatSession actor
    pub room: String,
    pub username: Option<String>,
    pub user_id: i32,
    pub friend_id: i32,
}

// -------------
// Message event
// -------------
#[derive(Message)]
#[rtype(result = "()")] //-- response type
pub struct Message(pub String); //-- ChatServer sends this message to a session

// ----------------
// Disconnect event
// ----------------
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect{
    pub username: Option<String>,
    pub id: usize, //-- socket, client session or actor id
}

// -------------------
// ClientMessage event
// -------------------
#[derive(Message)]
#[rtype(result = "UUID")]
pub struct ClientMessage{
    pub id: usize, //-- socket, client session or actor id
    pub msg: String,
    pub room: String,
    pub friend_id: i32,
    pub user_id: i32,
}

// --------------------------------------
// UUID MessageResponse for Message event
// --------------------------------------
#[derive(MessageResponse)]
pub struct UUID(pub Uuid);









// ========================================================================================================================
// implementing ChatServer struct to be an actor and a load balancer to communicate with all sessions through their address
// ========================================================================================================================
pub struct ChatServer{ //-- ChatServer is an actor and maintains list of connection client session and manages available rooms, peers send messages to other peers in same room through the ChatServer actor
    sessions: HashMap<usize, Recipient<Message>>, //-- on a 32 bit target usize is 4 bytes and on a 64 bit target usize is 8 bytes
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    cass_session: Arc<cass::CassSession>,
}


impl Default for ChatServer{
    fn default() -> Self{
        todo!()
    }
}

impl ChatServer{

    pub fn new(cass_session: Arc<cass::CassSession>) -> ChatServer{
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new()); //-- to_owned() creates an owned String from a string slice or &str by cloning
        ChatServer{
            sessions: HashMap::new(), //-- all sessions are an actor which is of type Addr object, the sessions field is a hash map contains the id as the key and the address of the session actor as its value
            rooms, //-- rooms contain a name as the key and one or more set of sessions (client session id) as its value
            rng: rand::thread_rng(),
            cass_session,
        }
    }

    fn send_message(&self, room: &str, message: &str, skip_id: usize){
        if let Some(sessions) = self.rooms.get(room){ //-- getting all sessions inside an specific room
            for id in sessions{ //-- iterating through all session ids
                if *id != skip_id{ //-- skip sending message if the id is equals to the skip_id
                    if let Some(addr) = self.sessions.get(id){ //-- getting the address of the session with this id
                        let _ = addr.do_send(Message(message.to_owned())); //-- sending a message to a specific session or peer using its address (Addr object) even if the recipient's mailbox is full
                    }
                }   
            }
        }
    }
}










// ========================================================================================
// implementing Actor and all required handlers for defined events of the ChatServer struct
// ========================================================================================

impl Actor for ChatServer{
    type Context = Context<Self>; //-- we're jut using simple Context cause i'll implement the start() method for our ChatServer actor
}

// -----------------------------------------------------------
// implementing Handler for Connect event for ChatServer actor
// -----------------------------------------------------------
impl Handler<Connect> for ChatServer{
    type Result = usize; //-- the response type is usize

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result{
        self.send_message(&msg.room.clone(), &format!("{} is online", msg.username.clone().unwrap()), 0); //-- notify two peers in their room
        let id = self.rng.gen::<usize>(); //-- generating random id for the socket or actor the client session
        self.sessions.insert(id, msg.addr); //-- inserting this session with its id into sessions hash map
        self.rooms.entry(msg.room.clone()).or_insert_with(HashSet::new).insert(id); //-- inserting the id of this session into the room, if the room doesn't exist create it with a new empty hash set - an in memory room checking algorithm
        id //-- returning the generated id for the socket to update the UserChatSession actor id field 
    }
}

// --------------------------------------------------------------
// implementing Handler for Disconnect event for ChatServer actor
// --------------------------------------------------------------
impl Handler<Disconnect> for ChatServer{
    type Result = (); //-- the response type is ()

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>){
        let mut rooms_for_this_session: Vec<String> = Vec::new();
        if self.sessions.remove(&msg.id).is_some(){ //-- removing a session with a specific id
            for (name, sessions) in &mut self.rooms{ //-- iterating through all rooms
                if sessions.remove(&msg.id){ //-- if a session was found with this id inside this room remove it from the hash set list of all session in hash map rooms 
                    rooms_for_this_session.push(name.to_owned()); //-- push this room into rooms_for_this_session - to_owned() converts &self to self means &String to String
                }
            }
        }
        for room in rooms_for_this_session{
            self.send_message(&room, &format!("{} is offline", msg.username.clone().unwrap()), 0); //-- notify all users in those rooms that this id was there and this user is offline now
        }
    }
}

// -----------------------------------------------------------
// implementing Handler for Message event for ChatServer actor
// -----------------------------------------------------------
impl Handler<ClientMessage> for ChatServer{
    type Result = <ClientMessage as actix::Message>::Result; //-- the response type is UUID struct

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> UUID{
        let player_chat = Chat{ //-- creating the Chat schema from the ClientMessage event
            id: Uuid::new_v4(),
            message: msg.msg.clone(),
            user_id: msg.user_id,
            friend_id: msg.friend_id,
            room_name: msg.room.clone(),
            chattime: chrono::Local::now().naive_local().timestamp(),
        };
        let inserted_uuid = player_chat.save(self.cass_session.clone());
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
        UUID(inserted_uuid) //-- returning the inserted uuid to update UserChatSession actor inserted_message_uuid field
    }
}


