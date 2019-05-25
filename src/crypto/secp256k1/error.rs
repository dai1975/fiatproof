def_error! { Secp256k1Error }

#[macro_export]
macro_rules! raise_secp256k1_error {
   ($m:expr) => {
      Err(crate::crypto::secp256k1::error::Secp256k1Error::new($m, 0))?
   }
}

#[macro_export]
macro_rules! secp256k1_error {
   ($m:expr) => {
      crate::crypto::secp256k1::error::Secp256k1Error::new($m, 0)
   }
}

#[macro_export]
macro_rules! error_secp256k1_error {
   ($m:expr) => {
      Err(crate::error::Error::from(crate::crypto::secp256k1::error::Secp256k1Error::new($m, 0)))
   }
}

use std::convert::Into;
impl From<secp256k1::Error> for crate::Error {
   fn from(err: secp256k1::Error) -> crate::Error {
      let msg = match err {
         secp256k1::Error::IncorrectSignature => "IncorrectSignature",
         secp256k1::Error::InvalidMessage     => "InvalidMessage",
         secp256k1::Error::InvalidPublicKey   => "InvalidPublicKey",
         secp256k1::Error::InvalidSignature   => "InvalidSignature",
         secp256k1::Error::InvalidSecretKey   => "InvalidSecretKey",
         secp256k1::Error::InvalidRecoveryId  => "InvalidRecoveryId",
         secp256k1::Error::InvalidTweak       => "InvalidTweak",
      };
      let moderr = secp256k1_error!(msg);
      crate::error::Error::from(moderr)
   }
}
