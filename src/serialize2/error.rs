use std;
use serde::{ser, de};

def_error! { SerializeError }
def_error! { DeserializeError }

impl ser::Error for SerializeError {
   fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
      SerializeError::new(format!("{}", msg))
   }
}
impl de::Error  for DeserializeError {
   fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
      DeserializeError::new(format!("{}", msg))
   }
}

impl From<std::io::Error> for SerializeError {
   fn from(err: std::io::Error) -> SerializeError {
      SerializeError::new(format!("std::io::Error {}", err))
   }
}
impl From<std::io::Error> for DeserializeError {
   fn from(err: std::io::Error) -> DeserializeError {
      DeserializeError::new(format!("std::io::Error {}", err))
   }
}


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


