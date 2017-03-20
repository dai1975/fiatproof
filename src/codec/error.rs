def_error! { EncodeError }
def_error! { DecodeError }

#[macro_export]
macro_rules! encode_error {
   ($m:expr) => {
      try!( Err(::codec::EncodeError::new($m)) )
   }
}

#[macro_export]
macro_rules! decode_error {
   ($m:expr) => {
      try!( Err(::codec::DecodeError::new($m)) )
   }
}


