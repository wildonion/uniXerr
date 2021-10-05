








use serde::{Serialize, Deserialize};
use uuid::Uuid;






#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData{
    pub id: Uuid,
    pub time: i64,
}






impl Default for MetaData{
    fn default() -> Self{
        MetaData{
            id: Uuid::new_v4(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}
