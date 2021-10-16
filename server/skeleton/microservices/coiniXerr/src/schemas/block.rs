




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



fn test_from_raw(){
    ///// -------------- changing the vaule in runtime using its pointer -------------- /////
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

fn test_pointers(){
    ///// ------------------------------------------------------------------------------- /////
    ///// &T and &mut T will be coerced into raw pointers *const T or *mut T respectively
    ///// ------------------------------------------------------------------------------- /////
    let mut name = String::from("wildonion");
    println!("`name` value before changing the value of `mut_smart_pointer_to_name` pointer >>>> {}", name);
    println!("`name` has the address >>>> {:p}", &name);
    // NOTE - trait Copy is not implemented for &mut T because if we had the Copy trait for them we could have multiple copy of them 
    //        in every where and they could change the data of what they have borrowed for thus the reference count of each variable 
    //        would be out of control inside the current thread, those references don't want a copy of data, they want to own and mutate 
    //        the data inside what they have borrowed thus we can't mutate the variable itself while its mutable pointer is changing the data inside of it.
    let mut raw_mut_pointer_to_name = &mut name as *mut String;
    let mut another_raw_mut_pointer_to_name = &mut name as *mut String;
    let mut mut_smart_pointer_to_name = &mut name; // copying `name` into `mut_smart_pointer_to_name` - we can only have one mutable reference or borrower in a same scope
    println!("`mut_smart_pointer_to_name` has the address >>>> {:p}", &mut_smart_pointer_to_name);
    println!("`raw_mut_pointer_to_name` value is the address of `name` >>>> {:p}", raw_mut_pointer_to_name);
    println!("`another_raw_mut_pointer_to_name` value is the address of `name` >>>> {:p}", another_raw_mut_pointer_to_name);
    println!("`raw_mut_pointer_to_name` address is >>>> {:p}", &raw_mut_pointer_to_name);
    println!("`another_raw_mut_pointer_to_name` address is >>>> {:p}", &another_raw_mut_pointer_to_name);
    // NOTE - can't assign to `name` in this scope cause it's borrowed by `mut_smart_pointer_to_name`
    // NOTE - can't mutate the `name` when there is another variable or a pointer pointing to the `name`
    // NOTE - `mut_smart_pointer_to_name` pointer has the ability to change the value of `name`  
    *mut_smart_pointer_to_name = "another_wildonion".to_string(); // we can rewrite the data which `mut_smart_pointer_to_name` is refering to, cause `name` is mutable - change the value of a smart pointer by dereferencing it
    println!("name value after changing the value of its smart pointer or borrower >>>> {}", name);
    println!("`name` address after changing its value using `mut_smart_pointer_to_name` pointer >>>> {:p}", &name);
    // NOTE - we can assign to `name` after dereferencing the `mut_smart_pointer_to_name` pointer
    name = "third_wildonion".to_string(); // raw pointers will change also
    println!("`raw_mut_pointer_to_name` value after changing name value >>>> {}", unsafe{&*raw_mut_pointer_to_name});
    println!("`another_mut_pointer_to_name` value after changing name value >>>> {}", unsafe{&*another_raw_mut_pointer_to_name});
    // NOTE - can't mutate the `name` when it's behind the mutable pointer, the only way of changing its vlaue is changing its pointer value
    // NOTE - cannot assign to `name` if the following is uncommented because `name` is borrowed using a pointer
    //        and we are trying to print that pointer when we are changing the `name` value at the same time otherwise
    //        can't assign to `name` when we are using its mutable pointer, since based on multiple mutable references can't
    //        be exist at the same time thus we can't have the `name` and its pointer at the same time for writing and reading.
    // println!("`mut_smart_pointer_to_name` value after changing `name` value >>>> {}", mut_smart_pointer_to_name); 



    let mut a = String::from("wildonion");
    println!("value of `a` before changing >>>> {}", a);
    println!("the address of `a` >>>> {:p}", &a);
    let mut c = &mut a as *mut String; // mutable raw pointer to `a` - coercing &mut String into *mut String
    println!("`c` value >>>> {}", unsafe{&*c}); // `c` has the same value of `a` - we have to take a reference to dereferenced raw pointer cause every &mut will be coerced to *mut and to get the actual data we must take a reference to dereferenced raw pointer
    println!("`c` contains the address of `a` >>>> {:p}", c);
    println!("`c` address >>>> {:p}", &c);
    a = String::from("another_wildonion"); // changing `a` will change the `c` value also
    println!("value of `a` after changing >>>> {}", a);
    println!("`c` value after changing `a` >>>> {}", unsafe{&*c});
    println!("`c` contains the address of `a` >>>> {:p}", c);
    println!("`c` address after changing `a` >>>> {:p}", &c);
    unsafe{*c = String::from("third_wildonion");} // changing `c` will change the `a` value also cause `a` is a mutable variable
    println!("`c` value after changing >>>> {}", a);
    println!("value of `a` after changing `c` >>>> {}", a);
    println!("`c` contains the address of `a` after changing its value >>>> {:p}", c);
    println!("`c` address after changing its value >>>> {:p}", &c);


    
    let mut g = 24;
    let mut v = &mut g;
    let mut m = &mut v;
    **m = 353; // it'll change the `v` first, since `v` is a mutable reference to `g` by changing its value the value of `g` will be changed
    println!("`v` after changing `m` >>>> {}", v);
    println!("`g` after changing `m` >>>> {}", g);
    g = 2435; // changing `g` value inside another scope and lifeimte
    println!("`g` after changing >>>> {}", g);
    // NOTE - can't mutate the `g` when it's behind the mutable pointer, the only way of changing its vlaue is changing its pointer value
    // NOTE - cannot assign to `g` if the following is uncommented because `g` is borrowed using a pointer
    //        and we are trying to print that pointer when we are changing the `g` value at the same time otherwise
    //        can't assign to `g` when we are using its mutable pointer, since based on multiple mutable references can't
    //        be exist at the same time thus we can't have the `g` and its pointer at the same time for writing and reading.
    // println!("`mut_smart_pointer_to_name` value after changing `name` value >>>> {}", mut_smart_pointer_to_name); 



    let var = 242;
    let mut pointer_var = &var; // this immutable reference or borrower
    println!("`var` is not changed cause it's not mutable {}", var);
    // let changed = &2243253;
    // *pointer_var = *changed; // ERROR - even though `pointer_var` is mutable but can't rewrite the data which `pointer_var` is refering to cause `var` is not mutable
    // println!("`pointer_var` is not changed cause the data it referes (`var`) to it's not mutable {}", pointer_var);



    let mut a = 32;
    println!("`a` address before change is ===== {:p}", &a);
    println!("`a` value before change is ===== {}", a);
    let mut b: *const i32 = &a; // const raw pointer to the location of a 
    println!("`b` address before changing `a` value is ===== {:p}", &b);
    a = 34; // b will change 
    println!("`a` value after change is ===== {}", a);
    println!("`a` address after change is ===== {:p}", &a);
    println!("`b` value after changing `a` is ===== {}", unsafe{&*b});
    println!("`b` address after changing `a` value is ===== {:p}", &b);
    println!("`a` address inside the `b` ===== {:p}", b);
    // unsafe{*b = 56}; // ERROR - even though `b` and `a` is mutable but can't rewrite the data which `b` is refering to cause its type is const and pointing to a const variable which can't be changed by this pointer
    // println!("`b` value after change is ===== {}", unsafe{&*b});
    // println!("`a` address inside the `b` after changing `b` value is not the same as the old one ===== {:p}", b);
    // println!("`a` value after changing `b` is ===== {}", a);
    // println!("`b` address after changing `b` value is ===== {:p}", &b);
    // a = 235;
    // println!("`b` value after changing `a` is ===== {}", unsafe{&*b});

}
