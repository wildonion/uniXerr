




pub struct MetaData{}
pub fn some_method(){}
async fn cls_fn() {
    
    println!("called `function()`");
    fn return_cls() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    let cls = return_cls()(32);
    let another_cls = return_cls();
    another_cls(32);
    function_with_callback(another_cls); // use .await to suspend the execution for solving the future
    
}

async fn function_with_callback(cb: Box<dyn Fn(i32) -> i32>) {
    cb(32);
}

