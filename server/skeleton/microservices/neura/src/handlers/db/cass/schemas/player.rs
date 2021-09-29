




/*  

TODO - 

    https://github.com/krojew/cdrs-tokio/blob/master/documentation
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/crud_operations.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/multiple_thread.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/insert_collection.rs
    https://github.com/krojew/cdrs-tokio/blob/master/cdrs-tokio/examples/prepare_batch_execute.rs
    https://www.oreilly.com/library/view/cassandra-the-definitive/9781491933657/ch04.html

*/




use crate::handlers::db::cass::establish::CassSession;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::frame::AsBytes;
use cdrs_tokio::types::from_cdrs::FromCdrsByName;
use cdrs_tokio::types::prelude::*;
use cdrs_tokio_helpers_derive::*;








/////////////////////////////////////////////////// CASSANDRA ARCHITECTURE ///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////// https://github.com/krojew/cdrs-tokio/blob/master/type-mapping.md ////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
#[derive(Serialize, Deserialize, Clone, Debug, TryFromRow, TryFromUdt)]
pub struct Chat{
    pub id: Uuid,
    pub user_id: i32,
    pub friend_id: i32,
    pub message: String,
    pub chattime: i64,
}






impl Chat{
    pub async fn first(session: Arc<CassSession>) -> Self{
        Chat{id: Uuid::new_v4(), user_id: 1, friend_id: 2, message: "hello my friend".to_string(), chattime: 0}
    }
    
    pub async fn all(session: Arc<CassSession>) -> Self{
        Chat{id: Uuid::new_v4(), user_id: 1, friend_id: 2, message: "hello my friend".to_string(), chattime: 0}
    }
    
    pub async fn first_by_id(session: Arc<CassSession>) -> Self{
        Chat{id: Uuid::new_v4(), user_id: 1, friend_id: 2, message: "hello my friend".to_string(), chattime: 0}
    }

    pub async fn last(session: Arc<CassSession>) -> Self{
        Chat{id: Uuid::new_v4(), user_id: 1, friend_id: 2, message: "hello my friend".to_string(), chattime: 0}
    }


    fn insert(&self) -> QueryValues{
        query_values!("id" => self.id, "user_id" => self.user_id, "friend_id" => self.friend_id, "message" => self.message.clone(), "chattime" => self.chattime)
    }



    pub async fn init(session: Arc<CassSession>){
        let create_player_data_table = "CREATE TABLE IF NOT EXISTS neura.user_chat (id UUID, user_id int, friend_id int, message text, chattime timestamp, servertime timestamp, PRIMARY KEY((id, user_id, friend_id), chattime, servertime));";
        session.query(create_player_data_table).await.expect("⚠️ user_chat table creation error");
    }



    pub async fn save(&self, session: Arc<CassSession>){
        let insert_player_data_cql = "INSERT INTO neura.user_chat (id, user_id, friend_id, message, chattime, servertime) VALUES (?, ?, ?, ?, ?, ?, toUnixTimestamp(now()))";
        let values = self.insert();
        session.query_with_values(insert_player_data_cql, values).await.expect("⚠️ user_chat column family insertion error");
    }
}
