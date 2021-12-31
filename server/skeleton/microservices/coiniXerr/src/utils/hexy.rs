


///////////// >>>>>>>> u8 bytes -> &str using str::from_utf8()
///////////// >>>>>>>> &str -> u8 bytes using as_bytes() or as_bytes_mut()
///////////// >>>>>>>> u8 -> u16 using transmute or shift bits operations (shift 2 bytes) or u8 to hex ascii string then to u16 using utils::from_hex_string_to_u16() function
///////////// >>>>>>>> u8 -> hex ascii string using utils::from_u8_to_hex_string() function
///////////// >>>>>>>> hex ascii string to u8 or u16 using from_str_radix() or utils functions
///////////// >>>>>>>> u8 -> hex ascii vector using :x? in println! macro or dividing operations : u8 bytes % 16 




use std::{fmt::Write, num::ParseIntError};


pub fn from_u8_to_hex_string(bytes: &[u8]) -> Result<String, ()> { //-- take a reference from u8 and will return a hex String
    /*
        let hex_ascii_string = "hello world".as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>()
        >> let mut s = String::new();
        >> use std::fmt::Write as FmtWrite; // renaming import to avoid collision
        >> for b in "hello world".as_bytes() { write!(s, "{:02x}", b); }
        ()
        >> s
        "68656c6c6f20776f726c64"
        >> 
    */
    let mut buffer = String::with_capacity(bytes.len() * 2); //-- length of the String must be double of the size of the u8 cause we want to write u16 or hex into this buffer
    for &b in bytes {
        write!(&mut buffer, "{:02x}", b).expect("⚠️ writing to String buffer error for hex ascii"); //-- writing formatted data into the buffer which is the String - panic on any error
    }
    Ok(buffer)
}


pub fn from_hex_string_to_u8(hex_string: &str) -> Result<Vec<u8>, ()>{
    let mut hex_bytes = hex_string.as_bytes().iter().filter_map(|b| {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }).fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    Ok(bytes)
}


pub fn from_hex_string_to_u16(s: &str) -> Result<Vec<u16>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u16::from_str_radix(&s[i..i + 2], 16))
        .collect()
}