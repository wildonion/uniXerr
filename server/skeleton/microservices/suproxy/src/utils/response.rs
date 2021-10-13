





use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};




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


pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}