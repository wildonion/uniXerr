






use diesel::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use crate::handlers::error::uniXerr;
use crate::schema::login_history;
use crate::entities::users::model::QueryableUser;
use serde::{Deserialize, Serialize}; // NOTE - Deserialize from json to struct to insert into db, Serialize from struct to json to send the response 
use crate::handlers::db::pg::establish as pg;
use chrono::NaiveDateTime;




///////////// =========================================================================================================================
#[derive(Insertable)]
#[table_name = "login_history"]
pub struct InsertableLoginHistory{
    pub user_id: i32,
    pub last_login: NaiveDateTime,
}

impl InsertableLoginHistory{
    fn from(login_history: InsertableLoginHistory) -> InsertableLoginHistory{
        InsertableLoginHistory{
            user_id: login_history.user_id,
            last_login: login_history.last_login,
        }
    }
}
///////////// ==========================================================================================================================
#[derive(Serialize, Identifiable, AsChangeset, Queryable)]
#[table_name="login_history"]
pub struct QueryableLoginHistory{
    pub id: i32,
    pub user_id: i32,
    pub last_login: NaiveDateTime,
}

impl QueryableLoginHistory{

    pub async fn create(un: &str) -> Option<InsertableLoginHistory>{
        if let Ok(user) = QueryableUser::find_user_by_username(un).await{
            let now = chrono::Local::now().naive_local();
            let login_history = InsertableLoginHistory{user_id: user.id, last_login: now,};
            Some(login_history)
        } else{
            None
        }
    }
    
    pub async fn add(login_history_record: InsertableLoginHistory) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let history = InsertableLoginHistory::from(login_history_record);
        Ok(diesel::insert_into(login_history::table).values(history).get_result(&conn)?)
    }
    
    pub async fn find_all() -> Result<Vec<Self>, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let histories = login_history::table.load::<QueryableLoginHistory>(&conn)?;
        Ok(histories)
    }

    pub async fn find(id: i32) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let history = login_history::table.filter(login_history::id.eq(id)).first::<QueryableLoginHistory>(&conn)?;
        Ok(history)
    }

    pub async fn delete(id: i32) -> Result<usize, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let response = diesel::delete(login_history::table.filter(login_history::id.eq(id))).execute(&conn)?; //-- usize is the size of the allocated bytes in memory like an i32 which is 4 bytes
        Ok(response)
    }
}
///////////// =========================================================================================================================

