








use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case





/// 1e-24 Ⓝ is 1 yocto Ⓝ 
/// 1 Ⓝ is 1e24 yocto Ⓝ 
/// 3 Ⓝ to yocto Ⓝ is : 3 * 1e-24 * 1e-24 = 3
/// 3 yocto Ⓝ to Ⓝ is : 3 * 1e-24 = 3e-24
pub const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000 as u128;