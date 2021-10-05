



mod schemas;
mod networks;
mod mathista;
mod handlers;
use std::env;
use dotenv::dotenv;
use networks::Model;
use handlers::trainer::ThreadPool;
use networks::mlp::Linear;
use schemas::brain::Neuron;


#[tokio::main] //-- await is only allowd inside an async function due to this reason we're using the tokio as a runtime to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    



    let pool = ThreadPool::new(4);
    pool.execute(move || {
        Model{
            networks: vec![
                            Linear{neurons: vec![Neuron::default()]}
                          ],
            is_training: true,
        };
    });






    Ok(())



}
