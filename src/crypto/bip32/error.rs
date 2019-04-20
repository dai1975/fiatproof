def_error! { Bip32Error }

#[macro_export]
macro_rules! raise_bip32_error {
   ($m:expr) => {
      Err(crate::crypto::bip32::error::Bip32Error::new($m, 0))?
   }
}

#[macro_export]
macro_rules! bip32_error {
   ($m:expr) => {
      crate::crypto::bip32::error::Bip32Error::new($m, 0)
   }
}

#[macro_export]
macro_rules! error_bip32_error {
   ($m:expr) => {
      Err(crate::error::Error::from(crate::crypto::bip32::error::Bip32Error::new($m, 0)))
   }
}

