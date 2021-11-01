






use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use crate::handlers::error::uniXerr;
use crate::utils::ResponseBody;
use crate::middlewares::auth::pass;
use crate::constants;
use serde_json::json;
use super::model::{QueryableLoginHistory}; //-- load from the root of the current crate





#[get("/auth/login-histories")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can get all histories
                let histories = QueryableLoginHistory::find_all().await?;
                Ok(HttpResponse::Ok().json(histories))
            } else{
                Ok(HttpResponse::Forbidden()
                    .json(ResponseBody::new(
                        constants::MESSAGE_ACCESS_DENIED,
                        constants::EMPTY,
                    ))
                    .into_body(),
                )
            }
        },
        Err(_) => {
            Ok(
                HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN,
                        constants::EMPTY,
                    ))
                    .into_body(),
            )
        }
    }
}








#[get("/auth/login-history/{id}")]
async fn find(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can get a history
                let history = QueryableLoginHistory::find(id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(history))
            } else{
                Ok(HttpResponse::Forbidden()
                    .json(ResponseBody::new(
                        constants::MESSAGE_ACCESS_DENIED,
                        constants::EMPTY,
                    ))
                    .into_body(),
                )
            }
        },
        Err(_) => {
            Ok(
                HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN,
                        constants::EMPTY,
                    ))
                    .into_body(),
            )
        }
    }
}






#[post("/auth/login-history/delete/{id}")]
async fn delete(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can delete a history
                let deleted_login_history = QueryableLoginHistory::delete(id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(json!({"deleted": deleted_login_history})))
            } else{
                Ok(HttpResponse::Forbidden()
                    .json(ResponseBody::new(
                        constants::MESSAGE_ACCESS_DENIED,
                        constants::EMPTY,
                    ))
                    .into_body(),
                )
            }
        },
        Err(_) => {
            Ok(
                HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN,
                        constants::EMPTY,
                    ))
                    .into_body(),
            )
        }
    }
}






pub fn login_history_init_service(config: &mut web::ServiceConfig){
    config.service(find_all);
    config.service(find);
    config.service(delete);
}