def_error! { EncodeError }

#[macro_export]
macro_rules! encode_error {
   ($m:expr) => {
      try!( Err(::encode::EncodeError::new($m)) )
   }
}


