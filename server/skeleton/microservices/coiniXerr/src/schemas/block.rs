








use serde::{Serialize, Deserialize};
use uuid::Uuid;






#[repr(C)] //-- #[repr(C, packed)] : borrow of packed field is unsafe and requires unsafe function or block
union TransactionMem{
    pub data: *mut self::Transaction,
    pub buffer: *const u8,
}






#[derive(Serialize, Deserialize, Clone, Debug, Copy)] //-- Serialize from struct into json and Deserialize from json back to struct
pub struct Transaction{
    pub id: Uuid,
    pub timestamp: Option<i64>,
}



impl Default for Transaction{
    fn default() -> Self{
        todo!()
    }
}



impl Transaction{
    fn new(buffer: &[u8]) -> Option<&mut Transaction>{
        unsafe impl Send for Transaction {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for Transaction union in order to be shareable between tokio threads
        unsafe{
            let mut gps_data: Option<&mut Transaction> = None; //-- in order to prevent from null checking we took a safe rust based reference to Transaction inside the Option (cause rust doesn't have null) cause *mut raw pointer to Transaction inside union is not safe also can't move between thread safely and once the TransactionMem data has dereferenced it might return a null pointer (dangled) due to unsafe manner of C pointers
            let gps = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //-- getting a C pointer of the filled buffer which is the hex address of the memory, doning this is unsafe due to unsafeness manner of raw pointers in rust
            gps_data = Some(&mut (*gps.data)); //-- taking a reference (smart pointer to address of gps.data in the stack) from dereferenced *mut raw pointer from the union inside the unsafe block is done through * like in C syntax  - we only wants the data so we didn't do any read operation on buffer field inside the union
            gps_data
        }
    }
}
