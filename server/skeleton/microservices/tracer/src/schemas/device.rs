


use serde::{Serialize, Deserialize};
use uuid::Uuid;




#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GPS{
    pub id: Uuid,
    pub time: i64,
}






impl Default for GPS{
    fn default() -> Self{
        GPS{
            id: Uuid::new_v4(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}
