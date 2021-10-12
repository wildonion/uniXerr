








pub struct MetaData{}
pub fn some_method(){}

async fn cls_fn() {
    fn return_cls() -> Box<dyn FnOnce(i32) -> i32>{
        Box::new(|x| x + 1)
    }    
    function_with_callback(return_cls()); // use .await to suspend the execution for solving the future
}

async fn function_with_callback(cb: Box<dyn FnOnce(i32) -> i32>){
    cb(32);
}

