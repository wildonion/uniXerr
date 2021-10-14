








use serde::{Serialize, Deserialize};
use uuid::Uuid;






// NOTE - all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
// NOTE - there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
// NOTE - enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself





#[repr(C)] //-- #[repr(C, packed)] : borrow of packed field is unsafe and requires unsafe function or block
union TransactionMem{
    pub data: *mut self::Transaction,
    pub buffer: *const u8,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block{
    pub id: Uuid,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //-- can't implement the Copy trait for Vec thus can't bound it to the Block structure 
}



#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Transaction{
    pub id: Uuid,
    pub amount: i32,
    pub timestamp: i64,
}



impl Default for Transaction{
    fn default() -> Self{
        todo!()
    }
}



impl Transaction{
    fn new(buffer: &[u8]) -> Option<&mut Transaction>{
        unsafe impl Send for Transaction {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads
        unsafe{
            let mut transaction_data: Option<&mut Transaction> = None; //-- in order to prevent from null checking we took a safe rust based reference to Transaction inside the Option (cause rust doesn't have null) cause *mut raw pointer to Transaction inside union is not safe also can't move between thread safely and once the TransactionMem data has dereferenced it might return a null pointer (dangled) due to unsafe manner of C pointers
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //-- getting a C pointer of the filled buffer which is the hex address of the memory, doning this is unsafe due to unsafeness manner of raw pointers in rust
            transaction_data = Some(&mut (*transaction.data)); //-- taking a reference (smart pointer to address of gps.data in the stack) from dereferenced *mut raw pointer from the union inside the unsafe block is done through * like in C syntax  - we only wants the data so we didn't do any read operation on buffer field inside the union
            transaction_data
        }
    }
}
