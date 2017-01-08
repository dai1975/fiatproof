use std;
use super::hexbytes::{FromHexError, FromBytesError};
use super::uint256::ParseUInt256Error;
use super::script::ScriptError;
use super::serialize::SerializeError;

#[derive(Debug)]
pub struct GenericError<T> {
   msg: String,
   phantom: std::marker::PhantomData<T>,
}

impl <T> GenericError<T> {
   pub fn new(s:&str) -> Self {
      GenericError { msg: s.to_string(), phantom: std::marker::PhantomData::<T>::default() }
   }
}

impl <T:std::fmt::Debug> std::error::Error for GenericError<T> {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl <T> std::fmt::Display for GenericError<T> {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
      write!(f, "{}: {}", unsafe { std::intrinsics::type_name::<T>() }, self.msg)
   }
}

#[derive(Debug)]
pub enum Error {
   Io(std::io::Error),
   Utf8(std::string::FromUtf8Error),
   ParseInt(std::num::ParseIntError),

   ParseUInt256(ParseUInt256Error),
   FromHex(FromHexError),
   FromBytes(FromBytesError),
   Serialize(SerializeError),
   Script(ScriptError),
}

impl From<std::io::Error> for Error {
   fn from(err: std::io::Error) -> Error {
      Error::Io(err)
   }
}

impl From<std::string::FromUtf8Error> for Error {
   fn from(err: std::string::FromUtf8Error) -> Error {
      Error::Utf8(err)
   }
}

impl From<std::num::ParseIntError> for Error {
   fn from(err: std::num::ParseIntError) -> Error {
      Error::ParseInt(err)
   }
}

impl From<FromHexError> for Error {
   fn from(err: FromHexError) -> Error {
      Error::FromHex(err)
   }
}
impl From<FromBytesError> for Error {
   fn from(err: FromBytesError) -> Error {
      Error::FromBytes(err)
   }
}
impl From<ParseUInt256Error> for Error {
   fn from(err: ParseUInt256Error) -> Error {
      Error::ParseUInt256(err)
   }
}
impl From<SerializeError> for Error {
   fn from(err: SerializeError) -> Error {
      Error::Serialize(err)
   }
}

impl From<ScriptError> for Error {
   fn from(err: ScriptError) -> Error {
      Error::Script(err)
   }
}

