



fn unsafer(){

    ///// -------------- changing the vaule in runtime using its pointer -------------- /////
    let v = vec![1, 2, 3];
    // let raw_parts = v.into_raw_parts(); //-- getting the pointer, len and capacity of the vector only in unstable rust! 
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
    
    ///// -------------- union, enum and struct -------------- /////

    #[repr(u32)]
    enum Tag{I, F}
    #[repr(C)]
    union U{
        i: i32,
        f: f32,
    }

    #[repr(C)]
    struct Value{
        tag: Tag,
        u: U,
    }

    fn is_zero(v: Value) -> bool{
        unsafe{
            match v{
                Value{tag: Tag::I, u: U{i: 0}} => true,
                Value{tag: Tag::F, u: U{f: num}} if num == 0.0 => true,
                _ => false,
            }
        }
    }

    ///// -------------- casting using raw pointer and transmute -------------- /////

    fn foo() -> i32{
        0
    }
    let pointer_to_func = foo as *const ();
    let func = unsafe{ // transmute the raw pointer of the function back into the function with i32 signature
        std::mem::transmute::<*const (), fn() -> i32>(pointer_to_func)
    };
    assert_eq!(func(), 0);


    let num_ = 10;
    let num_ptr: *const u8 = &num_; // ptr of num_
    let num = 10 as *const i32; // turn num into a constant raw pointer 
    let deref_raw_pointer_num = unsafe{&*num}; // dereferencing the raw pointer
    

    let mut name_ptr: *const u8;
    name_ptr = std::ptr::null(); // fill it with null pointer
    let name: &str = "wildonion";
    name_ptr = name.as_ptr(); // fill it with name bytes



    let c_const_pointer = 32 as *const i16;
    let c_mut_pointer = 64 as *mut i64;
    let thing1: u8 = 89.0 as u8;
    assert_eq!('B' as u32, 66);
    assert_eq!(thing1 as char, 'Y');
    let thing2: f32 = thing1 as f32 + 10.5;
    assert_eq!(true as u8 + thing2 as u8, 100);


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
    println!("`c` value >>>> {}", unsafe{&*c}); // `c` has the same value of `a` - we have to take a reference to dereferenced raw pointer cause *c is of type String which is not bounded to trait Copy thus we have to take a reference to it to move out of unsafe block
    println!("`c` contains the address of `a` >>>> {:p}", c);
    println!("`c` address >>>> {:p}", &c);
    a = String::from("another_wildonion"); // changing `a` will change the `c` value also
    println!("value of `a` after changing >>>> {}", a);
    println!("`c` value after changing `a` >>>> {}", unsafe{&*c});
    println!("`c` contains the address of `a` >>>> {:p}", c);
    println!("`c` address after changing `a` >>>> {:p}", &c);
    unsafe{*c = String::from("third_wildonion");} // changing `c` will change the `a` value also cause `a` is a mutable variable and `c` is a pointer to the `a`
    println!("`c` value after changing >>>> {}", a);
    println!("value of `a` after changing `c` >>>> {}", a);
    println!("`c` contains the address of `a` after changing its value >>>> {:p}", c);
    println!("`c` address after changing its value >>>> {:p}", &c);



    // NOTE - changing the value of the varibale using its pointer or its shallow copy and both the pointer and the object must be defined mutable
    // NOTE - making a deep copy from the object is done by cloning the object using clone() method (trait Clone must be implemented) to prevent double free pointer issue from happening
    let mut a = 32;
    let mut b = &mut a as *mut i32;
    println!("`b` value >>>> {}", unsafe{*b});
    println!("`a` address [{:p}] == `b` address [{:p}]", &a, b);
    a = 3535; //-- `b` will be changed
    println!("`b` value >>>> {}", unsafe{*b});
    unsafe{*b = 2435;} //-- `a` will be changed
    println!("`a` value >>>> {}", a);
    let deref_pointer = unsafe{&*b}; //-- a pointer to the dereferenced const raw pointer to the `a`



    
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
    // println!("`v` value after `g` value >>>> {}", v);
    // println!("`m` value after `g` value >>>> {}", m);



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



    ///// ------------------------------------------------------------------------ /////
    
    #[derive(Debug)]
    struct Test{
        a: String,
        b: *const String,
    }

    impl Test{
        fn new(txt: &str) -> Self{
            Test{
                a: String::from(txt),
                b: std::ptr::null(), // b is a const raw pointer to a String
            }
        }

        fn init(&mut self){
            // self.b = &self.a as *const String;
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str{
            &self.a
        }

        fn b(&self) -> &String{
            assert!(!self.b.is_null(), "call Test::init first");
            unsafe{&(*self.b)} // expected `&String` in return signature and because of that we dereferenced the `b` with & - `b` has the address of `a`, in order to get its value we have to dereference it which has the value same as `a`
        }
    }
    
    
    let mut test1 = Test::new("test1");
    println!("\n======== BEFORE INIT ========");
    println!("test1 `b` null pointer before init {:p}", test1.b);
    test1.init();
    let mut test2 = Test::new("test2");
    println!("test2 `b` null pointer before init {:p}", test2.b);
    test2.init();
    println!("\n======== BEFORE SWAP ========");
    println!("test1 `a` address {:p}", &test1.a);
    println!("test1 `b` address same as `a` address {:p}", test1.b); // same as `a` address
    println!("test1 `b` address itself {:p}", &test1.b); // different from `a` address cause this is the b address itself
    
    
    println!("test2 `a` address {:p}", &test2.a);
    println!("test2 `b` address same as `a` address {:p}", test2.b); // same as `a` address
    println!("test1 `b` address itself {:p}", &test2.b); // different from `a` address cause this is the b address itself
    
    
    println!("`a` and `b` for test1 - {}, {}", test1.a(), test1.b());
    println!("`a` and `b` for test2 - {}, {}", test2.a(), test2.b());
    
    
    
    println!("\n======== CHANGING THE a VALUE OF TEST1 ========");
    test1.a = "another_test1".to_string(); //  `b` will change cause is a pointer to the location of `a`
    println!("`a` and `b` for test1 - {}, {}", test1.a(), test1.b());
    println!("`a` and `b` for test2 - {}, {}", test2.a(), test2.b());
    
    
    
    std::mem::swap(&mut test1, &mut test2); // move test2 into test1
    println!("\n======== AFTER SWAP ========");
    println!("test1 `a` address remain the same {:p}", &test1.a);
    println!("test1 `b` address same as test2 `b` before swapping  {:p}", test1.b); // same as `a` address
    println!("test2 `a` address remain the same {:p}", &test2.a);
    println!("test2 `b` address same as test1 `b` before swapping {:p}", test2.b); // same as `a` address
    println!("`a` and `b` for test1 - {}, {}", test1.a(), test1.b());
    println!("`a` and `b` for test2 - {}, {}", test2.a(), test2.b());
    


    
    // NOTE - both `b` variables' value will remain the same, only their address are changed
    // test1.a -> 0x7ffd85579fc0 = "test1"    // test1.a -> 0x7ffd85579fc0 = "test2" 
    // test1.b -> 0x7ffd85579fc0 = "test1"    // test1.b -> 0x7ffd8557a058 = "test1"
        
        
    // test2.a -> 0x7ffd8557a058 = "test2"    // test2.a -> 0x7ffd8557a058 = "test1"
    // test2.b -> 0x7ffd8557a058 = "test2"    // test2.b -> 0x7ffd85579fc0 = "test2"
    
    
    

    ///// ------------------------------------------------------------------------ /////
    for i in 0..3{
        // the address of `a` will remain the same in each iteration
        // cause the allocated stack for this app inside 
        // the loop uses the same address or same location for a new variable 
        // that is built in each iteration.
        // //////////////////////////////////////////////////////////////////////////
        // if you want to move the location of a variable to another location  
        // inside the stack put the value of that variable inside another variable
        // by doing this the new defined variable has a new location and new address
        // inside the memory but with a same value as the old variable.
        let mut a: &i32 = &34;
        println!("address of a in memory is same as the old => {:p}", &a);
        a = &242354;
    }
    ///// ------------------------------------------------------------------------ /////






}
