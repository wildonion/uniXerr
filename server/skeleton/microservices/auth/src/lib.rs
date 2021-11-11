



#[macro_export]
macro_rules! send_transaction {
    ($transaction:expr) => {
        {

            use serde::{Deserialize, Serialize};


            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub struct Transaction{
                pub id: Uuid,
                pub ttype: u8,
                pub amount: i32,
                pub from_address: String,
                pub to_address: String,
                pub issued: i64,
                pub signed: Option<i64>,
                pub signature: Option<String>, //-- it's must be signed using sender's private key
                pub hash: String,
            }


            let coini_X_err_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set auth port in .env");
            let host = env::var("HOST").expect("⚠️ please set host in .env");
            let url = format!("http://{}:{}/coiniXerr/transaction", host, coini_X_err_http_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post(&url)
                        .body($transaction)
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<Transaction>().await{ //-- deserializing response utf8 bytes into the Transaction struct
                                    Ok(resp) => {
                                        println!("[+] CURRENT SERVER TIME : {:?} | MINED TRANSACTION FROM THE AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp);
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