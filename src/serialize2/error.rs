def_error! { SerializeError }
def_error! { DeserializeError }

#[macro_export]
macro_rules! serialize_error {
   ($m:expr) => {
      try!( Err(::serialize::SerializeError::new($m)) )
   }
}

#[macro_export]
macro_rules! deserialize_error {
   ($m:expr) => {
      try!( Err(::serialize::DeserializeError::new($m)) )
   }
}


