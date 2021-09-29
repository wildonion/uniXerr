



use std::env;
use std::net::IpAddr;
use argon2::{self, Config, ThreadMode, Variant, Version};
use uuid::Uuid;
use std::time::{Duration, Instant};
use actix_web::{web, get, Error, HttpRequest, HttpResponse};
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use crate::handlers::proxies::chat::balancer;







const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);








// ==================================================
// implementing UserChatSession struct to be an actor
// ==================================================
pub struct UserChatSession{
    id: usize,
    hb: Instant,
    ip: IpAddr,
    port: u16,
    room: String,
    name: Option<String>,
    addr: Addr<balancer::ChatServer>,
    friend_id: i32,
    user_id: i32,
    inserted_message_uuid: Option<Uuid>,
}

impl UserChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT { //-- if now is greater than 10 seconds the we have a dead session
                println!("[!] SERVER TIME : {} | Websocket Client heartbeat failed, disconnecting!", chrono::Local::now().naive_local());
                act.addr.do_send(balancer::Disconnect { username: act.name.clone(), id: act.id }); //-- sending disconnect event to ChatServer actor
                ctx.stop();
                return;
            }
            ctx.ping(b""); //-- sending heartbeat Ping messages, if server does not receive a heartbeat message for 10 seconds connection gets dropped
        });
    }
}











// =============================================================================================
// implementing Actor and all required handlers for defined events of the UserChatSession struct
// =============================================================================================

impl Actor for UserChatSession {
    type Context = ws::WebsocketContext<Self>; //-- all actors' lifetime are controlled by the Context or ctx variable

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address(); //-- getting the address of the actor
        self.addr
            .send(balancer::Connect { //-- sending asynchronous Connect event to ChatServer actor and waits for a response
                addr: addr.recipient(), //-- address of the UserChatSession actor for Message event inside the Connect event
                room: self.room.clone(),
                user_id: self.user_id,
                friend_id: self.friend_id,  
            })
            .into_actor(self) //-- converting normal future to a UserChatSession actor
            .then(|res, act, ctx| { //-- getting the result from Connect handler, UserChatSession actor and context inside .then callback through a closure
                match res {
                    Ok(res) => act.id = res, //-- updating the id of the UserChatSession actor back from the result of Connect handler 
                    _ => ctx.stop(), //-- in case of any errors we must stop the actor using its context
                }
                fut::ready(())
            })
            .wait(ctx); //-- spawns the future into the given context, waiting for it to resolve and stops processing any incoming events until this future resolves
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(balancer::Disconnect { username: self.name.clone(), id: self.id });
        Running::Stop
    }
}

// ----------------------------------------------------------------
// implementing Handler for Message event for UserChatSession actor
// ----------------------------------------------------------------
impl Handler<balancer::Message> for UserChatSession {
    type Result = ();

    fn handle(&mut self, msg: balancer::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0); //-- msg is a tuple structure and to get the first member we have to use msg.0 - handle messages from ChatServer, we simply send it to peer websocket
    }
}

// ---------------------------------------------------------------------------
// implementing StreamHandler for websocket messages for UserChatSession actor
// ---------------------------------------------------------------------------

//-- helper trait that allows to handle Stream in a similar way as normal actor messages
//-- when stream resolves to a next item, handle() method of this trait get called
/* ----------------------------------------------------------------------------------------------------------
    in StreamHandler trait implemented for the actor, msg is used to send and receive 
    websocket frame (header + application data) by encoding and decoding the outgoing and incoming 
    stream of binary data of type web::Payload inside the user_chat_sess_index route using ws::Message codec
   ---------------------------------------------------------------------------------------------------------- */
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context){
        let msg = match msg{
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop(); //-- stop the stream handler of this actor using the context
                return;
            }
        };
        match msg{
            ws::Message::Ping(msg) => {
                self.hb = Instant::now(); //-- beating the heart of the client
                ctx.pong(&msg); //-- getting the pong message from the client
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now(); //-- beating the heart of the client again on pong message
            }
            ws::Message::Text(text) => { //-- we're using the Text decoder to decode the incoming message stream into the String
                let m = text.trim();
                let msg = format!("[{}]: {}", self.name.clone().unwrap(), m);
                self.addr.send(balancer::ClientMessage{ //-- sending asynchronous ClientMessage event to ChatServer actor and waits for a response
                    id: self.id,
                    msg,
                    room: self.room.clone(),
                    friend_id: self.friend_id,
                    user_id: self.user_id,
                }).into_actor(self) //-- converting normal future to a UserChatSession actor
                    .then(|res, act, ctx| { //-- getting the result from ClientMessage handler, UserChatSession actor and context inside .then() callback through a closure
                        match res{
                            Ok(res) => act.inserted_message_uuid = Some(res.0), //-- updating the inserted_message_uuid back from the ClientMessage handler
                            _ => ctx.stop(), //-- in case of any errors we must stop the actor using its context
                        }
                        fut::ready(())
                    })
                    .wait(ctx); //-- spawns the future into the given context, waiting for it to resolve and stops processing any incoming events until this future resolves
            }
            ws::Message::Binary(_) => println!("[!] SERVER TIME : {} | Unexpected binary message", chrono::Local::now().naive_local()),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
            _ => (),
        }
    }
}









// ===================================================
// configuring the websocket index for our http server
// ===================================================

#[get("/chat/{username}/{friend_id}/{token}")] //-- route of private messaging between 2 peers
async fn user_chat_sess_index(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<balancer::ChatServer>>, 
                              web::Path((username, friend_id, token)): web::Path<(String, i32, String)>) -> Result<HttpResponse, Error> {
    
    let ip = req.peer_addr().unwrap().ip();
    let port = req.peer_addr().unwrap().port();
    let mut actor: Option<UserChatSession> = None;
    match helerium::authenticity!(token){ //-- check authenticity of the token to get the user_id
        Ok(user_id) => {
            let salt = env::var("SECRET_KEY").expect("⚠️ please set secret key in .env");
            let secret_to_hash = format!("{}&{}", &user_id.to_string(), &friend_id.to_string());
            let argon2_config = Config{
                variant: Variant::Argon2i,
                version: Version::Version13,
                mem_cost: 65536, // Kb
                time_cost: 10,
                lanes: 4,
                thread_mode: ThreadMode::Parallel,
                secret: &[],
                ad: &[],
                hash_length: 32
            };
            let room_name = argon2::hash_encoded(secret_to_hash.as_bytes(), salt.as_bytes(), &argon2_config).unwrap();
            actor = Some(UserChatSession{ //-- do websocket handshake and start UserChatSession actor
                    id: 0, //-- socket, client session or actor id
                    hb: Instant::now(), //-- heartbeat starting time
                    ip, //-- the ip address of the client session
                    port, //-- the port of the client session
                    room: room_name, //-- the hash of the user_id and friend_id
                    name: Some(username.clone()), //-- peer username
                    addr: srv.get_ref().clone(), //-- the address of ChatServer actor - can't implement the trait Clone for UserChatSession because of this field
                    friend_id, //-- friend_id of the user that she/he wants to chat with
                    user_id, //-- the id of the client session took from the verifying the token
                    inserted_message_uuid: None, //-- uuid of the inserted message into cassandra
                }); //-- actor.as_ref() converts from &Option<T> to Option<&T>
            },
            Err(e) => {
                println!("[!] SERVER TIME : {} | FAILED TO VERIFY THE TOKEN CAUSE : {} ", chrono::Local::now().naive_local(), e); 
            }
        }
    let res = if let Some(ucs_actor) = actor{
        println!("[+] SERVER TIME : {} | NUSHIN REQUEST FROM PEER ::: {}:{} ", chrono::Local::now().naive_local(), ucs_actor.ip, ucs_actor.port);
        ws::start(ucs_actor, &req, stream) //-- starting the UserChatSession actor for streaming of binary data
    } else{
        Ok(HttpResponse::Ok().json(format!("Server couldn't start the `UserChatSession` actor at time : {}", chrono::Local::now().naive_local())))
    };
    println!("{:?}", res);
    res

}

pub fn user_chat_sess_init(config: &mut web::ServiceConfig){
    config.service(user_chat_sess_index);
}









