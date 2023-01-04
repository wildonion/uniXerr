




use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case









//// for sharing data between threads safeyly the data must be inside Arc<Mutex<T>> and also must be bounded to the Send + Sync + 'static lifetime or have a valid lifetime across threads, awaits and other scopes when we move them between threads using tokio job queue channels
//// future objects must be Send and static and types that must be shared between threads must be send sync and static 
//// Box<dyn Future<Output=Result<u8, 8u>> + Send + Sync + 'static> means this future can be sharead acorss threads and .awaits safely
type Callback = Box<dyn 'static + FnMut(hyper::Request<hyper::Body>, hyper::http::response::Builder) -> CallbackResponse>; //// capturing by mut T - the closure inside the Box is valid as long as the Callback is valid due to the 'static lifetime and will never become invalid until the variable that has the Callback type drop
type CallbackResponse = Box<dyn Future<Output=GenericResult<hyper::Response<hyper::Body>, hyper::Error>> + Send + Sync + 'static>; //// CallbackResponse is a future object which will be returned by the closure and has bounded to Send to move across threads and .awaits - the future inside the Box is valid as long as the CallbackResponse is valid due to the 'static lifetime and will never become invalid until the variable that has the CallbackResponse type drop
type SafeShareAsync = Arc<Mutex<std::pin::Pin<Box<dyn Future<Output=u8> + Send + Sync + 'static>>>>; //// this type is a future object which has pinned to the ram inside a Box pointer and can be shared between thread safely also it can be mutated by threads - pinning the Boxed future object into the ram to prevent from being moved (cause rust don't have gc and each type will be dropped once it goes out of its scope) since that future object must be valid across scopes and in the entire lifetime of the app until we await on it 
type SafeShareClosure = Arc<Mutex<Box<dyn FnOnce(hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> + Send + Sync + 'static>>>; //// this type is safe and sendable to share between threads also it can be mutated by a thread using a mutex guard; we have to use the &dyn keyword or put them inside the Box<dyn> for traits if we want to treat them as a type since they have no sepecific size at compile time thus they must be referenced by the &dyn or the Box<dyn> 











// ------------------------------ heavy computational calculation using async and rayon multithreading simd
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
pub fn forward1(x_train: Arc<Vec<Vec<f64>>>) -> f64{ //// without &mut self would be an associated function not a method
    

    let mat = x_train; //// the data that we want to do some heavy computational on it
    let NJOBS: usize = mat.len(); //// number of tasks of the process (incoming x_train matrix) to share each one between threads inside the pool
    let (sender, receiver) = heavy_mpsc::<f64>();
    let mut mult_of_all_sum = 1.0;
    let mut rayon_workers = Vec::new();

    
    /* ----------------------------------------------------------------------------------- */
    /* ------------------------ JOBQ CHANNEL + RAYON THREADPOOL -------------------------- */
    /* ----------------------------------------------------------------------------------- */
    let future_res = async { //// we can also use tokio::spawn() to run the async task in the background using tokio event loop and green threads
        
        for i in 0..NJOBS{ //// iterating through all the jobs of the process - this can be an infinite loop like waiting for a tcp connection
            
            let cloned_sender = sender.clone(); //// cloning the sender since it's multiple producer and Clone trait is implemented for that
            let cloned_mat = mat.clone(); //// Vec is a heap data structure thus we must clone it
            rayon_workers.push( //// pushing a rayon::spawn() handler into the vector - rayon::spawn() will solve the task inside its threadpool safely since it uses mpsc as the message passing protocol between threads to avoid being in deadlock and race condition situations 
                rayon::spawn(move || {
                    let sum_cols = cloned_mat[0][i] + cloned_mat[1][i] + cloned_mat[2][i];
                    cloned_sender.send(sum_cols).unwrap(); //// sending the data to the downside of the channel 
                })
            );

            info!("job {} finished!", i);    
        
        }

        rayon::spawn(move ||{
            while let Ok(data) = receiver.recv(){ //// receiving the data from the asyncly inside rayon worker threadpool 
                mult_of_all_sum *= data;
            }
        });

        mult_of_all_sum

    };
    /* ----------------------------------------------------------------------------------- */


    /* ---------------------------------------------------------------------------- */
    /* ------------------------ RAYON PARALLEL ITERATION -------------------------- */
    /* ----------------------------------------------------------------------------- */
    mat
        .par_iter() //// iterate over the mat parallely using based on simd pattern
        .for_each(|row| {
            println!("row is {:?}", row) 
        });
    /* ---------------------------------------------------------------------------- */
    

    let res = block_on(future_res); //// will block the current thread to run the future to completion
    // let res = future_res.await; //// .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
    // let res = join!(future_res); //// join! only allowed inside `async` functions and blocks and is like .await but can wait for multiple futures concurrently
    println!("multiplication cols sum {:?}", res);
    let loss = 0.3535;
    loss


}




// ------------------------------ heavy computational calculation using async and multithreading design patterns
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
pub fn forward(x_train: Arc<Vec<Vec<f64>>>) -> f64{ //// without &mut self would be an associated function not a method
    

    /*  

    

        ➔ The three causes of data races
            • Two or more pointers access the same data at the same time.
            • At least one of the pointers is being used to write to the data.
            • There’s no mechanism being used to synchronize access to the data
        ➔ The three rules of ownership
            • Each value in Rust has a variable that’s called its owner.
            • There can only be one owner at a time.
            • When the owner goes out of scope, the value will be dropped.
        ➔ The two rules of references
            • At any given time, you can have either one mutable reference or any number of immutable references.
            • References must always be valid.


        ➔ types bounded to Sync and Send:
            Arc
            Mutex
            RwLock
        ➔ types not bounded to Sync and Send:
            Rc
            RefCell
            Cell



       ➔ in order to move the T between threads T must be bounded to Send and if it wasn't bound to Sync we have to clone it to move it between threads 
          and if it wasn't bound to Clone trait we must put it inside the Arc and to change it inside other thread we must put the Arc-ed type inside 
          the Mutex like mutating the Receiver<T> content inside other threads which must be in form Arc<Mutex<Receiver<T>>> since the Receiver is 
          not bounded to Sync (cause &Receiver is not bouned to Send cause there is no clone trait implemented for the Receiver thus it can't be copy 
          thus we can't have &Receiver and can't be clone) therefore we can't move it between threads due to the fact that the type T (Receiver in our case) 
          must be used by only one consumer or thread at a time by blocking threads waiting for the lock to become available.
       ➔ based on the rust ownership and borrowing rule which removes the need of garabe collection rule we can have multiple immutable references to a type but only one mutable pointer at a time must be exists
          and due to this rule we can have a safe concurrency in such a way that multiple threads can be the owner of a type to read its content but only on of them
          can mutate the content of the type (like reading from the receiver which is a mutable operation) therefore we can have mpsc message passing channel based on this rule
          the sender of the channel is Send but not Sync and can be cloned to send data between threads for reading (multiple producer) but the receiver is not Send and Sync and can't be cloned 
          since it can't be move or transfer to other threads cause the reading process from the receiver is a mutable operation and we can't have multiple mutable operations or references 
          at the same time with multiple threads thus only one thread at a time can do this job.
       ➔ since the reading from the receiver is a mutable process and it can't be cloned thus in order to read from it inside multiple thread we have to write 
          the receiver in form of Arc<Mutex<Receiver<T>>> and lock on the receiver to read the content of what we're receiving.   



        -- shareable rules : data which are Send + Sync + 'static must be share and trasferred between threads using mpsc channel
        -- in rust everything is all about having a type and size thus everything must be generic and we have to borrowing them using & to share them between other scopes like threads and functions a shareable data means we're sharing its references which it can be copied or cloned and safe to Send and mutate between threads
        -- Arc will be used instead of Rc in multi threading to avoid data races and is Send means all its references can be shared between threads and is an atomic reference to a type
        -- if &T is Send then T can be also Sync thus in order to share a data between threads safely the type must be bounded to Send + Sync + 'static means it must be cloneable or shareable between threads means we can simply borrow it to move it between threads and Sync with other threads to avoid mutating it by multiple threads at the same time
        -- if there is no possibility of undefined behavior like data races when passing &T between threads means &T must be Send then T is alos Sync and &mut T is Sync if T is Sync
        -- data which is utf8 encoded using borsh or serde to share a reference of it (by borrowing it) between threads using mpsc must be : Send + Sync + 'static + Unpin means it must be inside Arc<Mutex<T>>
        -- if a type is not Send + Sync it means we can't move its references between threads safely and we have to put it inside Arc since &Arc<T> is Send thus Arc<T> is Sync thus the type inside the Arc can be sent between threads safely
        -- a type might be mutated by other threads thus we have to put it inside Mutex or RwLock to avoid data races means that only one thread can mutate the state of a type
        -- instead of moving types into the thread we can borrow them using Arc to have them outside the threads
        -- based on mpsc rust has defined the rule which says multiple immutable can be inside a scope but only one of them can be mutable
        -- in order to share data (T must have shareable rules) between threads we have to use mpsc channel 
        -- Send is the access of sharing between threads, Sync is safe to transfer and static means the type must have static lifetime across threads and .awaits
        -- share data between routers and threads using .data() of routerify Router and to do that the passed in closure of the thread must be static + send and sync to send between threads safely we can't just simply borrow the data using & to pass them between threads (since the race condition might be happened) since the type must be send + sync and 'static to be shared between threads safely if it's not send and sync we can put it inside the Arc<Mutex<T>> to make it cloneable and borrow it mutably to mutate its content by locking on it inside a free thread, if other threads don't want to mutate it we can just put it inside Arc<T> to be just cloneable 
        -- share reference or share access means multiple threads can read and access a resource or a type but only on of them can mutate it and the channel for this task is the mpsc
        -- the type that wants to be sent between threads must be Send but not Sync necessarily like sender which is not Sync but it's Send and receiver is not Sync and Send
        -- it's better not to pass the receiver between threads due to the rule of mpsc since we can't mutate a data simply inside a thread while others are reading it we have to block that thread that wants to mutate the type using Mutex
        -- passing data between threads is done using mpsc channel which multiple threads can own a resource immutable referece but only on of them can mutate that resource at a time
        -- to pass data between thread the type must cloneable and sender must be cloned since inside a thread all env vars before that are moved to its scope.
        -- in order to mutate a type inside a thread the type must be inside Mutex since the receiver can't be referenced by multiple threads at a time thus is a single consumer means that it can't be cloned and moved to other threads 
        -- Send means that a type is safe to move from one thread to another
        -- Sync makes the type safe (&T nmust be Send) to access shared reference across threads at the same time 
        -- Clone trait is implemented for the mpsc sender and is bounded to Send but not Sync and due to this reason we have to clone it in order we can have it in multiple threads (multi producer)
        -- Clone trait is not implemented for the mpsc receiver and we must put it inside Arc also is not Sync means it can't be referenced by multiple threads at the same time due to the fact that only one thread can mutate its content at a time (single consumer) thus we have to put it inside a Mutex
        -- in order to pass the receiver between threads safely and mutate its content by locking on it we must put the receiver inside Arc and Mutex like let shareable_receiver = Arc<Mutex<Receiver<T>>> then clone it using Arc::new(&shareable_receiver) or shareable_receiver.clone()
        -- recv() will block the current thread if there are no messages available


    */
    
    
    let mat = x_train; //// the data that we want to do some heavy computational on it
    let NTHREADS: usize = 4; //// number of threads inside the pool
    let NJOBS: usize = mat.len(); //// number of tasks of the process (incoming x_train matrix) to share each one between threads inside the pool
    let (sender, receiver) = heavy_mpsc::<f64>();


    let mutexed_receiver = Mutex::new(receiver); //// putting the &receiver in its borrowed form inside the Mutex to get its data by locking on it inside other threads since the Sync is not implemented for the receiver and in order to get its data inside other threads we have to make cloneable using Arc and some kina syncable using Mutext
    let arced_mutexed_receiver = Arc::new(mutexed_receiver); //// putting the &mutexed_receiver in its borrowed form inside the Arc
    //// we can't use env::var() to make a rust constant type since by
    //// all vars inside the env file will be loaded at runtime into the ram
    //// not at compile time also const does not only mean a constant, it means a compile-time constant, 
    //// a value determined at compile-time and inscribed in the read-only memory of the program.
    //
    //// static means a global variable, with a lifetime that will span the entire program,
    //// and must be initialized from a constant, in order to be available from the start of the program
    //// otherwise the compiler says: attempt to use a non-constant value in a constant.
    //
    //// for statics to be mutable in rust, you need to wrap them in 
    //// a Mutex to follow rust's whole thing of guaranteeing thread safety
    //// by doing this that static type will be Sync-ed which avoids concurrent modifications
    //// or put a mut keyword before the name which is required to be Sync to avoid concurrent modifications
    //// otherwise we have to use unsafe block.
    pub static mut MULT_OF_ALL_SUM: f64 = 1.0;
    let mut mult_of_all_sum: &'static f64 = &1.0;
    let mut rayon_workers = Vec::new();

    
    let future_res = async { //// we can also use tokio::spawn() to run the async task in the background using tokio event loop and green threads
        
        for i in 0..NJOBS{ //// iterating through all the jobs of the process - this can be an infinite loop like waiting for a tcp connection
            let cloned_arced_mutexed_receiver = Arc::clone(&arced_mutexed_receiver); //// in order to move the receiver between threads we must have it in Arc<Mutex<Receiver<T>>> form and clone the &arced_mutexed_receiver which is the borrowed form of the arced and mutexed of the receiver or we can have arced_mutexed_receiver.clone()
            let cloned_sender = sender.clone(); //// cloning the sender since it's multiple producer and Clone trait is implemented for that
            let cloned_mat = mat.clone();
            rayon_workers.push( //// pushing a rayon::spawn() handler into the vector - rayon::spawn() will solve the task inside its threadpool safely since it uses mpsc as the message passing protocol between threads to avoid being in deadlock and race condition situations 
                rayon::spawn(move || {
                    let sum_cols = cloned_mat[0][i] + cloned_mat[1][i] + cloned_mat[2][i];
                    cloned_sender.send(sum_cols).unwrap();
                })
            );
            
            info!("job {} finished!", i);
            
            /* ----------------------------------------------------------------------------------- */
            /* -------- receiving inside another native and tokio threads inside the loop -------- */ 
            /* ----------------------------------------------------------------------------------- */
            // thread::spawn(move || loop{});
            thread::spawn(|| async move{ //// the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
                tokio::spawn(async move{ //// spawning async task to solve it on the background using tokio green threads based on its event loop model
                    while let Ok(data) = cloned_arced_mutexed_receiver.lock().unwrap().recv(){
                        /* 
                            -----------------------------------------------------------------------------------
                                          --- the reason that MULT_OF_ALL_SUM must be static ---

                            in this situation we shouldn't mutate any type inside here if the type is not
                            static and doesn't have a valid lifetime across threads.

                            one of the most popular cases of a T: 'static bound is std::thread::spawn
                            the reason there is that the closure and its return value need to be sent between threads, 
                            escaping their call-stack which is why they cannot contain any non-'static references 
                            since these could become invalidated and no longer available in the other thread in the mean-time.

                            if we don't want to send the data from a thread to another one we have to make static to have 
                            valid lifetime across threads also mutating the static types directly is unsafe!

                            due to the single consumer rule only on thread can mutate the received job or the task or the data
                            at a time in order to prevent data racing we've put the Arced (since it's not cloneable due to the single consumer rule) 
                            receiver inside the Mutex to lock on it and change the content of what it has received cause we want 
                            to mutate the data of the receiver inside other threads.
                            -----------------------------------------------------------------------------------
                        */
                        
                        // *mult_of_all_sum *= data; // ERROR - can't deref the mult_of_all_sum since its deref doesn't live long enough since its reference or its borrowed type is static not its deref 
                        unsafe {MULT_OF_ALL_SUM *= data;} //// mutating the data that we've just received - mutating static types needs unsafe block
                    }
               });
            });
            /* ----------------------------------------------------------------------------------- */
        }
        
        unsafe{MULT_OF_ALL_SUM} //// since MULT_OF_ALL_SUM has mutated thus we have to return it from an unsafe block

    };
    

    let res = block_on(future_res); //// will block the current thread to run the future to completion
    // let res = future_res.await; //// .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress and log the result later whenever the future has completed
    // let res = join!(future_res); //// join! only allowed inside `async` functions and blocks and is like .await but can wait for multiple futures concurrently
    println!("multiplication cols sum {:?}", res);
    let loss = 0.3535;
    loss

    
}







// ------------------------------ simd (vectorization) using mpsc channel + tokio + native thread
// ------------------------------------------------------------------------------------------------------
// https://stackoverflow.com/questions/35091979/why-is-vectorization-faster-in-general-than-loops
// ------------------------------------------------------------------------------------------------------
// simd ops means that dividing the vector of events or tasks into multiple parts in such a way that all parts will be executed concurrently
// NOTE - we could also use rayon simd its parallel iteration 
// ------------------------------------------------------------------------------------------------------


pub async fn simd<F>(number: u32, ops: F) -> Result<u32, String> where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //// in order to move the F between threads it must be bounded to Send trait
    
    
    // simd on a 32 bits number means solving 4 packs or operations like multiplication of 8 bits or 4 bytes in parallel
    // simd on a 256 bits number means solving 4 packs or operations like multiplication of 64 bits (4 * 8 bytes = 256 bits) or 8 packs of 32 bits (8 * 4 bytes = 256 bits) 
    let threads = 4; //// the total number of all packs or chunks containing 8 bits is 4 cause our number is of type u32
    let (sender, receiver) = std_mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //// network bytes which is in form utf8 or big endian bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
    let mut index = 0;
    


    while index < big_end_bytes.len(){
        
        info!("chunk {:?} in utf8 format -> [{:?}] at time {:?}", index, big_end_bytes[index], chrono::Local::now().naive_local());
        let cloned_sender = sender.clone();
        let cloned_ops = ops.clone();
        tokio::spawn(async move{ //// spawning async task to solve it on the background using tokio green threads based on its event loop model
            thread::spawn(move || async move{ //// the return body of the closure is async block means it'll return a future object (trait Future has implemented for that) with type either () or a especific type and for solving it we have to be inside an async function - in order to capture the variables before spawning scope we have to use move keyword before ||
                let new_chunk = cloned_ops(big_end_bytes[index]);
                info!("\tsender-channel---(chunk {:?})---receiver-channel at time {:?} ", index, chrono::Local::now().naive_local());
                cloned_sender.send(new_chunk).unwrap();
            });
        });
        index+=1

    }

    
    
    info!("collecting all chunks received from the receiver at time {:?}", chrono::Local::now().naive_local());
    let bytes: Vec<u8> = receiver.iter().take(threads).collect(); //// collecting 4 packs of 8 bits to gather all incoming chunks from the channel
    info!("collected bytes -> {:?} at time {:?}", bytes, chrono::Local::now().naive_local());
    
    

    
    let boxed_array = self::into_box_slice(&bytes).await.unwrap(); //// converting &Vec<u8> to [u8] with a fixed size
    let result = *boxed_array; //// dereferencing the box pointer to get the value inside of it 
    let final_res = u32::from_be_bytes(result); //// will create a u32 number from 4 pack of 8 bits - from_be_bytes() method creates a native endian integer value from its representation as a byte array in big endian
    Ok(final_res) //// the final results might be different from the input due to the time takes to send the each chunks through the channel and receive them from the receiver thus the order of chunks will not be the same as the input




}






// ------------------------------ into box method
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
pub async fn into_box_slice(u8_vector: &Vec<u8>) -> Result<Box<[u8; 4]>, String>{ //// the return type of this function is either a Box of [u8] slice with 4 bytes (32 bits) or a String of the error
    let to_owned_vec = u8_vector.to_owned(); //// creating owned vector from borrowed vector by cloning to call into_boxed_slice() method on the vector
    let boxed_slice = to_owned_vec.into_boxed_slice(); //// converting the collected bytes into a Box slice or array of utf8 bytes - we put it inside the Box cause the size of [u8] is not known at compile time
    let boxed_array: Box<[u8; 4]> = match boxed_slice.try_into() { //// Boxing u8 with size 4 cause our input number is 32 bits which is 4 packs of 8 bits
        Ok(arr) => return Ok(arr), //// returning a Box of 4 u8 slice or 4 packs of 8 bits
        Err(o) => return Err(format!("vector length must be 4 but it's {}", o.len())),
    };
}






// ------------------------------ String to static str
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
pub fn string_to_static_str(s: String) -> &'static str { //// the lifetime of the return str is static and is valid as long as the entire lifetime of the app
    Box::leak(s.into_boxed_str())
}










// ------------------------------ coiniXerr Db ORM 
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------

pub mod DbORM{

    use super::*; //// loading all the super modules in here which are all loaded module into the utils.rs

    //// following trait is not object safe trait since we're 
    //// returning Self from each method and also we have the 
    //// &self in first parameter.
    #[async_trait]
    pub trait StorageModel{ //// StorageModel trait for mongo structures based on ORM pattern
        
        type AppStorage; //// this is the app storage GAT which must be used to make queries

        //// Self referes to the implementor which is the structure contains the fields related to the db model 
        //// that this orm has implemented for we must return the structure itself in each method call to be able
        //// to call the struct methods later.
        //
        //// every trait method that returns Self would have to also specify where Self: Sized  
        //// since we don't know the actual size of the Self or the structure in here cause 
        //// it'll be specified at runtime thus we have to tell the compiler
        //// that hey remember that the Self is a Sized one compile it!
        
        async fn save(&self) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>;
        async fn fetch(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized;
        async fn filter(&self, query: &str) -> Result<Self, mongodb::error::Error> where Self: Sized;

    }


}









// ------------------------------ utility methods and structs
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
pub fn gen_chars(size: u32) -> String{
    let mut rng = rand::thread_rng();
    (0..size).map(|_|{
        char::from_u32(rng.gen_range(65..91)).unwrap() // generating a char from the random output of type u32 using from_u32() method
    }).collect()
}


pub fn gen_random_number(from: u32, to: u32) -> u32{
    let mut rng = rand::thread_rng(); // we can't share this between threads and across .awaits
    rng.gen_range(from..to)
} 




// TODO - implement custom error for the coiniXerr app
// ...
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct AppError{
    pub code: u16,
    pub msg: String,
}


impl AppError{

    pub async fn show(){
        (
            || async move{
                34
            }
        )().await;
    }

}



impl fmt::Display for AppError{ // implementing the formatter Display trait for the AppError struct to log every instance of it in a specific format to the console using println!() macro
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{ // self refers to the instance of the AppError
        f.write_fmt( // write the TheDoError args into a formatter which is `f` in here
            format_args!( // using format_args!() macro to print the following to the console when we log an instance of the AppError 
                "ERROR:{:#?}",
                &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() // return format error if there was any error in mapping each field
            )   
        )
    }

}










/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////             sending fake transaction to the coiniXerr rpc server  
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

pub async fn rpc_tx_emulatro() -> (){

}






/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////             sending fake transaction to the coiniXerr tcp server  
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//// can't use .await inside the pool.execute(move || {}); 
//// since it's a sync task scheduler and unlike tokio it's body won't return an async block future object 

pub async fn tcp_tx_emulator() -> (){
    
    let mut time = 0;
    let tcp_host = env::var("HOST").expect("⚠️ please set host in .env");
    let tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let ip_port = format!("{}:{}", tcp_host, tcp_port);
    let sleep = Duration::from_secs("3".to_string().parse::<u64>().unwrap());

    loop{ //// simulating a transaction emulator by sending infinite tx to the coiniXerr tcp server
        
        time+=1;
        let ip_port = ip_port.clone();
        tokio::spawn(async move{ //// an async block or future object is the param of the tokio::spawn()
            match TcpStream::connect(ip_port.as_str()).await{
                Ok(mut stream) => { //// stream must be muatble in order to write on it

                    info!("🪙 sending transaction {}", time);
                    let random_tx = Transaction::default(); //// creating a default transaction
                    let encoded_tx = random_tx.try_to_vec().unwrap(); //// encoding using borsh; we can convert a Vec<u8> to &[u8] by taking a reference to it since &[u8] which will be on the stack is an slice of the Vec<u8> which is inside the heap 
                    stream.write_all(&encoded_tx).await.unwrap(); //// writing the buffer, the encoded transaction, into the stream to send back to the server

                },
                Err(e) => {
                    error!("😕 : {}", e);
                }
            }
        });  

        thread::sleep(sleep);
    }
    
}





/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////             sending fake transaction to the coiniXerr udp server  
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
// since the UDP protocol doesn't have any capabilities to detect a broken connection 
// the server needs to be run first, otherwise the client will block forever.
    
pub async fn udp_tx_emulator() -> (){
        
    let mut time = 0;
    let tcp_host = env::var("HOST").expect("⚠️ please set host in .env");
    let tcp_port = env::var("COINIXERR_UDP_PORT").expect("⚠️ please set coiniXerr udp port in .env");
    let ip_port = format!("{}:{}", tcp_host, tcp_port);
    let sleep = Duration::from_secs("3".to_string().parse::<u64>().unwrap());


    loop{ //// simulating a transaction emulator by sending infinite tx to the coiniXerr udp server
        
        time+=1;
        let ip_port = ip_port.clone();
        tokio::spawn(async move{
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap(); // binding to any available address and any port selected by the os for outbound packets
            match socket.connect(ip_port.clone().as_str()).await{ //// let this user socket connect to the passed in address
                Ok(_) => {

                    info!("🪙 sending transaction {}", time);
                    let random_tx = Transaction::default(); //// creating a default transaction
                    let encoded_tx = random_tx.try_to_vec().unwrap(); //// encoding using borsh; we can convert a Vec<u8> to &[u8] by taking a reference to it since &[u8] which will be on the stack is an slice of the Vec<u8> which is inside the heap 
                    socket.send(&encoded_tx).await.unwrap(); //// send the buffer, the encoded transaction to the remote address that this socket is connected to or we can send to another address 

                },
                Err(e) => eprintln!(": {} at {}", e, chrono::Local::now()),
            }
        });

        thread::sleep(sleep);

    }        

}













// ------------------------------ data collision prevention structures 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
/*
    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

        

        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"}
        
        when initializing a data structure we have to make sure to give it a unique id, otherwise, it could point to other structure's key-value references;
        above collections will be collided with each other inside the memory since they share the same storage for their keys and have same keys
        to fix this we can allocate a unique storage key for each collection like binding a unique key for each entry that comes into the collection
        and that unique storage key must be built from a utf8 bytes encoded unique indentifire like an enum variant:

        NOTE - the reason we're using enum is because of by encoding each variant using borsh we'll get a unique vector of utf8 bytes array
        
        #[derive(BorshSerialize, BorshDeserialize)]
        pub enum CollectStorageKey{
            CollectionOne,
            CollectionTwo,
        }

        collection 1 storage key : 0 ---- built from the utf8 bytes encoded CollectionOne enum variant (CollectStorageKey::CollectionOne.try_to_vec().unwrap())
        collection 2 storage key : 1 ---- built from the utf8 bytes encoded CollectionTwo enum variant (CollectStorageKey::CollectionTwo.try_to_vec().unwrap())
        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"} -> put all the keys inside the created storage key for the first collection like: {0: [1, 2, 3]} or as a unique prefix for the keys: {01: "value64", 02: "value53", 03: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"} -> put all the keys inside the created storage key for the second collection like: {1: [1, 2, 3]} or as a unique prefix for the keys: {11: "anether", 12: "anither", 13: "another"}





        NOTE - by setting a unique storage key for each collection actually we're putting all the keys and entries of that collection inside a unique storage in memory which has a unique key or flag to avoid data collision for each collection's keys
        NOTE - since two different collections might have same key we'll set a prefix key for each collection using enum variant serialized to utf8 to avoid collection collision with same key in their entries, by doing this every collection will have a unique identifier and will be separated from other collection in which a same version of a key exists
        NOTE - every instascne of ByOwnerIdInner, ByNFTContractIdInner and ByNFTTokenTypeInner will have a new memory location address thus we can use it as an storage key since the hash of this key will be different and unique each time due to different memory location address of each instacne which won't be the same even if we create a new instance with a same field each time
        NOTE - enum has an extra size like 8 bytes, a 64 bits pointer which is big enough (64 bit arch os) to store the current vairant address for its tag which tells use which variant we have right now, but rust uses null pointer optimization instead of allocating 8 bytes tag  
        NOTE - null pointer optimization means a reference can never be null such as Option<&T> which is a pinter with 8 bytes length thus rust uses that reference or pointer as the tag with 8 bytes length for the current variant  
        NOTE - none struct variants in Storagekey enum allocates zero byte for the current persistent storage once the tag point to their address at a time
        NOTE - the enum size with zero byte for each variants would be the largest size of its variant + 8 bytes tag which would be 8 bytes in overall
        NOTE - an enum is the size of the maximum of its variants plus a discriminant value to know which variant it is, rounded up to be efficiently aligned, the alignment depends on the platform
        NOTE - an enum size is equals to a variant or the type with largest size + 8 bytes tag (there is only one 8 byte tag (size of the tag is usize) required since only one variant will be available at the same time)
        NOTE - enum size with a single f64 type variant would be 8 bytes and with four f64 variants would be 16 bytes cause one 8 bytes (the tag) wouldn't be enough because there would be no room for the tag
        NOTE - the size of the following enum is 24 (is equals to its largest variant size which belongs to the Text variant) + 8 (the tag size) bytes 
        
        pub enum UserID {
            Number(u64),
            Text(String),
        }
        

    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
*/
#[derive(BorshSerialize)] // NOTE - since UnorderedMap, LookupMap and UnorderedSet each one takes a vector of u8 as their key_prefix argument we have to bound the Storagekey enum to BorshSerialize trait to convert each variant into a vector of u8 using try_to_vec() method of the BorshSerialize trait - all collections (i.e. Vector, Map, Tree, etc) have an unique id which is called the storage key and can be either an encoded enum variant or an encoded string
// -> we've used an enum based storage key for better memory efficiency and avoiding data collision to keeps track of the persistent storage taken by the current collection (one of the following variant). 
// -> data collision could happen by UnorderedMap, LookupMap or UnorderedSet since these hashmap based structure generate a hash from their keys. 
// -> in order not to have a duplicate key entry inside hashmap based structures we can use enum to avoid having some hash collision with two distinct keys.
// -> with enum we can be sure that there will be only one collection (one of the following variant) at a time inside the storage that has been pointed by the enum tag.
// -> hash of the account_id inside the TokensPer* structs is the unique key to use it as the prefix for creating the UnorderedSet to avoid data collision cause every account_id has a unique hash with 256 bits long
pub enum Storagekey{ //// defining an enum based unique storage key for every our collections to avoid collection collision which might be happened when two different collections share a same storage for their keys on the chain which will face us data collision at runtime
    Sales, ////////---------➔ converting this to vector (Storagekey::Sales.try_to_vec().unwrap()) gives us an array of [0] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key 
    ByOwnerId, ////////---------➔ converting this to vector (Storagekey::ByOwnerId.try_to_vec().unwrap()) gives us an array of [1] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByOwnerIdInner { account_id_hash: [u8; 32] }, //// 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars (256/4=64) in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    ByNFTContractId, ////////---------➔ converting this to vector (Storagekey::ByNFTContractId.try_to_vec().unwrap()) gives us an array of [3] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTContractIdInner { account_id_hash: [u8; 2] }, //// 2 bytes or 256 bits (cause it's an array of 2 elements of type u8 which is 2 elements with 1 byte size) of the hash which will be 64 chars (256/4=64) in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    ByNFTTokenType, ////////---------➔ converting this to vector (Storagekey::ByNFTTokenType.try_to_vec().unwrap()) gives us an array of [5] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTTokenTypeInner { token_type_hash: [u8; 32] }, //// 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars (256/4=64) in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    FTTokenIds, ////////---------➔ converting this to vector (Storagekey::FTTokenIds.try_to_vec().unwrap()) gives us an array of [7] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    StorageDeposits, ////////---------➔ converting this to vector (Storagekey::StorageDeposits.try_to_vec().unwrap()) gives us an array of [8] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    Collection, ////////---------➔ converting this to vector (Storagekey::Collection.try_to_vec().unwrap()) gives us an array of [9] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
}







// ------------------------------ utility macros
// -------------------------------------------------------------------------------------------------
// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

// https://doc.rust-lang.org/reference/procedural-macros.html
// https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
// https://dev.to/dandyvica/rust-procedural-macros-step-by-step-tutorial-36n8
// https://doc.rust-lang.org/rust-by-example/macros.html
// https://doc.rust-lang.org/book/ch19-06-macros.html
// https://doc.rust-lang.org/reference/procedural-macros.html
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://www.youtube.com/watch?v=j0zDvzQr7WE&ab_channel=CodingTech
// https://steveklabnik.com/writing/an-overview-of-macros-in-rust
// https://hub.packtpub.com/creating-macros-in-rust-tutorial/
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/macros.html
// https://blog.logrocket.com/procedural-macros-in-rust/
// https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
// https://lib.rs/development-tools/procedural-macro-helpers
  
//// ➔ trait based proc macro attribute:
////    derive like macros like #[derive(Serialize, Deserialize, Copy, Clone, Debug)] to bound a struct to the trait which delegates trait implementation for the struct which contains the trait methods that will extend the interface of the type
////    cutom like macros like #[custom(out="code.wasm"] or #[near_bindgen] #[cfg(os)] or #[deprecated()] on top of struct or enum fields
////    convert a trait into a module that will extend the trait methods like near #[ex_contract(contract_name)] proc macro 
////    #[..] applies an attribute to the thing after it (struct, struct fields or crate) and  #![..] applies an attribute to the containing thing or crate
////    TokenStream arg using proc_macro2 crate and proc-macro = true flag inside the lib.rs file by using #[proc_macro], #[proc_macro_attribute] and #[proc_macro_derive] attributes

// ...


/*

    item      ➔ an Item | an item, like a function, struct, module, etc.
    block     ➔ a BlockExpression | a block (i.e. a block of statements and/or an expression, surrounded by braces)
    stmt      ➔ a Statement without the trailing semicolon (except for item statements that require semicolons)
    pat_param ➔ a PatternNoTopAlt
    pat       ➔ at least any PatternNoTopAlt, and possibly more depending on edition
    expr      ➔ an Expression
    ty        ➔ a Type
    ident     ➔ an IDENTIFIER_OR_KEYWORD or RAW_IDENTIFIER
    path      ➔ a TypePath style path | a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
    tt        ➔ a TokenTree (a single token or tokens in matching delimiters (), [], or {})
    meta      ➔ an Attr, the contents of an attribute | a meta item; the things that go inside #[...] and #![...] attributes
    lifetime  ➔ a LIFETIME_TOKEN
    vis       ➔ a possibly empty Visibility qualifier
    literal   ➔ matches -?LiteralExpression



macro based for pub/sub streamer actor on top of tokio, zmq and libp2p sockets like:
let stream = streamer!{
    tlp: "p2p|zmq|rpc|tokio-tcp|tokio-udp|local"
    type: "pub/sub"
    publish{
        topic: ""
        at: ""
    }
}

let res = streamer!{
    tlp: "p2p|zmq|rpc|tokio-tcp|tokio-udp|local"
    type: "pub/sub"
    subscribe{
        topic: ""

    }
}



//// event manager macro

list![func1, func2, func3] => {
    // first run event func2 then func1 and finally func3
    func2();
    func1();
    func3();
};


*/

#[macro_export]
macro_rules! db {

    ($name:expr, $engine:expr, $host:expr, $port:expr, $username:expr, $password:expr) => {

        { //// this is the key! this curly braces is required to use if let statement, use libs and define let inside macro
            let empty_app_storage = Some( //// putting the Arc-ed db inside the Option
                Arc::new( //// cloning app_storage to move it between threads
                    Storage{ //// defining db context 
                        id: Uuid::new_v4(),
                        db: Some(
                            Db{
                                mode: Mode::Off,
                                instance: None,
                                engine: None,
                                url: None,
                            }
                        ),
                    }
                )
            );
            let app_storage = if $engine.as_str() == "mongodb"{
                info!("➔ 🛢️ switching to mongodb");
                let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
                let db_addr = if environment == "dev"{
                    format!("{}://{}:{}", $engine, $host, $port)
                } else if environment == "prod"{
                    format!("{}://{}:{}@{}:{}", $engine, $username, $password, $host, $port)
                } else{
                    "".to_string()
                };
                match Db::new().await{
                    Ok(mut init_db) => { //// init_db instance must be mutable since we want to mutate its fields
                        init_db.engine = Some($engine);
                        init_db.url = Some(db_addr);
                        let mongodb_instance = init_db.GetMongoDbInstance().await; //// the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
                        Some( //// putting the Arc-ed db inside the Option
                            Arc::new( //// cloning app_storage to move it between threads
                                Storage{ //// defining db context 
                                    id: Uuid::new_v4(),
                                    db: Some(
                                        Db{
                                            mode: init_db.mode,
                                            instance: Some(mongodb_instance),
                                            engine: init_db.engine,
                                            url: init_db.url,
                                        }
                                    ),
                                }
                            )
                        )
                    },
                    Err(e) => {
                        error!("😕 init db error - {}", e);
                        empty_app_storage //// whatever the error is we have to return and empty app storage instance 
                    }
                }
            } else if $engine.as_str() == "postgres"{
                let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");                
                if environment == "dev"{
                    format!("{}://{}:{}", $engine, $host, $port)
                } else if environment == "prod"{
                    format!("{}://{}:{}@{}:{}", $engine, $username, $password, $host, $port)
                } else{
                    "".to_string()
                };
        
                // TODO 
                todo!();
            
            } else{
                empty_app_storage
            };

            app_storage //// returning the created app_storage

        }
    };

}


/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////                fetching user data from the conse auth server 
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
#[macro_export]
macro_rules! user_data {
    ($token:expr) => { //// we have to use match on where this macro is called
        {

            use mongodb::bson::oid::ObjectId;
            use serde::{Deserialize, Serialize};
            use log::info;


            #[derive(Debug, Serialize, Deserialize)]
            pub struct UserData{
                pub _id: Option<ObjectId>, //// this is the user id inside the users collection
                pub username: String,
                pub phone: String,
                pub access_level: u8, // NOTE - 0 means dev, 1 means admin, 2 means user
                pub status: u8, //// last status in an event game that this user has participated in
                pub role_id: Option<ObjectId>, //// last role in an event game that this user has participated in
                pub side_id: Option<ObjectId>, //// last side in an event game that this user has participated in
                pub created_at: Option<i64>,
                pub updated_at: Option<i64>,
                pub last_login_time: Option<i64>,
                pub wallet_address: String,
                pub balance: Option<u128>,
            }

            


            let coiniXerr_http_port = env::var("CONSE_PORT").expect("⚠️ please set ayoub port in .env");
            let host = env::var("HOST").expect("⚠️ please set host in .env");
            let url = format!("http://{}:{}/auth/check-token", host, coiniXerr_http_port);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .get(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<UserData>().await{ //// deserializing response utf8 bytes into the UserData struct
                                    Ok(resp) => {
                                        info!("[+] CURRENT SERVER TIME : {:?} | USER DATA FROM THE AYOUB SERVER : {:?}", chrono::Local::now().naive_local(), resp);
                                        Ok(resp)
                                    },
                                    Err(e) => {
                                        info!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                info!("[!] CURRENT SERVER TIME : {:?} | AYOUB SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    info!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! list {
    ($id1:ident | $id2:ident <- [$start:expr; $end:expr], $cond:expr) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc
        { //.... code block to return vec since using let statements must be inside {} block
            let mut vec = Vec::new();
            for num in $start..$end + 1{
                if $cond(num){
                    vec.push(num);
                }
            }
            vec
        } //....
    };
}
//////
/// let even = |x: i32| x%2 == 0;
/// let odd = |x: i32| x%2 != 0;
/// let evens = list![x | x <- [1; 10], even];
//////

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr)*) => { //// if this pattern matches the input the following code will be executed - * means we can pass more than one key => value statement
        { //.... code block to return vec since using let statements must be inside {} block
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )* //// * means we're inserting multiple key => value statement inside the map 
            map
        } //....
    };
}
//////
/// let d = dict!{"wildonion" => 1, "another_wildonion" => 2, "array": vec![1,3,4235,], "age": 24};
//////

#[macro_export]
macro_rules! exam {
    ($l:expr; and $r:expr) => { //// logical and match 
        $crate::macros::even(); //// calling even() function which is inside the macros module
        println!("{}", $l && $r);
    };

    ($l:expr; or $r:expr) => { //// logical or match 
        println!("{}", $l || $r);
    };
}
//////
/// exam!(1 == 2; and 3 == 2+1)
/// exam!(1 == 2; or 3 == 2+1)
//////


#[macro_export]
macro_rules! gendeh {
    ($iden:ident, $ty: tt) => {
        pub struct $iden(pub $ty);
        impl Default for $iden{
            fn default() -> Self{
                todo!()
            }
        }  
    };

    ($func_name:ident) => {
        fn $func_name(){
            println!("you've just called {:?}()", stringify!($func_name));
        }
    }
}
//////
/// gendeh!{bindgen, id} //// bindgen is the name of the struct and id is the name of the field
//////


#[macro_export]
macro_rules! query { // NOTE - this is a macro with multiple syntax support and if any pattern matches with the caller pattern, then the code block of that pattern will be emitted
    
    ( $value_0:expr, $value_1:expr, $value_2:expr ) => { //// passing multiple object syntax
        // ...
    };

    ( $($name:expr => $value:expr)* ) => { //// passing multiple key => value syntax 
        // ...

    };

}


#[macro_export]
macro_rules! log {
    ($arg:tt) => { //// passing single String message 
        $crate::env::log($arg.as_bytes()) //// log function only accepts utf8 bytes
    };
    ($($arg:tt)*) => { //// passing multiple String messages 
        $crate::env::log(format!($($arg)*).as_bytes()) //// log function only accepts utf8 bytes
    };
}


#[macro_export]
macro_rules! impl_engine_constructor {
    ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc 
        $(
            pub fn $new() -> Self{
                Self{
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )* //// * means defining function for every new Pos
    };
}


// #[derive(Debug, Clone)]
// pub struct Shape{
//     typ: &'static str,
//     positions: HashSet<Pos>,
//     anchor: Pos,
// }


// #[derive(Debug, Clone, Copy)]
// pub struct Pos(pub i32, pub i32);



// impl Shape {
//     impl_engine_constructor! {
//       new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
//       new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//       new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
//       new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
//       new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
//       new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
//       new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//     }
// }
