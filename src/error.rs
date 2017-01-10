use std;
use std::convert::Into;

#[derive(Debug)]
pub struct GenericError<T> {
   msg: String,
   phantom: ::std::marker::PhantomData<T>,
}

impl <T> GenericError<T> {
   pub fn new<S:Into<String>>(s: S) -> Self {
      GenericError { msg: s.into(), phantom: ::std::marker::PhantomData::<T>::default() }
   }
}

impl <T: ::std::fmt::Debug> std::error::Error for GenericError<T> {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl <T> ::std::fmt::Display for GenericError<T> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
      write!(f, "{}: {}", unsafe { ::std::intrinsics::type_name::<T>() }, self.msg)
   }
}

#[macro_export]
macro_rules! def_error {
   ($n:ident) => { interpolate_idents! {
      #[derive(Debug)]
      pub struct [$n Phantom];
      pub type $n = ::GenericError< [$n Phantom] >;
   } }
}

macro_rules! def_error_convert {
   ( $( ($to:ident, $from:ty) ),* ,) => {
      #[derive(Debug)]
      pub enum Error {
         $(
            $to($from),
         )*
      }
      $(
         impl From<$from> for Error {
            fn from(err: $from) -> Error {
               Error::$to(err)
            }
         }
      )*
   }
}

def_error_convert! {
   (Io,           ::std::io::Error),
   (Utf8,         ::std::string::FromUtf8Error),
   (ParseInt,     ::std::num::ParseIntError),
   (FromHex,      ::hexbytes::FromHexError),
   (FromBytes,    ::hexbytes::FromBytesError),
   (ParseUInt256, ::uint256::ParseUInt256Error),
   (Serialize,    ::serialize::SerializeError),
   (ParseScript,  ::script::ParseScriptError),
   (Script,       ::script::ScriptError),
}

