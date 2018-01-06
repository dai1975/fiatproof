def_error! { EncodeError }
def_error! { DecodeError }

#[macro_export]
macro_rules! encode_error {
   ($m:expr) => {
      ::serialize::EncodeError::new($m, 0)
   }
}
#[macro_export]
macro_rules! raise_encode_error {
   ($m:expr) => {
      try!(Err(encode_error!($m)))
   }
}

#[macro_export]
macro_rules! decode_error {
   ($m:expr) => {
      ::serialize::DecodeError::new($m, 0)
   }
}
#[macro_export]
macro_rules! raise_decode_error {
   ($m:expr) => {
      try!(Err(decode_error!($m)))
   }
}


