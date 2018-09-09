def_error! { Secp256k1Error }

#[macro_export]
macro_rules! raise_secp256k1_error {
   ($m:expr) => {
      try!( Err(::crypto::secp256k1::error::Secp256k1Error::new($m, 0)) )
   }
}

#[macro_export]
macro_rules! secp256k1_error {
   ($m:expr) => {
      ::crypto::secp256k1::error::Secp256k1Error::new($m, 0)
   }
}

#[macro_export]
macro_rules! error_secp256k1_error {
   ($m:expr) => {
      Err(::error::Error::from(::crypto::secp256k1::error::Secp256k1Error::new($m, 0)))
   }
}

use ::std::convert::Into;
extern crate secp256k1;
impl From<secp256k1::Error> for ::Error {
   fn from(err: secp256k1::Error) -> ::Error {
      let msg = match err {
         secp256k1::Error::IncorrectSignature => "IncorrectSignature",
         secp256k1::Error::InvalidMessage     => "InvalidMessage",
         secp256k1::Error::InvalidPublicKey   => "InvalidPublicKey",
         secp256k1::Error::InvalidSignature   => "InvalidSignature",
         secp256k1::Error::InvalidSecretKey   => "InvalidSecretKey",
         secp256k1::Error::InvalidRecoveryId  => "InvalidRecoveryId",
      };
      let moderr = secp256k1_error!(msg);
      ::error::Error::from(moderr)
   }
}
