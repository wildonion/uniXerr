



pub mod auth{
    use crate::constants;
    use log::{info, error};
    use actix_web::{HttpRequest, http::Method};
    use crate::utils::jwt::{user_token::UserToken, token_utils};
    use jsonwebtoken::TokenData;

    

    // NOTE - checking the token inside each incoming request header
    pub fn pass(req: HttpRequest) -> Result<Option<TokenData<UserToken>>, String>{
        println!("-> Middleware Auth Checking Request : {}", req.path());
        let mut authenticate_pass: bool = false;
        let mut user_data_inside_token: Option<TokenData<UserToken>> = None; 
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else{
            for ignore_route in constants::IGNORE_ROUTES.iter(){
                if req.path().starts_with(ignore_route){
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass{
                if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION){
                    info!("Parsing authorization header...");
                    if let Ok(authen_str) = authen_header.to_str(){
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer"){
                            info!("Parsing token...");
                            let token = authen_str[6..authen_str.len()].trim();
                            if let Ok(token_data) = token_utils::decode_token(token.to_string()){
                                info!("Valid token");
                                authenticate_pass = true;
                                user_data_inside_token = Some(token_data);
                            } else{
                                error!("Invalid token");
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass{
            Ok(user_data_inside_token)
        } else{
            Err(constants::MESSAGE_INVALID_TOKEN.to_string())
        }
    }
}