#[macro_use]
pub mod helper;
pub use self::helper::{
   input_hex, input_hex_rev,
   result_u8, result_hex, result_hex_rev,
   u8_to_u8, u8_to_hex, u8_to_hex_rev, hex_to_u8,
   hex_to_hex, hex_to_u8_rev, hex_to_hex_rev,
};
