




use actix_web::{HttpRequest, HttpResponse, Result, post, web};
use crate::middlewares::auth::pass;
use crate::utils::ResponseBody;
use crate::constants;
use crate::handlers::error::uniXerr;
use serde_json::json;
use super::model::{InsertableUserFriend, QueryableUserFriend};






// NOTE - a user sends a request to a friend and if his/her friend accepts the request 
//        means hits the follow button, then the status of the request will update to 1
//        and if the user unfollows his or her friend means hits the unfollow button 
//        to reject the request, the status will update to 0 





#[post("/auth/user/{user_id}/friend/{friend_id}/follow")] //-- required fields : from_user_id + to_friend_id
async fn follow(req: HttpRequest, user_id: web::Path<i32>, friend_id: web::Path<i32>,) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can follow a friend
                let updated_user_friend_status_row = QueryableUserFriend::accept_request(user_id.into_inner(), friend_id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(updated_user_friend_status_row)
                )
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




#[post("/auth/user/{user_id}/friend/{friend_id}/unfollow")] //-- required fields : from_user_id + to_friend_id
async fn unfollow(req: HttpRequest, user_id: web::Path<i32>, friend_id: web::Path<i32>,) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can unfollow a friend
                let updated_user_friend_status_row = QueryableUserFriend::reject_request(user_id.into_inner(), friend_id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(updated_user_friend_status_row)
                )
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




#[post("/auth/user/send-request")] //-- required fields : from_user_id + to_friend_id
async fn send_request_to_friend(req: HttpRequest, user_friend: web::Json<InsertableUserFriend>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can send a request to a friend
                let inserted_user_friend = QueryableUserFriend::send_request(user_friend.into_inner()).await?;
                Ok(HttpResponse::Ok().json(inserted_user_friend))
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




#[post("/auth/user-friend/delete/{id}")]
async fn delete_user_friend(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can delete a user_friend row
                let deleted_user_friend = QueryableUserFriend::delete(id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user_friend})))
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




#[post("/auth/user/get/{id}/friends")]
async fn get_all_friends(req: HttpRequest, user_id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can get all friends of a user
                let friends_for_a_user = QueryableUserFriend::find_all_user_friends(user_id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(friends_for_a_user)
                )
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




pub fn user_friend_init_service(config: &mut web::ServiceConfig){
    config.service(follow);
    config.service(unfollow);
    config.service(get_all_friends);
    config.service(send_request_to_friend);
    config.service(delete_user_friend);
}