




use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Serialize, Deserialize};
use uuid::Uuid;




#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Address(pub String);