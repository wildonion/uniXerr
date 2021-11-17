





use std::{env, slice, mem};
use crate::constants;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, AsChangeset};
use crate::handlers::error::uniXerr;
use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::utils::jwt::user_token::UserToken;
use crate::entities::login_history::model::QueryableLoginHistory;
use serde::{Deserialize, Serialize}; // NOTE - Deserialize from json to struct to insert into db, Serialize from struct to json to send the response 
use uuid::Uuid;
use crate::handlers::db::pg::establish as pg;
use liber;






///////////// =============================================== INSERTABLE STRUCTS ===============================================
#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    pub access_level: i16,
    pub id: i32,
    pub username: String,
    pub access_token: String,
}
#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct InsertableUser{
    pub wallet_address: String,
    pub password: String,
    pub username: String,
    pub phone_number: String,
    pub email: String,
    pub device_id: String,
    pub created_at: Option<chrono::NaiveDateTime>, //-- getting this field as Option means that it'll fill later automatically when building the insertable query
    pub updated_at: Option<chrono::NaiveDateTime>, //-- getting this field as Option means that it'll fill later automatically when building the insertable query
}
///////////// =============================================== UPDATABLE STRUCTS ===============================================
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username_or_email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub struct PasswordFields{
    pub current_password: String,
    pub password: String,
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UpdateCoins{
    pub coins: i32,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UpdatePassword{
    pub password: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}


#[derive(Deserialize, Serialize)]
pub struct UploadFile{
    pub name: String,
    pub time: u64,
}


#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name="users"]
pub struct UpdateProfImg{
    pub prof_img: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}


#[derive(Deserialize, Insertable, Serialize, AsChangeset)]
#[table_name="users"]
pub struct UpdatableUser{
    pub username: String,
    pub phone_number: String,
    pub sex: String,
    pub age: i16,
    pub email: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
///////////// =============================================== QUERYABLE STRUCTS ===============================================
#[derive(Serialize, Deserialize)]
pub struct UserData{
    pub username: String,
    pub email: String,
    pub phone_number: String,
    pub wallet_address: String,
    pub balance: i32,
    pub sex: String,
    pub age: i16,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction{
    pub id: Uuid,
    pub ttype: u8,
    pub amount: i32,
    pub from_address: String,
    pub to_address: String,
    pub issued: i64,
    pub signed: Option<i64>,
    pub signature: Option<String>, //-- it's going to be signed using sender's private key which is the access_token of the sender
    pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeliveredCoins{
    pub id: Uuid,
    pub amount: i32,
    pub from_address: String,
    pub to_address: String,
    pub transfer_timestamp: i64,
    pub signed: Option<i64>,
    pub user_id: i32,
    pub friend_id: i32,
    pub user_coins_after_loan: i32,
    pub friend_coins_after_borrowed: i32,
    pub send_time: i64,
    pub delivery_time: i64,
}

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[table_name="users"]
pub struct QueryableUser{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub wallet_address: String,
    pub access_token: String,
    pub access_level: i16,
    pub is_blocked: i16,
    pub phone_number: String,
    pub email: String,
    pub device_id: String,
    pub firebase_id: Option<String>,
    pub prof_img: Option<String>,
    pub coins: i32,
    pub sex: String,
    pub age: i16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl QueryableUser{

    pub async fn signup(user: InsertableUser) -> Result<String, String>{ // it returns error as String
        let conn = pg::connection().await.unwrap();
        if Self::find_user_by_username(&user.username).await.is_err(){ // no user found with this username
            let wallet_address = "a wallet address!".to_string(); // TODO - public key is used to generate wallet address
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = InsertableUser{
                wallet_address,
                password: hashed_pwd,
                username: user.username,
                phone_number: user.phone_number,
                email: user.email,
                device_id: user.device_id,
                created_at: Some(chrono::Local::now().naive_local()),
                updated_at: Some(chrono::Local::now().naive_local()),
            };
            diesel::insert_into(users::table).values(&user).execute(&conn).unwrap();
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else{ // means we found an already registered user 
            Err(format!("{} - User '{}' is already registered", constants::MESSAGE_SIGNUP_FAILED.to_string(), &user.username))
        }
    }

    pub async fn login(login: Login) -> Option<LoginInfo>{
        let conn = pg::connection().await.unwrap();
        if let Ok(user_to_verify) = users::table
            .filter(users::username.eq(&login.username_or_email))
            .or_filter(users::email.eq(&login.username_or_email))
            .get_result::<QueryableUser>(&conn){ // NOTE - we're returning the whole user info in here because of REST structure !!!!!
                if !user_to_verify.password.is_empty() && verify(&login.password, &user_to_verify.password).unwrap(){
                    if let Some(login_history) = QueryableLoginHistory::create(&user_to_verify.username).await{
                        match QueryableLoginHistory::add(login_history).await{
                            Ok(added_history) => {
                                // we added a record of login history into db
                                // ...
                            },
                            Err(err) => {
                                return None; // faced into and error while adding
                            }
                        }
                        let access_token_str = QueryableUser::generate_access_token().await;
                        if QueryableUser::update_access_token_to_db(&user_to_verify.username, &access_token_str).await{
                            let login_info = LoginInfo{
                                id: user_to_verify.id,
                                access_level: user_to_verify.access_level,
                                username: user_to_verify.username,
                                access_token: access_token_str,
                            };
                            return Some(login_info);
                        }   
                    }
                } else{
                    let login_info = LoginInfo{id: user_to_verify.id, access_level: user_to_verify.access_level, username: user_to_verify.username, access_token: user_to_verify.access_token};
                    return Some(login_info);
                }
            }
        None
    }

    pub async fn logout(user_id: i32){
        let conn = pg::connection().await.unwrap();
        if let Ok(user) = users::table.find(user_id).get_result::<QueryableUser>(&conn){
            Self::update_access_token_to_db(&user.username, "").await; // put an empty string instead of the current access token
        }
    }

    pub async fn is_valid_access_token(user_token: &UserToken) -> bool{
        let conn = pg::connection().await.unwrap();
        users::table
            .filter(users::username.eq(&user_token.user))
            .filter(users::access_token.eq(&user_token.access_token))
            .get_result::<QueryableUser>(&conn)
            .is_ok()
    }

    pub async fn find_user_by_username(un: &str) -> QueryResult<Self>{
        let conn = pg::connection().await.unwrap();
        let found_user = users::table.filter(users::username.eq(un)).get_result::<QueryableUser>(&conn);
        found_user
    }

    pub async fn generate_access_token() -> String{
        let access_token = Uuid::new_v4().to_simple().to_string();
        access_token
    }

    pub async fn update_access_token_to_db(un: &str, access_token_str: &str) -> bool{
        let conn = pg::connection().await.unwrap();
        if let Ok(user) = QueryableUser::find_user_by_username(un).await{
            diesel::update(users::table.find(user.id))
                .set(users::access_token.eq(access_token_str.to_string()))
                .execute(&conn)
                .is_ok()
        } else{
            false
        }
    }
    
    pub async fn find_all() -> Result<Vec<Self>, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let users = users::table.load::<QueryableUser>(&conn)?;
        Ok(users)
    }

    pub async fn find_by_id(id: i32) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let user = users::table.filter(users::id.eq(id)).first::<QueryableUser>(&conn)?;
        Ok(user)
    }

    pub async fn add(user: InsertableUser) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let wallet_address = "a wallet address!".to_string(); // TODO - public key is used to generate wallet address
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = InsertableUser{
            wallet_address,
            password: hashed_pwd,
            username: user.username,
            phone_number: user.phone_number,
            email: user.email,
            device_id: user.device_id,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
        };
        let user = diesel::insert_into(users::table).values(&user).get_result(&conn)?;
        Ok(user)
    }

    pub async fn update_info(id: i32, user: UpdatableUser) -> Result<Self, uniXerr>{ //-- Self refers to the User type
        let conn = pg::connection().await.unwrap();
        let user = UpdatableUser{
            username: user.username,
            phone_number: user.phone_number,
            sex: user.sex,
            age: user.age,
            email: user.email,
            updated_at: Some(chrono::Local::now().naive_local()),
        };
        let user = diesel::update(users::table.filter(users::id.eq(id))).set(user).get_result(&conn)?;
        Ok(user)
    }

    pub async fn update_password(id: i32, user: PasswordFields) -> Result<Self, String>{ // NOTE - `?` couldn't convert the error to `std::string::String` thus we can't use `?` to solve the error, instead we have to use unwrap()
        let conn = pg::connection().await.unwrap();
        let current_user = Self::find_by_id(id).await.unwrap(); // current_user contains all columns data inside the table
        if !current_user.password.is_empty() && verify(&user.current_password, &current_user.password).unwrap(){
            let new_password = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UpdatePassword{
                password: new_password,
                updated_at: Some(chrono::Local::now().naive_local()),
            };
            let user_with_new_password = diesel::update(users::table.filter(users::id.eq(id))).set(user).get_result(&conn).unwrap();
            Ok(user_with_new_password)
        } else{
            Err(constants::MESSAGE_INCORRECT_PASSWORD.to_string())
        }
    }

    pub async fn update_coins(id: i32, friend_id: i32, coins: i32) -> Result<DeliveredCoins, String>{ // NOTE - `?` couldn't convert the error to `std::string::String` thus we can't use `?` inside this function to solve the error, instead we have to use unwrap()
        let conn = pg::connection().await.unwrap();
        let current_user = Self::find_by_id(id).await.unwrap(); // current_user contains all columns data inside the table
        let current_user_friend = Self::find_by_id(friend_id).await.unwrap(); // current_user_friend contains all columns data inside the table
        if current_user.coins != 0 && current_user.coins > 0{
            let new_transaction = Transaction{ //-- creating new transaction to add to the blockchain (created block)
                id: Uuid::new_v4(),
                ttype: 0x00, //-- 0x00 means 0 in hex and a regular transaction - 0xFF or 1 (CRC21) and 0x02 or 2 (CRC20) and 0x03 or 3 (CRC22) in hex means smart contract transaction
                amount: coins,
                from_address: current_user.wallet_address,
                to_address: current_user_friend.wallet_address,
                issued: chrono::Local::now().naive_local().timestamp(),
                signed: None,
                signature: Some("sign the current transaction using genesis private key".to_string()), // TODO - it must be signed using the sender's private key which is the user access_token generated after successful registeration - transaction object needs to be signed using the sender's private key and this cryptographically proves that the transaction could only have come from the sender and was not sent fraudulently
                hash: "hash of the current transaction".to_string(), // TODO -
            };
            let new_transaction_bytes: &[u8] = unsafe { //-- encoding process of new transaction by building the &[u8] using raw parts of the struct - serializing a new transaction struct into &[u8] bytes
                //-- converting a const raw pointer of an object and its length into the &[u8], the len argument is the number of elements, not the number of bytes
                //-- the total size of the generated &[u8] is the number of elements (each one has 1 byte size) * mem::size_of::<Transaction>() and it must be smaller than isize::MAX
                //-- here number of elements or the len for a struct is the size of the total struct which is mem::size_of::<Transaction>()
                slice::from_raw_parts(&new_transaction as *const Transaction as *const u8, mem::size_of::<Transaction>()) 
            }; 
            let transfered_transaction_resp = match liber::send_transaction!(new_transaction_bytes){ //-- sending a binary stream of transaction data (serialized into bytes) to the coiniXerr network for mining and consensus process
                Ok(transfered_transaction) => {
                    if let Some(signed_transaction) = transfered_transaction.signed{ //-- if the transaction was added to the blockchain means it's confirmed and we can update both user coins
                        let updated_user_coins = UpdateCoins{
                            coins: current_user.coins - coins,
                            updated_at: Some(chrono::Local::now().naive_local()),
                        };
                        let updated_user_friend_coins = UpdateCoins{
                            coins: current_user_friend.coins + coins,
                            updated_at: Some(chrono::Local::now().naive_local()),
                        };
                        let current_user_with_updated_coins = diesel::update(users::table.filter(users::id.eq(id))).set(updated_user_coins).get_result::<QueryableUser>(&conn).unwrap();
                        let current_user_friend_with_updated_coins = diesel::update(users::table.filter(users::id.eq(friend_id))).set(updated_user_friend_coins).get_result::<QueryableUser>(&conn).unwrap();
                        let signed_transaction = DeliveredCoins{
                            id: transfered_transaction.id,
                            amount: transfered_transaction.amount,
                            from_address: transfered_transaction.from_address,
                            to_address: transfered_transaction.to_address,
                            transfer_timestamp: transfered_transaction.issued,
                            signed: Some(signed_transaction),
                            user_id: current_user_with_updated_coins.id,
                            friend_id: current_user_friend_with_updated_coins.id,
                            user_coins_after_loan: current_user_with_updated_coins.coins,
                            friend_coins_after_borrowed: current_user_friend_with_updated_coins.coins,
                            send_time: current_user_with_updated_coins.updated_at.timestamp(), //-- the time that the user sent his/her coins
                            delivery_time: current_user_friend_with_updated_coins.updated_at.timestamp(), //-- the time that the friend borrowed coins
                        };
                        Ok(signed_transaction)
                    } else{
                        Err(constants::MESSAGE_NOT_MINED_TRANSACTION.to_string()) //-- can't transfer coins
                    }
                },
                Err(e) => {
                    println!("[!] SERVER TIME : {} | FAILED TO SEND TRANSACTION TO THE coiniXerr NETWORK CAUSED BY : {} ", chrono::Local::now().naive_local(), e);
                    Err(constants::MESSAGE_SEND_TRANSACTION_FAILED.to_string()) 
                }
            };
            Ok(transfered_transaction_resp.unwrap())
        } else{
            Err(constants::MESSAGE_NOT_ENOUGH_COINS.to_string())
        }
    }

    pub async fn update_prof_img(id: i32, f: UploadFile) -> Result<Self, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let user_prof = UpdateProfImg{
            prof_img: f.name,
            updated_at: Some(chrono::Local::now().naive_local()),
        };
        let user_with_updated_profile_img = diesel::update(users::table.filter(users::id.eq(id))).set(user_prof).get_result(&conn).unwrap();
        Ok(user_with_updated_profile_img)
    }

    pub async fn delete(id: i32) -> Result<usize, uniXerr>{
        let conn = pg::connection().await.unwrap();
        let response = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?; //-- usize is the size of allocated bytes in memory to take a reference from any type like on i32 is 4 bytes
        Ok(response)
    }
}
///////////// =========================================================================================================================

