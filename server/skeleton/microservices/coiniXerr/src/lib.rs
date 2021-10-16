



// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1




#[macro_export]
macro_rules! user_data {
    ($user_id:expr, $token:expr) => {
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
            let url = format!("http://localhost:{}/uniXerr/api/auth/user/get/{}", coiniXerr_http_port, $user_id);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .get(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<UserData>().await{
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
            let url = format!("http://localhost:{}/uniXerr/api/auth/check-token", auth_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<ResponseBody>().await{
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




async fn cls_fn() {
    fn return_cls() -> Box<dyn FnOnce(i32) -> i32>{ //-- instances of FnOnce can be called, but might not be callable multiple times. Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once - FnOnce is a supertrait of FnMut
        Box::new(|x| x + 1)
    }    
    function_with_callback(return_cls()); // use .await to suspend the function execution for solving the future
}

async fn function_with_callback(cb: Box<dyn FnOnce(i32) -> i32>){
    cb(32);
}