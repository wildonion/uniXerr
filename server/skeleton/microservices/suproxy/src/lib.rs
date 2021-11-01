

use std::collections::HashMap;




// NOTE - Error is an object safe trait and it's Sized cause we don't know what type we have to implement this trait for at runtime 
//        thus we must take a reference to it by using Box<dyn std::error::Error> or &dyn std::error::Error
pub fn parser(text: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>{ 
    let mut params_dict = HashMap::new();
    let splitted_packet: Vec<&str> = text.split("&").collect();
    for param in splitted_packet{
        let splitted_param: Vec<&str> = param.split("=").collect();
        params_dict.insert(splitted_param[0].to_string(), splitted_param[1].to_string());
    }
    Ok(params_dict)
}



pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}



#[macro_export]
macro_rules! authenticity {
    ($token:expr) => {
        {

            use serde::{Deserialize, Serialize};
            use std::env;
            use dotenv::dotenv;

            
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
            let client = reqwest::blocking::Client::new();
            match client
                    .post(&url)
                    .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                    .send(){
                        Ok(res) => {
                            match res.json::<ResponseBody>(){ //-- deserializing response utf8 bytes into the ResponseBody struct
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
        }
    };
}
