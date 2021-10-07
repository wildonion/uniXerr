




use crate::handlers::db::cass::establish::CassSession;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::query_values;
use cdrs_helpers_derive::*;








/////////////////////////////////////////////////// CASSANDRA ARCHITECTURE ///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////// https://github.com/AlexPikalov/cdrs/blob/master/type-mapping.md /////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
#[derive(Serialize, Deserialize, Clone, IntoCDRSValue, PartialEq, Debug, TryFromRow)] //-- Serialize and Deserialize is required for sedning json in response
pub struct Chat{
    pub id: Uuid,
    pub user_id: i32,
    pub friend_id: i32,
    pub message: String,
    pub room_name: String,
    pub chattime: i64, //-- the time that Chat object is created
}






impl Chat{
    
    pub fn all(session: Arc<CassSession>, user_id: i32, friend_id: i32, room_name: String) -> Vec<Self>{
        // NOTE - order by is only supported on the clustered columns of the PRIMARY KEY and 
        //        when the partition key is restricted by an Equality or an IN operator in where clause.
        //        so in our select query we have to order by chattime and servertime respectively 
        //        where all of our primary keys are equal to the given value took from the function parameters.
        let values = query_values!("user_id" => user_id, "friend_id" => friend_id, "room_name" => room_name);
        let select_player_chat_data_cql = "SELECT * FROM uniXerr.user_chat WHERE user_id = ? AND friend_id = ? AND room_name = ? ORDER BY chattime, servertime LIMIT 20";
        let rows = session.query_with_values(select_player_chat_data_cql, values)
                                      .expect("⚠️ user_chat column family selecting rows error")
                                      .get_body()
                                      .expect("⚠️ user_chat column family getting body error")
                                      .into_rows()
                                      .expect("⚠️ user_chat column family into rows error");

        rows.into_iter().map(|cass_row| {
            Chat::try_from_row(cass_row).expect("⚠️ user chats into Chat struct error")
        }).collect::<Vec<Chat>>()
    }

    fn insert(&self) -> QueryValues{
        query_values!("id" => self.id, "user_id" => self.user_id, "friend_id" => self.friend_id, 
                      "room_name" => self.room_name.clone(), "message" => self.message.clone(), 
                      "chattime" => self.chattime)
    }

    pub fn init(session: Arc<CassSession>){
        let create_player_data_table = "CREATE TABLE IF NOT EXISTS uniXerr.user_chat (id UUID, user_id int, friend_id int, room_name text, message text, 
                                             chattime timestamp, servertime timestamp, PRIMARY KEY((id, user_id, friend_id, room_name), chattime, servertime))
                                             WITH CLUSTERING ORDER BY ((chattime, servertime) ASC);";
        session.query(create_player_data_table).expect("⚠️ user_chat table creation error");
    }

    pub fn save(&self, session: Arc<CassSession>) -> Uuid{
        let insert_player_chat_data_cql = "INSERT INTO uniXerr.user_chat (id, user_id, friend_id, room_name, message, chattime, servertime) 
                                                VALUES (?, ?, ?, ?, ?, ?, ?, toUnixTimestamp(now()))";
        let values = self.insert();
        let frame = session.query_with_values(insert_player_chat_data_cql, values).expect("⚠️ user_chat column family insertion error");
        frame.tracing_id.unwrap() //-- returning the inserted uuid back to where it's has been called
    }
}
