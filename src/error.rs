use std;
use super::uint256::ParseUInt256Error;
use super::script::ScriptError;
//use serialize::SerializeError;

#[derive(Debug,PartialEq,Eq)]
pub struct GenericError {
   msg: String
}

impl GenericError {
   pub fn new(s:&str) -> Self {
      GenericError { msg:s.to_string() }
   }
}

impl std::error::Error for GenericError {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl std::fmt::Display for GenericError {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
      write!(f, "{}", self.msg)
   }
}

#[derive(Debug)]
pub enum Error {
   Io(std::io::Error),
   Utf8(std::string::FromUtf8Error),
   ParseInt(std::num::ParseIntError),

   ParseUInt256(ParseUInt256Error),
   //Serialize(SerializeError),
   Script(ScriptError),
   Generic(GenericError),
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

impl From<ParseUInt256Error> for Error {
   fn from(err: ParseUInt256Error) -> Error {
      Error::ParseUInt256(err)
   }
}
/*
impl From<SerializeError> for Error {
   fn from(err: SerializeError) -> Error {
      Error::Serialize(err)
   }
}
*/
impl From<ScriptError> for Error {
   fn from(err: ScriptError) -> Error {
      Error::Script(err)
   }
}

impl From<GenericError> for Error {
   fn from(err: GenericError) -> Error {
      Error::Generic(err)
   }
}

