extern crate crypto;
pub use self::crypto::digest::Digest;
pub use self::crypto::sha2::{Sha512, Sha256};
pub use self::crypto::sha1::Sha1;
pub use self::crypto::ripemd160::Ripemd160;

#[macro_use]
pub mod helper;
pub use self::helper::{
   DigestHelpable,
   DigestHelper,
   Sha256Helper, Sha512Helper, Ripemd160Helper,
};

pub mod double;
pub use self::double::{
   DHash256, DHash256Helper,
   Hash160, Hash160Helper,
};


