





/*




    NOTE - actix actors are used for sending messages and events through their address instead of blocking the local thread for mutex acquisition using mpsc channel
    NOTE - all actix actors are on top of tokio in which every future task like actors communication events and messages will be handled by mpsc job q channel and multithreading patterns
    NOTE - mpsc channel can be used to communicate between threads while we're using a thread pool to mutate a data structure by locking on the data and blocking the local thread to acquires the mutex and prevent other thread from mutating and locking it at a same time to avoid being in dead lock situation
    NOTE - the sender of mpsc channel can be owned by multiple threads but the receiver can only be owned by only one thread at a time, that's because it's called multi producer and single consumer  
    NOTE - we have to clone the receiver for passing between multiple threads and for mutating what's in it we have to put it inside a Mutex to insure that only one thread can change the content of the receiver at a time
    NOTE - mutex acquisition is done by waiting on the receiver until a job or task becomes available to down side of the channel then locking on the receiver to acquire the mutex
    NOTE - multiple producers or workers own the receiver (Ac<T>) and single consumer or worker get the job at a time from the receiver (Mutex<T>)
    NOTE - in order to move all data through the socket or http protocol they must be encoded from struct and converted to &[u8] serde codec serialize 
    NOTE - in order to get the data from the socket or http protocol they must be decoded from &[u8] to struct using serde codec deserialize
    TODO - a codec for streaming of binary data like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice or mapping struct into &[u8] bytes
    TODO - implement tokio channels like mpsc, oneshot, broadcast and watch
    TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
    https://github.com/actix/examples/blob/master/websockets/tcp-chat/src/codec.rs
    https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    https://stackoverflow.com/questions/2490912/what-are-pinned-objects
    https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
    https://github.com/zupzup/warp-websockets-example
    https://github.com/tokio-rs/tokio/tree/master/examples
    https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
    https://danielkeep.github.io/tlborm/book/
    https://cetra3.github.io/blog/implementing-a-jobq/
    https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
    https://docs.rs/tokio/1.12.0/tokio/sync/index.html
    https://docs.rs/tokio-stream/0.1.7/tokio_stream/
    https://doc.rust-lang.org/std/pin/index.html
    https://doc.rust-lang.org/std/sync/struct.Arc.html
    https://doc.rust-lang.org/std/sync/struct.Mutex.html
    https://doc.rust-lang.org/std/sync/struct.RwLock.html
    https://doc.rust-lang.org/std/cell/struct.RefMut.html
    https://doc.rust-lang.org/std/cell/struct.RefCell.html
    https://doc.rust-lang.org/std/rc/struct.Weak.html
    https://doc.rust-lang.org/std/rc/struct.Rc.html
    https://danielkeep.github.io/practical-intro-to-macros.html
    https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
    https://blog.logrocket.com/procedural-macros-in-rust/
    http://gradebot.org/doc/ipur/trait.html
    https://cheats.rs/#behind-the-scenes
    https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1




*/



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


            let url = format!("http://localhost:7366/uniXerr/api/auth/user/get/{}", $user_id);
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


            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post("http://localhost:7366/uniXerr/api/auth/check-token")
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