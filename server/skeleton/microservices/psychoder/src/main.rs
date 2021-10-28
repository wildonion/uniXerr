



mod schemas;
mod networks;
mod mathista;
use networks::Model;
use networks::mlp::Linear;
use networks::NetworkType;
use schemas::brain::Neuron;





#[tokio::main] //-- await is only allowd inside an async function due to this reason we're using the tokio as a runtime to make the main() function as an async one
async fn main() -> std::io::Result<()>{
    


    let x_train = vec![ vec![0.24, 0.345, 0.23, 0.13], 
                                     vec![0.24, 0.345, 0.23, 0.456], 
                                     vec![0.24, 0.345, 0.25, 0.212]
                                   ]; //-- it's a (3 X 4) matrix of input data


    let model = Model{ // TODO - take model parameters from cli args
                                    networks: vec![
                                                    NetworkType::Linear(Linear{neural_circuit: vec![Neuron::default()]}),
                                                    NetworkType::Linear(Linear{neural_circuit: vec![Neuron::default(), Neuron::default()]})
                                                ],
                                    is_training: true,
                                    epochs: 200,
                                    batch_size: 64,
                                    device: "cpu".to_string(),
                            };


    model.train(x_train).await;



    Ok(())



}
