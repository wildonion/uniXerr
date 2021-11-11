



#[macro_export]
macro_rules! user_data {
    ($user_id:expr, $token:expr) => { //-- we have to use match on where this macro is called
        {

            use serde::{Deserialize, Serialize};


            #[derive(Debug, Serialize, Deserialize)]
            pub struct UserData{
                pub username: String,
                pub email: String,
                pub phone_number: String,
                pub wallet_address: String,
                pub balance: i32,
                pub sex: String,
                pub age: i16,
            }


            let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set auth port in .env");
            let host = env::var("HOST").expect("⚠️ please set host in .env");
            let url = format!("http://{}:{}/auth/user/get/{}", host, coiniXerr_http_port, $user_id);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .get(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<UserData>().await{ //-- deserializing response utf8 bytes into the UserData struct
                                    Ok(resp) => {
                                        println!("[+] CURRENT SERVER TIME : {:?} | USER DATA FROM THE AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp);
                                        Ok(resp)
                                    },
                                    Err(e) => {
                                        println!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                println!("[!] CURRENT SERVER TIME : {:?} | AUTH MICROSERVICE SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    println!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}




#[macro_export]
macro_rules! authenticity {
    ($token:expr) => {
        {

            use serde::{Deserialize, Serialize};


            #[derive(Debug, Serialize, Deserialize)]
            struct ResponseBody{
                pub message: String,
                pub data: UserId, // NOTE - this is a string pretty json and we have to deserialize it into UserId struct
            }


            #[derive(Serialize, Deserialize, Debug)]
            struct UserId{
                pub user_id: i32,
            }


            let auth_port = env::var("AUTH_PORT").expect("⚠️ please set auth port in .env");
            let host = env::var("HOST").expect("⚠️ please set host in .env");
            let url = format!("http://{}:{}/auth/check-token", host, auth_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<ResponseBody>().await{ //-- deserializing response utf8 bytes into the ResponseBody struct
                                    Ok(resp) => {
                                        println!("[+] CURRENT SERVER TIME : {:?} | RESPONSE MESSAGE FROM AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp.message);
                                        println!("[+] CURRENT SERVER TIME : {:?} | USER ID FROM THE AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp.data.user_id);
                                        Ok(resp.data.user_id)
                                    },
                                    Err(e) => {
                                        println!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                println!("[!] CURRENT SERVER TIME : {:?} | AUTH MICROSERVICE SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    println!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}