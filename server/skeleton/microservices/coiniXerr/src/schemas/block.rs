




// NOTE - all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
// NOTE - there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
// NOTE - enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself




use serde::{Serialize, Deserialize};
use uuid::Uuid;











#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block{
    pub id: Uuid,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //-- can't implement the Copy trait for Vec thus can't bound it to the Block structure 
}












unsafe impl Send for TransactionMem {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads
union TransactionMem{
    pub data: *mut self::Transaction, //-- defining the data as a raw mutable pointer to a mutable Transaction object, changing the data will change the Transaction object and vice versa
    pub buffer: *const u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub id: Uuid,
    pub amount: i32,
    pub from_address: String,
    pub to_address: String,
    pub timestamp: i64,
    pub is_mined: bool,
}

impl Default for Transaction{
    fn default() -> Self{
        todo!()
    }
}

impl Transaction{
    
    pub fn test_from_raw(){
        ///// -------------- changing the vaule in runtime using pointer -------------- /////
        let v = vec![1, 2, 3];
        let mut v = std::mem::ManuallyDrop::new(v); // a wrapper to inhibit compiler from automatically calling T’s destructor, this wrapper is 0-cost
        let pointer = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        unsafe{
            for i in 0..len as isize{
                std::ptr::write(pointer.offset(i), 4 + i);
            }

            let rebuilt = Vec::from_raw_parts(pointer, len, cap);
            assert_eq!(rebuilt, [4, 5, 6]);
        }

        let mut a = "another_wo".to_string();
        // changing the value of 'a' by changing the offset of its u8 raw pointer
        let new_a = unsafe{
            //         ptr      len  capacity
            //       +--------+--------+--------+
            //       | 0x0123 |      2 |      4 |
            //       +--------+--------+--------+
            //             |
            //             v
            // Heap   +--------+--------+--------+--------+
            //       |    'a' |    'n' | uninit | uninit |
            //       +--------+--------+--------+--------+
            let ptr_of_a = a.as_mut_ptr(); // *mut u8 - a pointer to the code ascii of the String
            let len = a.len();
            let cap = a.capacity();
            std::ptr::write(ptr_of_a.offset(5), 117); // 117 is the code ascii of `u`
            String::from_raw_parts(ptr_of_a, len, cap)
            // let mut changed_offset = ptr_of_a.offset(5);
            // String::from_raw_parts(changed_offset, len, cap)
            
        };

        println!("new `a` is {}", new_a);

        ///// ------------------------------------------------------------------------ /////
    }

    pub fn new() -> Self{
        todo!()
    }

    pub fn from_mem(buffer: &[u8]) -> Result<&mut Self, Box<dyn std::error::Error>>{
        unsafe{
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //-- filling the buffer field will also fill the data cause thay have a same memory storage
            let deserialized_transaction = &mut *transaction.data.clone(); //-- since the data inside the union is a pointer to a mutable Transaction object we have to return a dereferenced of the data which is a mutable object of Transaction
            Ok(deserialized_transaction)
        }
    }
}
