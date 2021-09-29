



// Authentication APIs


use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    constants,
    utils::jwt::{token_utils, user_token::UserToken},
    handlers::error::ServiceError,
    entities::users::model::{QueryableUser, Login, InsertableUser},
};
use actix_web::
    http::{
        StatusCode,
        header::HeaderValue,
    };



#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse{
    pub token: String,
    pub token_type: String,
    pub user_id: i32,
    pub username: String,
}



pub async fn signup(user: InsertableUser) -> Result<String, ServiceError>{ // it returns ServiceError object 
    match QueryableUser::signup(user).await{
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message))
    }
}

pub async fn login(login: Login) -> Result<TokenBodyResponse, ServiceError>{
    match QueryableUser::login(login).await{
        Some(logged_user) => {
            match serde_json::from_value(
                json!({ // TokenBodyResponse json
                    "token": UserToken::generate_token(&logged_user), // creats the jwt based on the username and access_token 
                    "token_type": "bearer",
                    "username": logged_user.username,
                    "user_id": logged_user.id,
                })
            ){
                Ok(token_res) => {
                    if logged_user.access_token.is_empty(){ // means the user might be logged out or the bug of some empty access_token inside the code or db :/
                        Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAILED.to_string()))
                    } else{
                        Ok(token_res) // returns the generated token after successful login
                    }
                }, 
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        None => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_USER_NOT_FOUND.to_string()))
    }
}


pub async fn logout(auth_header: HeaderValue) -> Result<(), ServiceError>{
    if let Ok(authen_str) = auth_header.to_str(){
        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer"){
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()){
                if let Ok(user_token) = token_utils::verify_token(&token_data).await{
                    if let Ok(user) = QueryableUser::find_user_by_username(&user_token.user).await{
                        QueryableUser::logout(user.id).await;
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()))
}


pub async fn check_token(auth_header: HeaderValue) -> Result<i32, ServiceError>{
    if let Ok(authen_str) = auth_header.to_str(){
        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer"){
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()){
                if let Ok(user_token) = token_utils::verify_token(&token_data).await{
                    return Ok(user_token.id);
                } 
                return Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_INVALID_TOKEN.to_string()));
            }
        }
    }
    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()))
}