





 


// transaction capnp build schemas method


use capnpc::CompilerCommand;


// https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts
fn main(){
    // ::capnpc::CompilerCommand::new().file("transaction.capnp").run().unwrap();
    CompilerCommand::new()
        .file("transaction.capnp")
        .run()
        .unwrap(); //// building the transaction capnp schema to generate the rust code
}