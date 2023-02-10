





 
//// build.rs can be used to build other lang codes
//// before the Rust crate can be compiled also
//// placing a `build.rs` in the root of a package
//// will cause Cargo to compile that script and 
//// execute it just before building the package
//// and Rust crates which is useful for building 
//// native C or C++ code as part of the package.


use capnpc::CompilerCommand; //// loaded in Cargo.toml in [build-dependencies] section


// https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts
fn main() -> Result<(), Box<dyn std::error::Error>>{
    // ::capnpc::CompilerCommand::new().file("transaction.capnp").run().unwrap();
    CompilerCommand::new() //// OUT_DIR locations are also handled by the capnpc library
        .file("transaction.capnp")
        .run()
        .unwrap(); //// building the transaction capnp schema to generate the rust code
}