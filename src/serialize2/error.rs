use std;
use serde;

def_error! { SerializeError }
def_error! { DeserializeError }

impl serde::ser::Error for SerializeError {
   fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
      SerializeError::new(format!("{}", msg))
   }
}
impl serde::de::Error  for DeserializeError {
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

/* serde::x:Error は trait なので Sized ではなく、From<T> は暗黙に Sized 要求。
impl From<serde::ser::Error> for SerializeError {
   fn from(err: serde::ser::Error) -> SerializeError {
      SerializeError::new(format!("serde::ser::Error {}", err))
   }
}
impl From<serde::de::Error> for DeserializeError {
   fn from(err: serde::de::Error) -> DeserializeError {
      DeserializeError::new(format!("serde::de::Error {}", err))
   }
}
*/


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


#[macro_export]
macro_rules! ser_error {
   ($m:expr) => {
      use serde::ser;
      return Err(ser::Error::custom($m));
   }
}

#[macro_export]
macro_rules! de_error {
   ($m:expr) => {
      use serde::de;
      return Err(de::Error::custom($m));
   }
}



