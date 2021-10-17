





// NOTE - declarative macros are written using macro_rules!
// NOTE - procedural macros are custom derive: #[derive(CustomDerive)], attribute-like: #[CustomAttribute], and function-like: custom!(...)
// NOTE - procedural macros enables other prgrammers to use our trait on our own struct
// NOTE - Fn trait is an object safe trait, because of unknown size at compile time it needs to be inside the Box<dyn Trait_Name>
// NOTE - macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. 
// NOTE - function gets called at runtime and a trait needs to be implemented at compile time.
// NOTE - for those types specially concrete types like traits which don't have size at compile time means they are not bounded to Sized trait, we have to point them using a pointer like Box<dyn Trait> or &dyn Trait
// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes







#[macro_export]
macro_rules! user_data {
    ($user_id:expr, $token:expr) => {
        {

            use serde::{Deserialize, Serialize};


            #[derive(Debug, Serialize, Deserialize)]
            pub struct UserData{
                pub username: String,
                pub email: String,
                pub phone_number: String,
                pub wallet_address: String,
                pub balance: i32,
                pub sex: String,
                pub age: i16,
            }


            let coiniXerr_http_port = env::var("COINIXERR_HTTP_PORT").expect("⚠️ please set auth port in .env");
            let url = format!("http://localhost:{}/uniXerr/api/auth/user/get/{}", coiniXerr_http_port, $user_id);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .get(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<UserData>().await{
                                    Ok(resp) => {
                                        println!("[+] CURRENT SERVER TIME : {:?} | USER DATA FROM THE AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp);
                                        Ok(resp)
                                    },
                                    Err(e) => {
                                        println!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                println!("[!] CURRENT SERVER TIME : {:?} | AUTH MICROSERVICE SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    println!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}




#[macro_export]
macro_rules! authenticity {
    ($token:expr) => {
        {

            use serde::{Deserialize, Serialize};


            #[derive(Debug, Serialize, Deserialize)]
            struct ResponseBody{
                pub message: String,
                pub data: UserId, // NOTE - this is a string pretty json and we have to deserialize it into UserId struct
            }


            #[derive(Serialize, Deserialize, Debug)]
            struct UserId{
                pub user_id: i32,
            }


            let auth_port = env::var("AUTH_PORT").expect("⚠️ please set auth port in .env");
            let url = format!("http://localhost:{}/uniXerr/api/auth/check-token", auth_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .post(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<ResponseBody>().await{
                                    Ok(resp) => {
                                        println!("[+] CURRENT SERVER TIME : {:?} | RESPONSE MESSAGE FROM AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp.message);
                                        println!("[+] CURRENT SERVER TIME : {:?} | USER ID FROM THE AUTH MICROSERVICE SERVER : {:?}", chrono::Local::now().naive_local(), resp.data.user_id);
                                        Ok(resp.data.user_id)
                                    },
                                    Err(e) => {
                                        println!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                println!("[!] CURRENT SERVER TIME : {:?} | AUTH MICROSERVICE SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    println!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}

async fn cls_fn() {
    fn return_cls() -> Box<dyn FnOnce(i32) -> i32>{ //-- instances of FnOnce can be called, but might not be callable multiple times. Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once - FnOnce is a supertrait of FnMut
        Box::new(|x| x + 1)
    }    
    function_with_callback(return_cls()); // use .await to suspend the function execution for solving the future
}

async fn function_with_callback(cb: Box<dyn FnOnce(i32) -> i32>){
    cb(32);
}

fn mactrait(){

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