





 


// transaction capnp build schemas method


use capnpc::CompilerCommand;



fn main(){
    // ::capnpc::CompilerCommand::new().file("transaction.capnp").run().unwrap();
    CompilerCommand::new()
        .file("transaction.capnp")
        .run()
        .unwrap(); //// building the transaction capnp schema to generate the rust code
}