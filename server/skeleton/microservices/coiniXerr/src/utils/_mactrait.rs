



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
        let req = Request{user: 2893};
        cls(req);
    }
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