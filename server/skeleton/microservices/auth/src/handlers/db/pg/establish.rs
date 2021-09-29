





use crate::handlers::error::SKELETON;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::env;
use dotenv::dotenv;
use r2d2;



type Pool = r2d2::Pool<ConnectionManager<PgConnection>>; //-- diesel r2d2 is safe to move and share between threads cause it's bounded to Send and Safe traits
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;





async fn pool() -> Pool{
    dotenv().expect("⚠️ .env file not found");
    let db_url = env::var("DATABASE_URL").expect("⚠️ couldn't find the database url");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("⚠️ failed to create pg db pool") //-- it's a pool object of connections
}






//-- due to no move and no change of the connection between actix threads we didn't put the DbConnection inside Arc<Mutex<DbConnection>>. 
pub async fn connection() -> Result<DbConnection, SKELETON>{
    pool().await.get().map_err(|e| SKELETON::new(500, format!("⚠️ failed getting db connection: {}", e))) //-- this will return a result
}


