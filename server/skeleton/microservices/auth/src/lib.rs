



#[macro_export]
macro_rules! send_transaction {
    ($transaction:expr) => {
        {

            use serde::{Deserialize, Serialize};


            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub struct Transaction{
                pub id: Uuid,
                pub amount: i32,
                pub from_address: String,
                pub to_address: String,
                pub issued: i64,
                pub signed: Option<i64>,
                pub hash: String,
            }


            let coini_X_err_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set auth port in .env");
            let url = format!("http://localhost:{}/uniXerr/api/coiniXerr/transaction", coini_X_err_http_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post(&url)
                        .body($transaction)
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<Transaction>().await{
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