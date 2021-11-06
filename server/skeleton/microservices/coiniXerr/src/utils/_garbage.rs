use std::str;
use std::collections::HashMap;
use std::{cmp::Eq, hash::Hash};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fs;



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
    let a_simple_var: u8 = 34;
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


fn trash(){
	
	   {
            'outer: loop{
                println!("this is the outer loop");
                'inner: loop{
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
            let p2 = p1; // because type has no `Copy`, this is a move instead. to avoid moving we can clone the p1
            println!("{:?} {:?}", p1, p2);
        }

	

        // reading image pixels or bytes
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

    // =============================================================================================================================
    
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);


}