


use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use crate::utils::ResponseBody;

///////////// ==========================================================================================================================

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

///////////// ==========================================================================================================================

#[derive(Debug, Deserialize)] //-- deserialize the structure
pub struct uniXerr{
    pub error_status_code: u16,
    pub error_message: String,
}


impl uniXerr{
    pub fn new(error_status_code: u16, error_message: String) -> uniXerr{ //-- constructor
        uniXerr{
            error_status_code,
            error_message,
        }
    }
}


impl fmt::Display for uniXerr{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for uniXerr{
    fn from(error: DieselError) -> uniXerr{
        match error{
            DieselError::DatabaseError(_, err) => uniXerr::new(409, err.message().to_string()), //-- this error will occure because of bad insert or update operation
            DieselError::NotFound => {
                uniXerr::new(404, "⚠️ not found".to_string())
            }
            err => uniXerr::new(500, format!("⚠️ unknown diesel error: {}", err)),
        }
    }
}

impl ResponseError for uniXerr{
    fn error_response(&self) -> HttpResponse{ //-- return a http response object
        // let status_code = StatusCode::from_u16(self.error_status_code).unwrap();
        let status_code = match StatusCode::from_u16(self.error_status_code){
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500{
            true => self.error_message.clone(),
            false => "⚠️ internal server error".to_string(),
        };
        HttpResponse::build(status_code).json(json!({"message": error_message}))
    }
}