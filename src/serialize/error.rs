def_error! { SerializeError }

#[macro_export]
macro_rules! serialize_error {
   ($m:expr) => {
      try!( Err(::serialize::SerializeError::new($m)) )
   }
}


