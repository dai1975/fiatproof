def_error! { SerializeError }
def_error! { DeserializeError }

#[macro_export]
macro_rules! serialize_error {
   ($m:expr) => {
      ::bitcoin::serialize::SerializeError::new($m, 0)
   }
}

#[macro_export]
macro_rules! raise_serialize_error {
   ($m:expr) => {
      try!( Err( serialize_error!($m) ) )
   }
}



#[macro_export]
macro_rules! deserialize_error {
   ($m:expr) => {
      ::bitcoin::serialize::DeserializeError::new($m, 0)
   }
}

#[macro_export]
macro_rules! raise_deserialize_error {
   ($m:expr) => {
      try!( Err( deserialize_error!($m) ) )
   }
}

