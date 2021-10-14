




// NOTE - actix actors are used for sending messages and events through their address instead of blocking the local thread for mutex acquisition using mpsc channel
// NOTE - all actix actors are on top of tokio in which every future task like actors communication events and messages will be handled by mpsc job q channel and multithreading patterns
// NOTE - mpsc channel is used while we're using a thread pool to mutate a data structure by blocking the thread to acquires a mutex by locking it and prevent other thread from mutating and locking it at a same time
// NOTE - the sender of mpsc channel can be owned by multiple threads but the receiver can only be owned by only one thread at a time, that's because it's called multi producer and single consumer  
// NOTE - we have to clone the receiver for passing between multiple threads and for mutating what's in it we have to put it inside a Mutex to insure that only one thread can change the content of the receiver at a time
// NOTE - mutex acquisition is done by waiting on the receiver until a job or task or the lock process becomes available to down side of the channel then locking on the receiver to acquire the mutex
// NOTE - multiple producers or workers own the receiver (Ac<T>) and single consumer or worker get the job at a time from the receiver (Mutex<T>)




pub mod miner;