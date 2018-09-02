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
