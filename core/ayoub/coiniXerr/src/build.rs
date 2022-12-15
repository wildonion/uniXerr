





 


// capnp build schemas method


use capnpc::CompilerCommand;



fn main(){
    CompilerCommand::new().file("schemas.capnp").run().unwrap(); //// building the capnp schema to generate the rust code
}