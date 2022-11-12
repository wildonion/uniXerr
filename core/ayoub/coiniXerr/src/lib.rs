


use std::mem::size_of_val;
use std::str;
use std::{slice, mem};
use std::collections::HashMap;
use std::{cmp::Eq, hash::Hash};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fs;
use std::{sync::{Arc, Mutex}, iter::Cloned};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;





















struct Cacher<U, T> where T: FnMut(U) -> U{
    closure: T,
    map: HashMap<U, U>,
    result: Option<U>,
}

impl<U, T> Cacher<U, T> where T: FnMut(U) -> U, U: Eq + Hash + Display + Copy{
    fn new(_: U, closure: T) -> Cacher<U, T>{
        Cacher{
            closure,
            map: HashMap::new(),
            result: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.result{
            Some(v) => v,
            None => {
                let result = self.map.entry(arg).or_insert((self.closure)(arg));
                self.result = Some(*result);
                *result
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut a_simple_var: u8 = 34;
	let callback = move |num: u32| -> u32 {
            a_simple_var = 56;
            println!("a simple var just moved here");
            println!("calculating slowly...");
            num+1 // we can add one to the num because this closure can mutate its environment vairable and it moves them to its scope!
        
      };
      
    let mut expensive_result = Cacher::new(34, callback);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", expensive_result.value(intensity)
            );
        }
    }
}


async fn cls_fn() {
    fn return_cls() -> Box<dyn FnOnce(i32) -> i32>{ //-- instances of FnOnce can be called, but might not be callable multiple times. Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once - FnOnce is a supertrait of FnMut
        Box::new(|x| x + 1)
    }    
    function_with_callback(return_cls()); // use .await to suspend the function execution for solving the future
}

async fn function_with_callback(cb: Box<dyn FnOnce(i32) -> i32>){
    cb(32);
    #[derive(Clone)]
    struct Request{
        pub user: u32,
        pub access: u32,
    }
    
    let res = run(move |req: Request|{
        println!("user {} has access {}", req.user, req.access);
    });
    
    
    fn run<C>(cls: C) where C: FnOnce(Request) + Send + 'static {
        let req = Request{user: 2893, access: 1};
        cls(req);
    }
}



//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////

pub async fn trash(){
	
	   {
            'outer: loop{ // outter labeled block 
                println!("this is the outer loop");
                'inner: loop{ // inner labeled block 
                    println!("this is the inner loop");
                    // break; // only the inner loop

                    break 'outer;
                }

                println!("this print will never be reached");
            }




            let mut counter = 0;
            let result = loop{
                counter += 1;
                if counter == 10{
                break counter * 2;
                }
            };
            println!("counter is {}", counter);
            println!("result is {}", result);

	    }
	

        // ------------------------------ testing trait Copy and Clone for closure ------------------------------
        let outside_num = 353;
            let callback = move |num: i32| {
                let got_outside_num = outside_num;
                let copy_of_num = num; //-- trait Copy is implemented for i32 thus has trait Clone so we don't need to clone it and we can also access it after it's moved into copy_of_num var 
            };

        // ------------------------------ testing trait Copy and Clone for i32 and String/str ------------------------------
        let name = String::from("wildonion");
        let name_slice = &name[0..3]; // pointing to an offset on the heap by borrowing some parts of the name String
        let anot_cop_of_slice = name_slice; // this is ok cause the Copy trait is implemented for &T which is &str in here
        // NOTE - we have still access to name_slice in here
        // ...
        // this is no ok cause name is on the heap with a saved reference to the heap on the stack also it doesn't implement Copy trait
        // the Clone trait is implemented for that because of double free pointer issue at runtime and the implementation of drop trait.
        // let another_name = name;
        // println!("name is droped {:?}", name); 
        let another_name = name.clone(); // we used the clone method here to copy the whole the reference on the stack and the whole data on the heap as well 
        let another_name = &name; // this is ok cause the Copy trait is implemented for &T which in our case is &String which is coerced &str or string slice which is saved somewhere in the memory(heap, stack or binary)
        let number: i32 = 3534;
        let another_number = number; // this is ok cause the number it's on the stack thus the drop trait is not implemented for that(still got the number even it's moved) so we can copy the whole number variable into another_number

        // ------------------------------ testing trait Copy and Clone for u8 and Vec<u8> ------------------------------
        // u8 implements Copy
        let x: u8 = 123;
        let y = x;
        // x can still be used
        println!("x={}, y={}", x, y);

        // Vec<u8> implements Clone, but not Copy
        let v: Vec<u8> = vec![1, 2, 3];
        let w = v.clone();
        //let w = v // This would *move* the value, rendering v unusable.

        // ------------------------------ testing trait Copy and Clone for structs ------------------------------
        #[derive(Debug, Clone, Copy)]
        pub struct PointCloneAndCopy {
            pub x: f64,
        }

        #[derive(Debug, Clone)]
        pub struct PointCloneOnly {
            pub x: f64,
        }

        fn test_copy_and_clone() {
            let p1 = PointCloneAndCopy { x: 0. };
            let p2 = p1; // because type has `Copy`, it gets copied automatically.
            println!("{:?} {:?}", p1, p2);
        }

        fn test_clone_only() {
            let p1 = PointCloneOnly { x: 0. };
            // let p2 = p1; // because type has no `Copy`, this is a move instead. to avoid moving we can clone the p1
            // println!("{:?} {:?}", p1, p2);
        }

	

        // reading image pixels or bytes which is utf8 and each pixel is between 0 up to 255
        // ...
        if let Ok(bytes) = fs::read("/home/wildonion/Pictures/test.jpg"){
            println!("image bytes >>>> {:?}", bytes);
        }

	

        'outer: for x in 0..5 {
            'inner: for y in 0..5 {
                println!("{},{}", x, y);
                if y == 3 {
                    break 'outer;
                }
            }
        }


    // 	::::::::::iterator for struct::::::::::
	struct Alternate {
	    state: i32,
	}

	impl Iterator for Alternate {
	    type Item = i32;

	    fn next(&mut self) -> Option<i32> {
            let val = self.state;
            self.state = self.state + 1;

            // if it's even, Some(i32), else None
            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
	    }
	}

	let mut iter = Alternate { state: 0 };

	// we can see our iterator going back and forth
	assert_eq!(iter.next(), Some(0));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), None);




    // =============================================================================================================================
    
    /*
	
        let mut my_name = "Pascal".to_string();
        my_name.push_str( " Precht");
        let last_name = &my_name[7..];
        
        
        
                         buffer
                        /    capacity
                       /    /   length
                      /    /   /
                    +–––+–––+–––+
        stack frame │ • │ 8 │ 6 │ <- my_name: String
                    +–│–+–––+–––+
                      │
                    [–│–––––––– capacity –––––––––––]
                      │
                    +–V–+–––+–––+–––+–––+–––+–––+–––+
               heap │ P │ a │ s │ c │ a │ l │   │   │
                    +–––+–––+–––+–––+–––+–––+–––+–––+

                    [––––––– length ––––––––]
                    
                    
                    
        Notice that last_name does not store capacity information on the stack. 
        This is because it’s just a reference to a slice of another String that manages its capacity. 
        The string slice, or str itself, is what’s considered ”unsized”. 
        Also, in practice string slices are always references so their type will always be &str instead of str.
                    
                    

                    my_name: String   last_name: &str
                    [––––––––––––]    [–––––––]
                    +–––+––––+––––+–––+–––+–––+
        stack frame │ • │ 16 │ 13 │   │ • │ 6 │ 
                    +–│–+––––+––––+–––+–│–+–––+
                      │                 │
                      │                 +–––––––––+
                      │                           │
                      │                           │
                      │                         [–│––––––– str –––––––––]
                    +–V–+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+–––+
               heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
                    +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
                    
                    

        string literals are a bit special. They are string slices that refer to “preallocated text” 
        that is stored in read-only memory as part of the executable. In other words, 
        it’s memory that ships with our program and doesn’t rely on buffers allocated in the heap.
        that said, there’s still an entry on the stack that points to that preallocated memory when the program is executed:

        
        let my_name = "Pascal Precht";
        
        
                    my_name: &str
                    [–––––––––––]
                      +–––+–––+
        stack frame   │ • │ 6 │ 
                      +–│–+–––+
                        │                 
                        +––+                
                            │
            preallocated  +–V–+–––+–––+–––+–––+–––+
            read-only     │ P │ a │ s │ c │ a │ l │
            memory        +–––+–––+–––+–––+–––+–––+
        
        
                    
			    
	*/
	let first_name = "Pascal"; // str - &str is a reference to String some where in the heap
    let last_name = "Precht".to_string(); // turn to String
    let another_last_name = String::from("Precht");
    greet(first_name); // first_name is &str by default
    greet(&last_name); // last_name is passed by reference
    greet(&another_last_name); // another_last_name is passed by reference

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

        
    let name = String::from("erfan"); // String
    let another_name = "another erfan"; // str
    let combined = name + &another_name;
    // name.push_str(&another_name); // name moved due to above operator
    println!("{}", combined);
    // println!("{}", name); // error - borrowed after move
    println!("{}", another_name);

    let sample_string = String::from("wildonion");
    let bytes = sample_string.bytes(); // turn a string into buffer (asccii)
    println!("[..] two first bytes of the string are : {}", &sample_string[0..2]); // byte indices
    println!("[..] the string bytes : {:?}", bytes);

    let text = "hello hello from wildonion here double again again wildonion";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0); // return a mutable reference inserted or the old value
        *count += 1; // updating the old value by dereferencing it, cause count is a mutable reference of the value 
    }

    println!("{:?}", map);

    
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
	



	// =============================================================================================================================
    // =============================================================================================================================
    // =============================================================================================================================
    //                                                 GENERIC AND LIFETIMES
    // we can return the reference from function with the lifetime of the passed in args or with a static ref
	// =============================================================================================================================
    // =============================================================================================================================
    // =============================================================================================================================
	// NOTE - generic types in function signature can be bounded to lifetimes and traits so we can use the lifetime to avoid having dangling pointer of the generic type in function body and traits to extend the type interface

	impl<'a, Pack: Interface + 'a> Into<Vec<u8>> for Unpack<'a, Pack, SIZE>{ //-- based on orphan rule we have to import the trait inside where the struct is or bound the instance of the struct into the Into trait in function calls - we wanto to return the T inside the wrapper thus we can implement the Into trait for the wrapper struct which will return the T from the wrapper field
	    fn into(self) -> Vec<u8> {
            self.arr.to_vec()
	    }
	}

    
    pub const WO: &str = "widonion";
	pub const SIZE: usize = 325;
	pub type Context<'a, Pack> = Unpack<'a, Pack, SIZE>; //-- Pack type will be bounded to Interface trait and 'l lifetime 
	pub struct Unpack<'l, T: Interface + 'l + Into<T>, const U: usize>{ //-- T is of type Pack struct which is bounded to 'l lifetime the Into and the Interface traits and U (constant generic) must be a constant usize type - Unpack takes a generic type of any kind which will be bounded to a trait and a lifetime but it must be referred to a field or be inside a PhantomData since T and the lifetime will be unused and reserved by no variables inside the ram
	    pub pack: T, //-- pack is a pointer or a reference and is pointing to T which is a generic type and bounded to a trait and a valid lifetime as long as the lifetime of the struct instance
	    pub arr: &'l [u8; U], //-- U is a constant usize
	}

	pub struct Pack; //-- we've allocated some space inside the stack for this struct when defining it which has long enough lifetime to initiate an instance from it using struct declaration and return a reference to that instance inside any function 
	pub trait Interface{}

	impl Interface for Pack{} //-- is required for return_box_trait(), return_impl_trait() and return_none_trait() functions in order to work

	fn return_none_trait<T>() -> () where T: Interface{ // NOTE - `T` type must be bound to Interface trait

	}

    // by returning the impl Interface for the type that is being returned we can call the trait methods on the type when we're calling the following method since we're returning acutally kinda an instance of the type
	fn return_impl_trait() -> impl Interface { // NOTE - returning impl Trait from function means we're implementing the trait for the object that is returning from the function regardless of its type that we're returning from the function cause compiler will detect the correct type in compile time and implement or bound the trait for that type
	    Pack {}
	}

	fn return_box_trait() -> Box<dyn Interface + 'static> { // NOTE - returning Box<dyn Trait> from function means we're returning a struct inside the Box which the trait has implemented for and since traits have unknown size at compile time we must put them inside the Box with a valid lifetime like 'static
	    Box::new(Pack {})
	}

	impl Pack{ ////// RETURN BY POINTER EXAMPLE //////


	    fn new() -> Self{


            let name = Some("wildonion".to_string());
            struct User{
                username: String,
                age: u8,
            }

            let user = User{
                username: match name{
                    Some(name) => name,
                    None => "".to_string(),
                },
                age: 26,
            };


            let User{username, age} = user; //-- unpacking struct

            let hello = "Здравствуйте";
            let s = &hello[0..2];
            // every index is the place of an element inside the ram which has 1 byte size which is taken by that element
            // in our case the first element takes 2 bytes thus the index 0 won't return 3 
            // cause place 0 and 1 inside the ram each takes 1 byte and the size of the
            // first element is two bytes thus &hello[0..2] which is index 0 and 1 both returns 3 
            // and we can't have string indices in rust due to this reason!


            ///////////////////////////////////////////// ENUM MATCH TEST
            #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
            enum Chie{
                Avali(u8),
                Dovomi(String),
                Sevomi,
            }


            let ine = Chie::Avali(12); //-- the Dovomi variant is never constructed cause we've used the first variant  

            match ine{
                Chie::Avali(value) if value == 23 => { //-- matching on the Avali arm if the value was only 23
                println!("u8 eeee");

                },
                Chie::Dovomi(value) if value == "wildonion".to_string() => { //-- matching on the Dovomi arm if the value was only "wildonion" string
                println!("stringeeee");
                },
                _ => {
                println!("none of them");
                }
            }

            // --------------- CODEC OPS ON ENUM ---------------
            let encoded = serde_json::to_vec(&Chie::Sevomi); ////// it'll print a vector of utf8 encoded JSON
            let decoded = serde_json::from_slice::<Chie>(&encoded.as_ref().unwrap()); //-- as_ref() returns a reference to the original type

            let encoded_borsh = Chie::Sevomi.try_to_vec().unwrap(); ////// it'll print 2 cause this the third offset in memory
            let decoded_borsh = Chie::try_from_slice(&encoded_borsh).unwrap();

            /////////////////////////////////////////////
            Pack{}
	    }

	    fn ref_struct(num_thread: &u8) -> &Pack{ //-- returning ref from function to a pre allocated data type (not inside the function) Pack struct in our case, is ok
            let instance = Pack::new(); //-- since new() method of the Pack struct will return a new instance of the struct which is allocated on the stack and is owned by the function thus we can't return a reference to it or as a borrowed type
            // &t //-- it's not ok to return a reference to `instance` since `instance` is a local variable which is owned by the current function and its lifetime is valid as long as the function is inside the stack and executing which means after executing the function its lifetime will be dropped
            let instance = &Pack{}; //-- since we're allocating nothing on the stack inside this function thus by creating the instance directly using the the Pack struct and without calling the new() method which is already lives in memory with long enough lifetime we can return a reference to the location of the instance of the pack from the function
            instance //-- it's ok to return a reference to `instance` since the instance does not allocate anything on the stack thus taking a reference to already allocated memory with long enough lifetime is ok since the allocated memory is happened in struct definition line
	    }

	    // NOTE - argument can also be &mut u8
	    pub fn ref_str_other_pointer_lifetime(status: &u8) -> &str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
            let name = "wildonion";
            name //-- name has a lifetime as valid as the passed in status argument lifetime from the caller scope 

	    }

	    // NOTE - first param can also be &mut self; a mutable reference to the instance and its fields
	    pub fn ref_to_str_other_self_lifetime(&self) -> &str{ //-- in this case we're good to return the pointer from the function or send a copy to the caller's space since we can use the lifetime of the first param which is &self which is a borrowed type (it's a shared reference means that other methods are using it in their scopes) of the instance and its fields (since we don't want to lose the lifetime of the created instance from the contract struct after calling each method) and have a valid lifetime (as long as the instance of the type is valid) which is generated from the caller scope by the compiler to return the pointer from the function
            let name = "wildonion";
            name //-- name has a lifetime as valid as the first param lifetime which is a borrowed type (it's a shared reference means that other methods are using it in their scopes) of the instance itself and its fields and will borrow the instance when we want to call the instance methods
	    }

	    // NOTE - 'a lifetime has generated from the caller scope by the compiler
	    pub fn ref_to_str_specific_lifetime<'a>(status: u8) -> &'a str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we've defined a valid lifetime for the pointer of the return type to return the pointer from the function which &'a str
            let name = "wildonion";
            name //-- name has a lifetime as valid as the generated lifetime from the caller scope by the compiler and will be valid as long as the caller scope is valid
	    }

        // NOTE - use 'static lifetime in order to be able to return &str from the function since rust doesn't allow to return reference by default unless the return type has a valid and defined lifetime
	    // NOTE - 'static lifetime will be valid as long as the whole lifetime of the caller scope (it can be the main function which depends on the whole lifetime of the app)
	    pub fn ref_to_str_static() -> &'static str{
            let name = "wildonion";
            name //-- name has static lifetime valid as long as the whol lifetime of the caller scope which can be the main function which will be valid as long as the main or the app is valid
	    }
		
	    //// ERROR - can't return a reference to heap allocated data structure from function due to their unknown size at compile time and they are temprary value
	    // pub fn ref_to_string<'s>() -> &'s String{
	    //     let name = &"wildonion".to_string();
	    //     name //-- ERROR - we can't return this or &"wildonion".to_string() since they are temporary value due to the fact that heap data structure's size are not specific at compile time and they are some kina a temporary value thus heap data structures can't be returned in their borrowed form from the function since their size are not specific at compile time therefore by taking a pointer to the location of them we might have dangling pointer later once their location gets dropped during the function lifetime body 
	    // }

	    pub fn ref_to_num<'n>() -> &'n i32{
            let num = 23;
            // &num //-- ERROR - we can't return this since the num is owned by the current function and returning the reference to the local variable which is owned by the function is denied
            &23 //-- we can return &23 since we did allocate nothing on the stack inside the function (which this can be done by creating a local variable inside the function) and we're just returning a pointer to the location of a number directly   

	    }

        pub const fn test(name: &String) -> &str{ // we can return &str in here sicne we're using the lifetime of the passed in param which is &String thus it's ok to use that reference (the reference to the passed in String) to return a &str (since its lifetime is valid as long as the passed in param is valid)
            WO // we must return const value from the constant function
        }

        pub fn run() -> impl std::future::Future<Output=u8>{ //-- implementing the Future trait for the return type of the function by doing this we have to return an async block from the function
            async move{ //-- returning an async block from the function
                26
            }

            // let res = run.await;
        }

        pub async fn _run() -> u8{ //-- above implementation is equivalent to this one 
            26

            // let res = run.await;
        }

	}
	



    // =============================================================================================================================
    // -SUCCESS-
    //  type Boxed = Box<dyn Trait + 'lifetime>;
    //  type Boxed = Box<&'a u64>;
    //  Generic : Trait + 'lifetime
    //  let var: &'lifetime Type;
    //  let var: &' Boxed = Box::new(||{});
    // -ERROR-
    //  Generic : Type + 'lifetime
    // >>> variable lifetime is how long the data it points to can be statically verified by the compiler to be valid at its current memory address


    trait Some{
        fn run(&self){}
    }
    impl Some for Boxed{
        fn run(&self){} 
    }

    type Boxy8<'a> = Box<&'a String>; //-- we have to store a pointer to the String inside this Box with a valid lifetime of 'a 
    type Boxed = Box<dyn FnMut() + 'static + Send + Sync>; //-- Boxed type can be shared between threads and .awaits safely - we must bound the type that wants to be a pointer or to be a referenced from a heap location like FnMut() closure to a valid lifetime like 'static
    let var: Boxed = Box::new(||{}); //-- since the Some trait is implemented for Boxed type we can call the run() method on the isntance of this type also the closure is bounded to a static lifetime

    fn call<'a>(a: &'a mut Boxed) -> Boxed where Boxed: Some + 'a { //-- in order to bind the Boxed to Some trait the Some trait must be implemented for the Boxed - can't bound a lifetime to a self-contained type means we can't have a: u32 + 'static
        // 'a lifetime might be shorter than static and describes how long the data it points to can be valid
        //...
        a.run(); //-- a is a mutable pointer of type a Boxed with 'a lifetime - since we have &self in the first param of the run() method for the Some trait we can call the run() method using dot notation
        Box::new(||{})
    }

    //// we can't remove the 'a lifetime from the passed in parameter since a pointer to name doesn't live long enough to return it from the function
    //// lifetime bounding is for those types that are a reference or a pointer to other types or are borrowing the ownership of a type due to this fact if T was bounded to a lifetime means it must be a pointer to a type (which is &'a T in our case in function param) with a valid lifetime 
    fn ref_me<'a, T>(name: &'a T) -> &'a T where T: ?Sized{ //-- since the trait `Sized` is not implemented for `str` or those types that have unknown size at compile time we've bounded the T to the 'a lifetime and ?Sized trait in order to pass unknown size types like str to the function
        let get_name: &'a T = &name; //-- since T is bounded to 'a lifetime in order to return a reference to type T we have to define the var to be of type &'a T
        get_name
    }

    let name = "narin";
    let res = ref_me::<&str>(&name); //-- we have to pass a reference to the name since the function param is of type &T which in our case will be &&str - the generic type can be str and &str since it's bounded to ?Sized trait
    // =============================================================================================================================
    

	
    enum MyResult<R, E>{
        Result(R),
        Err(E),
    }


    fn run(id: u8) -> MyResult<u8, i32>{
        MyResult::Result(1) 
        // if there was something went wrong we can return MyResult::Err(1);
        // ...
    } 

	
    





    
    // =============================================================================================================================
    //-------------------------- let else
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str_radix(count_str, 10) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item) // we can return them since their scopes didn't dropped when we're using let else
        
        // -------- using if let
        // --------------------------------
        // let (count_str, item) = match (it.next(), it.next()) {
        //     (Some(count_str), Some(item)) => (count_str, item),
        //     _ => panic!("Can't segment count item pair: '{s}'"),
        // };
        // let count = if let Ok(count) = u64::from_str(count_str) {
        //     count
        // } else {
        //     panic!("Can't parse integer: '{count_str}'");
        // };
        // --------------------------------
        
    }
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    // =============================================================================================================================




    

    
    // =============================================================================================================================
    // closure coding - trait must be inside Box or use with &dyn Trait if they want to be a type    
    pub struct Complex{
        pub callback: Box<dyn FnOnce(Option<String>) -> u8>,
        pub labeled_block: bool,
        pub callback_result: u8,
    }
    
    let comp = Complex{
        callback: Box::new(
            |_: Option<String>| 13
        ),
        labeled_block: 'block:{
            if 22 % 2 == 0{
                break 'block true; // it'll break the 'labeled block with a true return
            } else{
                break 'block false; // it'll break the 'labeled block with a false return
            }
        },
        callback_result: ( // building and calling the closure at the same time inside the struct field
            |_| 254
        )(Some("wildonion".to_string())),
    };

    let Complex{ 
        callback, 
        labeled_block,
        callback_result 
    } = comp else{ // the else part is not needed since the unpacking process will be matched always
        panic!("can't unpack");
    }; // struct unpacking

    pub async fn do_it<F>(callback: F) // callback is of type F where F is a closure which is Send Sync and have a valid static lifetime
        where F: FnOnce(Option<String>) -> u8 + Send + Sync + 'static{
        callback(Some("wildonion".to_string()));
    }


    let statement = |x: u32| Some(2);
    let Some(3) = statement(3) else{ // in else part there must be panic message
        panic!("the else part");
    };

    let res = { // res doesn't have any type
        ( // building and calling at the same time inside the res scope
            |x| async move{
                x
            }
        )(34).await; 
    };
    

    let callback = |_| Some(1); // |_| means that the param name can be anything  
    let (
        |callback| callback // the return type is the callback which is a closure
    ) = match callback(..){ // callback(..) means that it'll take anything as the range - when we're do a matching on the callback(..) means that by calling the callback(..) we should get a closure in its return type which this is not the case hence this code is unreachable 
        |_| Some(2) => |_| Some(3), // |_| Some(2) is the other syntax for calling the x closure - the or pattern: it can also be _ | Some(2) since _ means the value can be anything thus one of side can only be executed (either _ or Some(2))  
        |_| _ => unreachable!(), // |_| _ is the other syntax for calling the x closure - the or pattern: it can also be _ | _ since _ means the value can be anything thus one of side can only be executed (either _ or _)
    };
    // the return type of calling callback(..) is not a closure hence we can't do a matching on closures and because of that the code will be unreachabled
    assert!(matches!(callback(..), |_| Some(4))); // it'll be unreachable since the first arm of the match is not match with this 
    // =============================================================================================================================


}






//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////

pub fn mactrait(){


    
    /////////////////////////////////////////////////////////
    // gat example with lifetime
    trait BorrowArray<T> where Self: Send + Sized{
    
        type Array<'x, const N: usize> where Self: 'x;
    
        fn borrow_array<'a, const N: usize>(&'a mut self) -> Self::Array<'a, N>;
    }



    /////////////////////////////////////////////////////////
    // default type parameter example
    struct Valid(u8, u8);
    struct test<Output = Valid>{ // default type parameter
        name: Output,
        id: i32,
    }
    ///// ========================================================================
    trait SuperTrait: Give + See{}

    trait Give{
        fn take(&self) -> &i32;
    }
    
    trait See{
        fn what(&self) -> &String;
    }
    
    struct Who{
        a: i32,
        name: String,
    }
    
    impl See for Who{
        fn what(&self) -> &String{
            &self.name
        }
    }
    
    impl Give for Who{
        fn take(&self) -> &i32{
            &self.a // take() function doesn't own the `a` variable so we can return a reference to the type i32
        }
    }
    
    fn test_trait_0<T: Give + See>(item: &T){ // T is bounded to Give and See trait
        let val: &i32 = item.take();
        let name: &str = item.what().as_str();
        println!("the value of w.a is : {}", val);
        println!("the value of w.name is : {}", name);
    }
    
    fn test_trait_1(item: &(impl Give + See)){ // item is bounded to Give and See trait
        let val: &i32 = item.take();
        let name: &str = item.what().as_str();
        println!("the value of w.a is : {}", val);
        println!("the value of w.name is : {}", name);
    }
    
    fn test_trait_2(item: Box<dyn SuperTrait>){ // item is bounded to SuperTrait cause SuperTrait is an object safe trait
        let val: &i32 = item.take();
        let name: &str = item.what().as_str();
        println!("the value of w.a is : {}", val);
        println!("the value of w.name is : {}", name);
    }
    
    fn test_trait_3<T>(item: &T) where T: Give + See{ // T is bounded to Give and See trait
        let val: &i32 = item.take();
        let name: &str = item.what().as_str();
        println!("the value of w.a is : {}", val);
        println!("the value of w.name is : {}", name);
    }
    
    
    let w = Who{a: 64, name: "wildonion".to_string()};
    let p_to_w: *const Who = &w; // a const raw pointer to the Who truct
    println!("address of w is : {:p}", p_to_w);
    test_trait_0(&w);
    ///// ========================================================================

      
    // Used in a pattern.
    macro_rules! pat {
        ($i:ident) => (Some($i))
    }

    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
    }

    // Used in a type.
    macro_rules! Tuple {
        { $A:ty, $B:ty } => { ($A, $B) };
    }

    type N2 = Tuple!(i32, i32);


    // Used as an associated item.
    macro_rules! const_maker {
        ($t:ty, $v:tt) => { const CONST: $t = $v; };
    }
    trait T {
        const_maker!{i32, 7}
    }

    // Macro calls within macros.
    macro_rules! example {
        () => { println!("Macro call in a macro!") };
    }
    // Outer macro `example` is expanded, then inner macro `println` is expanded.
    example!();

}



//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////

pub fn unsafer(){





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
    let c_const_pointer = c_mut_pointer.cast_const(); // casting the c raw mutable pointer into a constant one
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


    
    // size of the &str is equals to its bytes and more less than the size of the String 
    // which is 24 bytes usize (8 bytes or 64 bits on 64 bits arch) for each of len, pointer and capacity 
    let name = "wildn🥲oion";
    let string_name = name.to_string();
    let byte_name = name.as_bytes();    
    println!("size name -> {:#?}", size_of_val(name));
    println!("size string name -> {:#?}", size_of_val(&string_name));
    println!("size byte name -> {:#?}", size_of_val(byte_name));
    
    
    
    
    
    ///// ------------------------------------------------------------------------ /////
    // python like inline swapping
    ///// ------------------------------------------------------------------------ /////
    let var_a = 32;
    let var_b = 535;
    let mut a = &var_a; //-- a is a pointer with a valid lifetime to the location of var_a type and it contains the address and the data of that type
    let mut b = &var_b; //-- b is a pointer with a valid lifetime to the location of var_b type and it contains the address and the data of that type
    ///// inline swapping : a, b = b, a -> a = b; b = a and under the hood : a = &var_b, b = &var_a
    a = &var_b; //-- pointer of var_a must points to the location of var_b and after that it can access the data inside var_b 
    b = &var_a; //-- pointer of var_b must points to the location of var_a and after that it can access the data inside var_a




    ///// ------------------------------------------------------------------------ /////
    //          encoding an instance into utf8 using unsafe from_raw_parts
    ///// ------------------------------------------------------------------------ /////
    // NOTE - unsafe block for serializing doesn't work like serde due to the need of padding and memory mapping operations which borsh and serde are handling                            
    // NOTE - encoding or serializing process is converting struct object into utf8 bytes
    // NOTE - decoding or deserializing process is converting utf8 bytes into the struct object
    // NOTE - from_raw_parts() forms a slice or &[u8] from the pointer and the length and mutually into_raw_parts() returns the raw pointer to the underlying data, the length of the vector (in elements), and the allocated capacity of the data (in elements)
    // let signed_transaction_serialized_into_bytes: &[u8] = unsafe { //-- encoding process of new transaction by building the &[u8] using raw parts of the struct - serializing a new transaction struct into &[u8] bytes
    //     //-- converting a const raw pointer of an object and its length into the &[u8], the len argument is the number of elements, not the number of bytes
    //     //-- the total size of the generated &[u8] is the number of elements (each one has 1 byte size) * mem::size_of::<Transaction>() and it must be smaller than isize::MAX
    //     //-- here number of elements or the len for a struct is the size of the total struct which is mem::size_of::<Transaction>()
    //     slice::from_raw_parts(deserialized_transaction_borsh as *const Transaction as *const u8, mem::size_of::<Transaction>()) //-- it'll form a slice from the pointer to the struct and the total size of the struct which is the number of elements inside the constructed &[u8] array; means number of elements in constructing a &[u8] from a struct is the total size of the struct allocated in the memory
    // };
    


}
