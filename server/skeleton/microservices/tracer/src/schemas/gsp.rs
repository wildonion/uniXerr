


use serde::{Serialize, Deserialize};
use uuid::Uuid;





// -----------------------------------------------------------------------------------------------
// in order to prevent from null checking we have to take a safe rusty based reference to GPSData 
// inside the Option (cause rust doesn't have null) cause *mut raw pointer to GPSMem inside 
// union is not safe also can't move between thread safely and once the GPS data has dereferenced 
// it might return a null pointer due to unsafe manner of C pointers.
// -----------------------------------------------------------------------------------------------
#[repr(C)] //-- #[repr(C, packed)] : borrow of packed field is unsafe and requires unsafe function or block
union GPSData{
    pub data: *mut self::GPSMem,
    pub buffer: *const u8,
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[repr(C)]
pub struct GPSMem{
    pub id: Uuid,
    pub time: i64,
}






impl Default for GPSMem{
    fn default() -> Self{
        GPSMem{
            id: Uuid::new_v4(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}
