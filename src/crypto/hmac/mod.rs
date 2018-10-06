extern crate crypto;
pub use self::crypto::hmac::Hmac;
pub use self::crypto::mac::Mac;

#[macro_use]
pub mod helper;
pub use self::helper::{
   Helpable,
   Helper,
   HmacSha256Helper, HmacSha512Helper,
};



