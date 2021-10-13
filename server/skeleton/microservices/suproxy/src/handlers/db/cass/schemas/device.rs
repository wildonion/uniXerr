






use crate::handlers::db::cass::establish::CassSession;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::query_values;
use cdrs_helpers_derive::*;






#[repr(C)] //-- #[repr(C, packed)] : borrow of packed field is unsafe and requires unsafe function or block
union GPSMem{
    pub data: *mut self::GPS,
    pub buffer: *const u8,
}






/////////////////////////////////////////////////// CASSANDRA ARCHITECTURE ///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////// https://github.com/AlexPikalov/cdrs/blob/master/type-mapping.md /////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// 
#[derive(Serialize, Deserialize, Clone, IntoCDRSValue, PartialEq, Debug, TryFromRow)] //-- Serialize and Deserialize is required for sedning json in response
pub struct GPS{
    pub id: Uuid,
    pub imei: Option<i16>,
    pub lat: Option<i32>,
    pub lon: Option<i32>,
    pub alt: Option<i16>,
    pub angle: Option<i16>,
    pub satellites: Option<u8>,
    pub speed: Option<i16>,
    pub timestamp: Option<i64>,
}



impl Default for GPS{
    fn default() -> Self{
        todo!()
    }
}



impl GPS{
    fn new(buffer: &[u8]) -> Option<&mut GPS>{
        unsafe impl Send for GPS {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for GPS union in order to be shareable between tokio threads
        unsafe{
            let mut gps_data: Option<&mut GPS> = None; //-- in order to prevent from null checking we took a safe rust based reference to GPS inside the Option (cause rust doesn't have null) cause *mut raw pointer to GPS inside union is not safe also can't move between thread safely and once the GPSMem data has dereferenced it might return a null pointer (dangled) due to unsafe manner of C pointers
            let gps = GPSMem{buffer: buffer.as_ptr() as *const u8}; //-- getting a C pointer of the filled buffer which is the hex address of the memory, doning this is unsafe due to unsafeness manner of raw pointers in rust
            gps_data = Some(&mut (*gps.data)); //-- taking a reference (smart pointer to address of gps.data in the stack) from dereferenced *mut raw pointer from the union inside the unsafe block is done through * like in C syntax  - we only wants the data so we didn't do any read operation on buffer field inside the union
            gps_data
        }
    }
}
