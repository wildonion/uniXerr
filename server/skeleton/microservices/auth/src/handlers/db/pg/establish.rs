





use crate::handlers::error::uniXerr;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::env;
use dotenv::dotenv;
use r2d2;



type Pool = r2d2::Pool<ConnectionManager<PgConnection>>; //-- diesel r2d2 is safe to move and share between threads cause it's bounded to Send and Safe traits
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type r2d2Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;





async fn pool() -> Pool{
    dotenv().expect("⚠️ .env file not found");
    let db_url = env::var("DATABASE_URL").expect("⚠️ couldn't find the database url");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("⚠️ failed to create pg db pool") //-- it's a pool object of connections
}






//-- due to no move and no change of the connection between actix threads we didn't put the DbConnection inside Arc<Mutex<DbConnection>>. 
pub async fn connection() -> Result<DbConnection, uniXerr>{
    pool().await.get().map_err(|e| uniXerr::new(500, format!("⚠️ failed getting db connection: {}", e))) //-- this will return a result
}




pub async fn connections() -> Result<r2d2Pool, Box<dyn std::error::Error>>{
    dotenv().expect("⚠️ .env file not found");
    let pghost = env::var("PGHOST").expect("⚠️ couldn't find the postgres host");
    let pgport = env::var("PGPORT").expect("⚠️ couldn't find the postgres port");
    let pguser = env::var("POSTGRES_USER").expect("⚠️ couldn't find the postgres user");
    let pgpassowrd = env::var("POSTGRES_PASSWORD").expect("⚠️ couldn't find the postgres user");
    let db = env::var("POSTGRES_DB").expect("⚠️ couldn't find the postgres db");
    let db_url = format!("postgres://{}:{}@{}:{}/{}", pguser, pgpassowrd, pghost, pgport, db).parse().expect("⚠️ pg db url config error"); //-- unwrap() will make the compiler panic on any error
    let manager = PostgresConnectionManager::new(db_url, NoTls);
    match r2d2::Pool::new(manager){
        Ok(pool) => {
            Ok(pool) //-- it's a pool object of connections
        },
        Err(e) => {
            eprintln!("⚠️ failed to create pg pool");
            Err(From::from(e))
        }
    } 
}


