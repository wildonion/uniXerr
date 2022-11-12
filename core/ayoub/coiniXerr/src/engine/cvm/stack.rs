







use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case









pub struct Vm(pub u8);



pub async fn init() -> JoinHandle<Vm>{


    let vm = thread::spawn(move || {
        
        // vm codes and logics in here
        // ...
        
        Vm(38)
    
    });

    vm
    


}