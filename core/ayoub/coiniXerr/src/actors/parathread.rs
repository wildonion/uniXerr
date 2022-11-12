







use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case
use super::peer; //-- super is the root of the current directory which is actors directory contains parathread.rs and peer.rs crates














// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                  messages events
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct Communicate{ //-- parathread sends this message to a parachain
    pub id: Uuid,
    pub cmd: Cmd,
}

#[derive(Clone, Debug)]
pub struct UpdateParachainEvent{
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub current_block: Option<Block>,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Default)]
pub enum Cmd{
    #[default] //// enum data types can only have one field as the default value
    GetCurrentBlock, //// Mine field is the default value; borsh utf8 encoded variant is 0
    GetSlot, //// borsh utf8 encoded variant is 1
    GetBlockchain, //// borsh utf8 encoded variant is 2
    GetNextParachain, //// borsh utf8 encoded variant is 3
    GetGenesis, //// borsh utf8 encoded variant is 4
    GetParachainUuid, //// borsh utf8 encoded variant is 5
    WaveResetSlotFrom(String), //// borsh utf8 encoded variant is 6 - Uuid is the id of the parachain that waved a hi
    WaveSlotToNextParachainActor, //// borsh utf8 encoded variant is 7
    WaveSlotToParachainActor(String), //// borsh utf8 encoded variant is 8 - String is the path of the selected parachain actor
    WaveResetSlotFromSystem, //// borsh utf8 encoded variant is 9
}

#[derive(Clone, Debug)]
pub struct ParachainCreated(pub Uuid); //// a message event to broadcast it by the channel to all parachain subscriber actors about creating a new parachain - first element of this struct is the parachain uuid

#[derive(Clone, Debug)]
pub struct ParachainUpdated(pub Uuid); //// a message event to broadcast it by the channel to all parachain subscriber actors about updating a parachain - first element of this struct is the parachain uuid













// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                 Parachain type actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[actor(Communicate, UpdateParachainEvent, ParachainCreated, ParachainUpdated)] //-- Parachain actor will receive a message either from other actor or a channel to subscribe to of type Communicate, UpdateParachainEvent, ParachainCreated and ParachainUpdated
#[derive(Debug, Clone, Default)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Parachain {
    pub id: Uuid,
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub next_parachain: Option<ActorRef<<Parachain as Actor>::Msg>>, //-- next parachain actor which is of type Parachain
    pub current_block: Option<Block>,
}

impl Parachain{ //// Parachain is the parallel chain of the coiniXerr network which is a shard actor
    
    pub fn heart_beat(){

        // TODO - check the parachain health using scheduling process
        // ...
    
    }

    pub fn get_uuid(&self) -> Option<Uuid>{
        Some(self.id.clone())
    }

    pub fn get_current_block(&self) -> Option<Block>{
        self.current_block.clone()
    }

    pub fn get_genesis(&self) -> Option<Block>{ //// the lifetime of the &Block is the lifetime of the &self
        let genesis_block = self.blockchain.as_ref().unwrap().get_genesis();
        Some(genesis_block) //// returning the genesis_block as an Option 
    }

    pub fn get_next_parachain(&self) -> Option<ActorRef<<Parachain as Actor>::Msg>>{
        self.next_parachain.clone()
    }

    pub fn get_slot(&self) -> Option<Slot>{
        self.slot.clone()
    }

    pub fn get_blockchain(&self) -> Option<Chain>{
        self.blockchain.clone()
    }

    pub fn set_slot(&mut self, slot: Slot) -> Self{ //// Self referes to the Parachain struct
        self.slot = Some(slot);
        Self{ //// Self referes to the Parachain struct 
            id: self.id, 
            slot: self.slot.clone(), 
            blockchain: self.blockchain.clone(), 
            next_parachain: self.next_parachain.clone(), 
            current_block: self.current_block.clone() 
        }
    }

    pub fn set_blockchain(&mut self, blockchain: Chain) -> Self{ //// Self referes to the Parachain struct
        self.blockchain = Some(blockchain);
        Self{ //// Self referes to the Parachain struct 
            id: self.id, 
            slot: self.slot.clone(), 
            blockchain: self.blockchain.clone(), 
            next_parachain: self.next_parachain.clone(), 
            current_block: self.current_block.clone() 
        }
    }

    pub fn set_current_block(&mut self, block: Block) -> Self{ //// Self referes to the Parachain struct
        self.current_block = Some(block);
        Self{ //// Self referes to the Parachain struct 
            id: self.id, 
            slot: self.slot.clone(), 
            blockchain: self.blockchain.clone(), 
            next_parachain: self.next_parachain.clone(), 
            current_block: self.current_block.clone() 
        }
    }

}












// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Actor for the Parachain type
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Actor for Parachain{

    //// Parachain actor must support the message type of the channels related to the parachain message events (ParachainCreated, ParachainUpdated) that they want to subscribe to
    //// When using the #[actor()] attribute, the actor's Msg generic associated type (GAT) should be set to '[DataType]Msg'. 
    //// E.g. if an actor is a struct named MyActor, then the Actor::Msg generic associated type (GAT) will be MyActorMsg.
    type Msg = ParachainMsg; //// we can access all the message event actors which has defined for Parachain using ParachainMsg::   //// Msg generic associated type (GAT) is the actor mailbox type and is of type ParachainMsg which is the Parachain type itself; actors can communicate with each other by sending message to each other

    fn recv(&mut self, 
            ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
            msg: Self::Msg, 
            sender: Sender){
        
        self.receive(ctx, msg, sender);

    }
}


impl ActorFactoryArgs<(Uuid, Option<Slot>, Option<Chain>, Option<ActorRef<<Parachain as Actor>::Msg>>, Option<Block>)> for Parachain{

    fn create_args((id, slot, blockchain, next_parachain, current_block): (Uuid, Option<Slot>, Option<Chain>, Option<ActorRef<<Parachain as Actor>::Msg>>, Option<Block>)) -> Self{ //// Self referes to the Parachain struct
        
        Self { id, slot, blockchain, next_parachain, current_block } //// initiate an instance of the Parachain with the passed in args
    
    }

}











//// we must first define the event then impl its handler for our actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈  
//      message event receive handlers for the Parachain actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Receive<UpdateParachainEvent> for Parachain{ //// implementing the Receive trait for the Parachain actor to handle the incoming message of type UpdateParachainEvent
    type Msg = ParachainMsg; //// we can access all the message event actors which has defined for Parachain using ParachainMsg::Communicate, ParachainMsg::UpdateParachainEvent, ParachainMsg::ParachainCreated, ParachainMsg::ParachainUpdated  

    fn receive(&mut self, 
                _ctx: &Context<Self::Msg>, 
                _msg: UpdateParachainEvent, 
                _sender: Sender){ //// _sender is a BasicActorRef that can setup a message that must be sent to an actor using try_tell() method
        info!("➔ 🔃 update parachain message info received");
    
        //// updating the state of the parachain with passed in message
        let updated_parachain = Self{ //// Self referes to the Parachain struct
            id: self.id,
            slot: match _msg.slot{
                Some(slot) => Some(slot),
                None => self.slot.clone(), //// keeping the old slot
            },
            blockchain: match _msg.blockchain{
                Some(blockchain) => Some(blockchain),
                None => self.blockchain.clone(), //// keeping the old blockchain
            },
            current_block: match _msg.current_block{
                Some(current_block) => Some(current_block),
                None => self.current_block.clone(), //// keeping the old current_block
            },
            next_parachain: self.next_parachain.clone(), //// keeping the old next_parachain
        };

        _sender
            .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
            .unwrap()
            .try_tell(
                updated_parachain, //// sending the updated parachain as the response message 
                Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
            );
    }

}

impl Receive<Communicate> for Parachain{ //// implementing the Receive trait for the Parachain actor to handle the incoming message of type Communicate
    type Msg = ParachainMsg; //// we can access all the message event actors which has defined for Parachain using ParachainMsg::  

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: Communicate, //-- _msg is of type Communicate since we're implementing the Receive trait for the Communicate type
                _sender: Sender){ //// _sender is a BasicActorRef that can setup a message that must be sent to an actor using try_tell() method
    
        info!("➔ 📩 message info received with id [{}] and command [{:?}]", _msg.id, _msg.cmd);
        match _msg.cmd{
            Cmd::GetCurrentBlock => {
                info!("➔ 🔙 returning current block of the parachain with id [{}]", self.id);
                let current_block = self.get_current_block();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        current_block, //// sending the current_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            },
            Cmd::GetNextParachain => {
                info!("➔ 🔙 returning the next parachain of the parachain with id [{}]", self.id);
                let next_parachain = self.get_next_parachain();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        next_parachain, //// sending the next_parachain as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            },
            Cmd::GetBlockchain => {
                info!("➔ 🔙 returning the blockchain of the parachain with id [{}]", self.id);
                let blockchain = self.get_blockchain();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        blockchain, //// sending the blockchain as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            },
            Cmd::GetGenesis => {
                info!("➔ 🔙 returning the genesis block of the parachain with id [{}]", self.id);
                let genesis_block = self.get_genesis();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        genesis_block, //// sending the genesis_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            },
            Cmd::GetParachainUuid => {
                info!("➔ 🔙 returning the parachain uuid");
                let genesis_block = self.get_uuid();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        genesis_block, //// sending the genesis_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            },
            Cmd::WaveSlotToNextParachainActor => {
                info!("➔ 👋🏼 waving from parachain with id [{}] to its next parachain", self.id);
                
                let next_parachain = self.get_next_parachain().unwrap(); //// getting the next parachain field
                let actor_system = &_ctx.system; //// getting the borrowed form of the actor system from the _ctx

                // TODO - update the default parachain slot using a successful auction process by the coiniXerr validators like when we reach 600k blocks inside a slot then we have to reset the slot 
                // ....
                
                //// resetting the slot field of the next parachain but untouched other fields using ask() function 
                //// since the parachain is guared by ActorRef thus in order to access its field we have to ask the guardian :)
                //// passing other fields as the None won't update them to None they will be remained as their last value
                //// we can also put the instance of the UpdateParachainEvent inside the ParachainMsg::UpdateParachainEvent() tuple struct
                //// the receiver must be an actor of type ActorRef since Tell<Msg> the trait `riker::actor::Tell<T>` is implemented for `riker::actor::ActorRef<M>` 
                let update_next_parachain_remote_handle: RemoteHandle<Parachain> = ask(actor_system, &next_parachain, ParachainMsg::UpdateParachainEvent(UpdateParachainEvent{slot: Some(Slot::default()), current_block: None, blockchain: None})); //// asking the coiniXerr system to update the state of the passed in parachain actor as a future object
                let update_next_parachain_future = update_next_parachain_remote_handle;
                let update_next_parachain = block_on(update_next_parachain_future); //// we can't use .await here since we're not inside an async function

                //// sending the updated next parachain (slot field) to the caller or the previous actor 
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        update_next_parachain, //// sending the update_next_parachain as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );

            },
            Cmd::WaveSlotToParachainActor(parachain_path) => {
                info!("➔ 👋🏼 waving from parachain with id [{}] to parachain [{}]", self.id, parachain_path);

                let path = parachain_path.as_str();
                let selected_parachain = _ctx.select(path).unwrap(); //// selecting the passed in parachain to wave reset slot from this parachain to it - calling between actors by selecting the desired actor using select() method
                let waver_id = self.id.to_string(); //// getting the uuid string of this parachain
                
                //// waving a reset slot message from this parachain to the selected_parachain
                selected_parachain.try_tell( //// try to tell the selected_parachain
                                    ParachainMsg::Communicate(Communicate{id: Uuid::new_v4(), cmd: Cmd::WaveResetSlotFrom(waver_id)}), //// that you will have a wave reset slot message from this parachain 
                                    None, //// there is no need to pass a sender since we're communicating with selected_parachain itself and not returning a response (not this parachain or the caller of this Cmd arm) to tell a message that we've just gotten from this parachain 
                                );
            
            },
            Cmd::WaveResetSlotFrom(waver_id) => {

                //// logging the incoming wave reset slot from the waver parachain to this parachain
                info!("➔ ⭕ got a reset wave sent from parachain with id [{}] to this parachain with id [{}]", waver_id, self.id);

                // TODO - update the default parachain slot using a successful auction process by the coiniXerr validators like when we reach 600k blocks inside a slot then we have to reset the slot
                // ....
                
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell( //// try to tell this parachain
                        ParachainMsg::UpdateParachainEvent(UpdateParachainEvent{slot: Some(Slot::default()), current_block: None, blockchain: None}), //// that we want to update the slot field
                        None //// there is no need to pass a sender since we're communicating with this parachain itself and not returning a response (not the caller of this Cmd arm) to tell a message that we've just gotten from this parachain
                    );
            },
            Cmd::WaveResetSlotFromSystem => {
                info!("➔ ⭕ got a reset wave sent from system to this parachain with [{}]", self.id);

                // TODO - update the default parachain slot using a successful auction process by the coiniXerr validators like when we reach 600k blocks inside a slot then we have to reset the slot
                // ....

                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell( //// try to tell this parachain
                        ParachainMsg::UpdateParachainEvent(UpdateParachainEvent{slot: Some(Slot::default()), current_block: None, blockchain: None}), //// that we want to update the slot field
                        None //// there is no need to pass a sender since we're communicating with this parachain itself and not returning a response (not the caller of this Cmd arm) to tell a message that we've just gotten from this parachain
                    );
            },
            _ => { //// GetSlot
                info!("➔ 🔙 returning the slot of the parachain with id [{}]", self.id);
                let current_slot = self.get_slot();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        current_slot, //// sending the current_slot as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself - sender is the caller itself which the passed in message will be sent back to this actor
                    );
            }
        }            


    }

}


impl Receive<ParachainCreated> for Parachain{ //// implementing the Receive trait for the Parachain actor to handle the incoming message of type ParachainCreated
    type Msg = ParachainMsg; //// we can access all the message event actors which has defined for Parachain using ParachainMsg::  

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: ParachainCreated, //-- _msg is of type ParachainCreated since we're implementing the Receive trait for the ParachainCreated type
                _sender: Sender){ //// _sender is a BasicActorRef that can setup a message that must be sent to an actor using try_tell() method
    
        info!("➔ 🥳 new parachain created with id [{}]", _msg.0); //// ParachainCreated is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
}


impl Receive<ParachainUpdated> for Parachain{ //// implementing the Receive trait for the Parachain actor to handle the incoming message of type ParachainUpdated
    type Msg = ParachainMsg; //// we can access all the message event actors which has defined for Parachain using ParachainMsg::  

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: ParachainUpdated, //-- _msg is of type ParachainUpdated since we're implementing the Receive trait for the ParachainUpdated type
                _sender: Sender){ //// _sender is a BasicActorRef that can setup a message that must be sent to an actor using try_tell() method
    
        info!("➔ 🥳 parachain updated with id [{}]", _msg.0); //// ParachainUpdated is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
}