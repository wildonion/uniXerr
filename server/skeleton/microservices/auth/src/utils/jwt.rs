




///////////// ==========================================================================================================================

pub mod user_token{
    
    use serde::{Deserialize, Serialize};
    use std::env;
    use crate::entities::users::model::LoginInfo;
    use chrono::Utc;
    use jsonwebtoken::{
        EncodingKey, 
        Algorithm,
        Header
    };

    
    #[derive(Serialize, Deserialize)]
    pub struct UserToken{
        pub iat: i64, // issued at
        pub exp: i64, // expiration at
        pub user: String,
        pub id: i32,
        pub access_level: i16,
        pub access_token: String,
    }
    
    impl UserToken{
        pub fn generate_token(login: &LoginInfo) -> String{
            let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
            let payload = UserToken{
                iat: now,
                exp: now + env::var("JWT_EXPIRATION").expect("⚠️ found no jwt expiration time").parse::<i64>().unwrap(),
                user: login.username.clone(), // need to clone cause it's String in order not to move its ownership
                id: login.id, // doesn't need to clone cause it's integer
                access_level: login.access_level,
                access_token: login.access_token.clone(), // need to clone cause it's String in order not to move its ownership
            };
            let secret_key = env::var("JWT_SECRET_KEY").expect("⚠️ found no jwt secret key"); 
            let key: &[u8] = &secret_key.as_str().as_bytes(); // this is salt
            jsonwebtoken::encode(&Header::new(Algorithm::HS512), &payload, &EncodingKey::from_secret(key)).unwrap()
        }
    }
}

///////////// ==========================================================================================================================

pub mod token_utils{
    
    use std::env;
    use jsonwebtoken::{DecodingKey, TokenData, Validation, Algorithm};
    use crate::entities::users::model::QueryableUser;

    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<super::user_token::UserToken>>{
        let secret_key = env::var("JWT_SECRET_KEY").expect("⚠️ found no jwt secret key"); 
        let key: &[u8] = &secret_key.as_str().as_bytes();
        match jsonwebtoken::decode::<super::user_token::UserToken>(&token, &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS512)){
            Ok(token_data) => Ok(token_data),
            Err(e) => { println!("[!] Decoding token error - {:?} ", e); Err(e) }
        }
    }

    pub async fn verify_token(token_data: &TokenData<super::user_token::UserToken>) -> Result<&super::user_token::UserToken, String>{
        if QueryableUser::is_valid_access_token(&token_data.claims).await{
            let user_data_inside_token = &token_data.claims;
            Ok(user_data_inside_token)
        } else{
            Err("Invalid token".to_string())
        }
    }
}

// =================================================================================================================
