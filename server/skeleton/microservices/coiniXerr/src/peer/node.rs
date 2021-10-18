


// NOTE - a p2p based network for coiniXerr
// TODO - use a codec like serde_json::from_slice or Transaction struct (TransactionMem) to map and deserialize utf8 bytes from memory into the defined object
// ...

// a tcp handler to control streaming of incoming future tasks or utf8 binary by awaiting on each of them 
// while let Some(task) = stream.next().await{}

// or

// a udp handler to control streaming of incoming future tasks or utf8 binary data through job queue channel protocol by awaiting on each of them
// while let Some((buffer, device_addr)) = receiver.recv().await{}


// NOTE - in order to move all data through the socket or http protocol they must be encoded from struct and converted to &[u8] serde codec serialize 
// NOTE - in order to get the data from the socket or http protocol they must be decoded from &[u8] to struct using serde codec deserialize
// TODO - a codec like web::Payload and ws::Message for streaming of binary data like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice or mapping struct into &[u8] bytes
// TODO - implement tokio channels like mpsc, oneshot, broadcast and watch
// TODO - jobq implementation in utils folder
// https://github.com/actix/examples/blob/master/websockets/tcp-chat/src/codec.rs
// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
// https://stackoverflow.com/questions/2490912/what-are-pinned-objects
// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
// https://github.com/zupzup/warp-websockets-example
// https://github.com/tokio-rs/tokio/tree/master/examples
// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://danielkeep.github.io/tlborm/book/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://docs.rs/tokio/1.12.0/tokio/sync/index.html
// https://docs.rs/tokio-stream/0.1.7/tokio_stream/
// https://doc.rust-lang.org/std/pin/index.html
// https://doc.rust-lang.org/std/sync/struct.Arc.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/std/sync/struct.RwLock.html
// https://doc.rust-lang.org/std/cell/struct.RefMut.html
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// https://doc.rust-lang.org/std/rc/struct.Weak.html
