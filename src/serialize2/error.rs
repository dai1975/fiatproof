def_error! { SerializeError }
def_error! { DeserializeError }

#[macro_export]
macro_rules! serialize_error {
   ($m:expr) => {
      try!( Err(::serialize2::SerializeError::new($m)) )
   }
}

#[macro_export]
macro_rules! deserialize_error {
   ($m:expr) => {
      try!( Err(::serialize2::DeserializeError::new($m)) )
   }
}


