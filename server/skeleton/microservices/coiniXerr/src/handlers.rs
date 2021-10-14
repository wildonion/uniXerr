



// NOTE - in order to move all data through the socket or http protocol they must be encoded from struct and converted to &[u8] serde codec serialize 
// NOTE - in order to get the data from the socket or http protocol they must be decoded from &[u8] to struct using serde codec deserialize
// TODO - a codec for streaming of binary data like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice or mapping struct into &[u8] bytes
// TODO - implement tokio channels like mpsc, oneshot, broadcast and watch
// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros 
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
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/std/sync/struct.RwLock.html
// https://doc.rust-lang.org/std/cell/struct.RefMut.html
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// https://doc.rust-lang.org/std/rc/struct.Weak.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1


pub mod pos;
pub mod socket;
pub mod jobq;
pub mod actors;