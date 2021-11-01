


use serde_json::json;
use actix_web::{HttpRequest, HttpResponse, Result, post, web};
use crate::{
    utils::ResponseBody,
    constants,
    services::account as account_service,
    entities::users::model::{Login, InsertableUser},
};








#[post("/auth/register")] // required fields : username + email + password + phone_number + device_id 
async fn signup(user: web::Json<InsertableUser>) -> Result<HttpResponse> { // signup route doesn't need auth middleware
    match account_service::signup(user.0).await{ // user.0 is the actual data inside the json body which is the user data 
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}





#[post("/auth/login")] // required fields : username_or_eamil + password
async fn login(login: web::Json<Login>) -> Result<HttpResponse> { // login route doesn't need auth middleware
    match account_service::login(login.0).await{ // login.0 is the actual data inside the json body which is the login data 
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}



#[post("/auth/check-token")]
async fn check_token(req: HttpRequest) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match account_service::check_token(authen_header.clone()).await{
            Ok(user_id) => {
                let user_id_json = json!({ "user_id": user_id });
                Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_VALID_TOKEN, user_id_json)))
            },
            Err(err) => Ok(err.response()),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}



#[post("/auth/logout")]
async fn logout(req: HttpRequest) -> Result<HttpResponse> { // logout route doesn't need auth middleware cause it has the token inside its request header  
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header.clone()).await;
        Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}






pub fn auth_init_service(config: &mut web::ServiceConfig){
    config.service(signup);
    config.service(login);
    config.service(logout);
    config.service(check_token);
}