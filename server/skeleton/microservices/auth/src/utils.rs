





use serde::{Deserialize, Serialize};




pub mod jwt;
pub mod another_utils_mod{}






#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T>{
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T>{
    pub fn new(message: &str, data: T) -> ResponseBody<T>{
        ResponseBody{
            message: message.to_string(),
            data,
        }
    }
}