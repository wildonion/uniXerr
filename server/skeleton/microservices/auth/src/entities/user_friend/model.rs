




use diesel::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use crate::handlers::error::uniXerr;
use crate::schema::user_friend;
use serde::{Deserialize, Serialize}; // NOTE - Deserialize from json to struct to insert into db, Serialize from struct to json to send the response 
use crate::handlers::db::pg::establish as pg;
use chrono::NaiveDateTime;





///////////// =========================================================================================================================
#[derive(Deserialize, Insertable)]
#[table_name = "user_friend"]
pub struct InsertableUserFriend{
    pub from_user_id: i32,
    pub to_friend_id: i32,
    pub requested: Option<NaiveDateTime>, //-- getting this field as Option means that it'll fill later automatically when building the insertable query
}
///////////// ==========================================================================================================================
#[derive(Serialize, AsChangeset, Queryable)]
#[table_name="user_friend"]
pub struct QueryableUserFriend{
    pub id: i32,
    pub from_user_id: i32,
    pub to_friend_id: i32,
    pub status: i16,
    pub requested: NaiveDateTime,
}

impl QueryableUserFriend{

    pub async fn accept_request(user_id: i32, friend_id: i32) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let updated_status_user_friend = diesel::update(user_friend::table
                                                                                                .filter(user_friend::from_user_id.eq(user_id)))
                                                                                                .filter(user_friend::to_friend_id.eq(friend_id))
                                                                                                .set(user_friend::status.eq(1))
                                                                                                .get_result(&conn)?;
        Ok(updated_status_user_friend)
    }

    pub async fn reject_request(user_id: i32, friend_id: i32) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let updated_status_user_friend = diesel::update(user_friend::table
                                                                                                .filter(user_friend::from_user_id.eq(user_id)))
                                                                                                .filter(user_friend::to_friend_id.eq(friend_id))
                                                                                                .set(user_friend::status.eq(0))
                                                                                                .get_result(&conn)?;
        Ok(updated_status_user_friend)

    }
    
    pub async fn send_request(user_friend: InsertableUserFriend) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let new_request = InsertableUserFriend{
            from_user_id: user_friend.from_user_id,
            to_friend_id: user_friend.to_friend_id,
            requested: Some(chrono::Local::now().naive_local()),
        };
        let inserted_new_user_friend = diesel::insert_into(user_friend::table).values(new_request).get_result(&conn)?;
        Ok(inserted_new_user_friend)
    }

    pub async fn find_all_user_friends(user_id: i32) -> Result<Vec<Self>, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let friends_for_a_user = user_friend::table.filter(user_friend::from_user_id.eq(user_id)).get_results::<QueryableUserFriend>(&conn)?;
        Ok(friends_for_a_user)
    }

    pub async fn delete(id: i32) -> Result<usize, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let response = diesel::delete(user_friend::table.filter(user_friend::id.eq(id))).execute(&conn)?; //-- usize is the size of the allocated bytes in memory like an i32 which is 4 bytes
        Ok(response)
    }

    
}
///////////// =========================================================================================================================

