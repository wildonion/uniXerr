



use actix_web::{Error, HttpRequest, HttpResponse, Result, get, post, web};
use crate::middlewares::auth::pass;
use crate::utils::ResponseBody;
use crate::constants;
use crate::handlers::error::uniXerr;
use serde_json::json;
use super::model::{InsertableUser, QueryableUser, UpdatableUser, PasswordFields, UploadFile, UserData}; //-- load from the root of the current crate
use actix_multipart::Multipart;
use std::fs;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;







#[get("/auth/users")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can get all users
                let users = QueryableUser::find_all().await?;
                Ok(HttpResponse::Ok().json(users))
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




#[get("/auth/user/get/{id}")]
async fn find(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can get a user data
                let user = QueryableUser::find_by_id(id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(
                    UserData{
                        username: user.username,
                        email: user.email,
                        phone_number: user.phone_number,
                        wallet_address: user.wallet_address,
                        balance: user.coins,
                        sex: user.sex,
                        age: user.age,
                    }
                ))
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





#[post("/auth/user/add")] //-- required fields : username + email + password + phone_number + device_id 
async fn add(req: HttpRequest, user: web::Json<InsertableUser>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 2{ // NOTE - only admin can add a user
                let user = QueryableUser::add(user.into_inner()).await?;
                Ok(HttpResponse::Ok().json(user))
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




#[post("/auth/user/edit/{id}")] //-- required fields : username + phone_number + sex + age + email
async fn update(req: HttpRequest, id: web::Path<i32>, user: web::Json<UpdatableUser>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can edit a user data
                let user = QueryableUser::update_info(id.into_inner(), user.into_inner()).await?;
                Ok(HttpResponse::Ok().json(UpdatableUser{
                        username: user.username,
                        phone_number: user.phone_number,
                        sex: user.sex,
                        age: user.age,
                        email: user.email,
                        updated_at: Some(user.updated_at),
                    })
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




#[post("/auth/user/edit/password/{id}")] //-- required fields : current_password + password
async fn update_pwd(req: HttpRequest, id: web::Path<i32>, user: web::Json<PasswordFields>) -> Result<HttpResponse, uniXerr>{ //-- on Err result the error_message field of the uniXerr struct inside an actix http response as a json will return
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can edit a user password
                let user = QueryableUser::update_password(id.into_inner(), user.into_inner()).await.unwrap(); // can't solve error with `?` when the error is of type String and update_password() method will return String 
                Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_UPDATE_SUCCESS, constants::EMPTY)))
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





#[post("/auth/user/{id}/transfer/{coins}/{friend_id}")]
async fn transfer_coins(req: HttpRequest, id: web::Path<i32>, friend_id: web::Path<i32>, coins: web::Path<i32>) -> Result<HttpResponse, uniXerr>{ //-- on Err result the error_message field of the uniXerr struct inside an actix http response as a json will return
    match pass(req){
        Ok(user_data_inside_token) => { //-- updating coins process is based on the user token not the wallet address
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can borrow coins
                let loan_borrow_coins_status = QueryableUser::update_coins(id.into_inner(), friend_id.into_inner(), coins.into_inner()).await.unwrap(); 
                Ok(HttpResponse::Ok().json(loan_borrow_coins_status))
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





// NOTE - extracting binary data from request
#[post("/auth/user/edit/profile/{id}")]
async fn update_prof(req: HttpRequest, id: web::Path<i32>, mut prof_img: Multipart) -> Result<HttpResponse, Error>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can edit a user profile image
                fs::create_dir_all(constants::UPLOAD_PATH)?;
                let mut filename = "".to_string();
                while let Ok(Some(mut field)) = prof_img.try_next().await{ // NOTE - parsing the incoming prof_img stream into MultipartItem instances and getting the result of each stream future by blocking the current thread using .await
                    let content_type = field.content_disposition().unwrap();
                    filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), content_type.get_filename().unwrap());
                    let filepath = format!("{}/{}", constants::UPLOAD_PATH, sanitize_filename::sanitize(&filename));
                    let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
                    while let Some(chunk) = field.next().await{
                        let data = chunk.unwrap();
                        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
                    }
                }
                let res = UploadFile{
                    name: filename,
                    time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                };
                let user = QueryableUser::update_prof_img(id.into_inner(), res).await?;
                Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_UPDATE_SUCCESS, constants::EMPTY)))
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





#[get("/auth/user/profile/{id}")]
async fn download_prof(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can download a user profile image
                let user = QueryableUser::find_by_id(id.into_inner()).await?;
                let user_img = user.prof_img.unwrap();
                let path = format!("{}/{}", constants::UPLOAD_PATH, user_img);
                if !Path::new(path.as_str()).exists(){
                    return Ok(
                        HttpResponse::NotFound().json(&UploadFile{
                            name: user_img,
                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                        })
                    );
                }
                let data = fs::read(path).unwrap(); // NOTE - convert the file into a utf-8 bytes vector
                Ok(HttpResponse::Ok().header("Content-Disposition", format!("form-data; filename={}", user_img)).body(data))
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






#[post("/auth/user/delete/{id}")]
async fn delete(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, uniXerr>{
    match pass(req){
        Ok(user_data_inside_token) => {
            let access_level = user_data_inside_token.unwrap().claims.access_level;
            if access_level == 1 || access_level == 2{ // NOTE - only admin and regular user can delete a user
                let deleted_user = QueryableUser::delete(id.into_inner()).await?;
                Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
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





pub fn user_init_service(config: &mut web::ServiceConfig){
    config.service(find_all);
    config.service(find);
    config.service(add);
    config.service(update);
    config.service(update_pwd);
    config.service(transfer_coins);
    config.service(update_prof);
    config.service(download_prof);
    config.service(delete);
}
