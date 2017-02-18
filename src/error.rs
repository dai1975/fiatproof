use std;
use std::convert::Into;

#[derive(Debug,Clone)]
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
      #[derive(Debug,Clone)]
      pub struct [$n Phantom];
      pub type $n = ::GenericError< [$n Phantom] >;
   } }
}

macro_rules! def_error_convert {
   ( $( ($to:ident, $from:ty) ),* ,) => {
      #[derive(Debug,Clone)]
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
   (Io,           ::std::sync::Arc<::std::io::Error>), //be clonable
   (Utf8,         ::std::sync::Arc<::std::string::FromUtf8Error>),
   (ParseInt,     ::std::num::ParseIntError),
   (Encode,       ::codec::EncodeError),
   (Decode,       ::codec::DecodeError),
   (FromHex,      ::codec::FromHexError),
   (FromBytes,    ::codec::FromBytesError),
   (ParseScript,  ::script::ParseScriptError),
   (Script,       ::script::ScriptError),
}

impl From<::std::io::Error> for Error {
   fn from(err: ::std::io::Error) -> Error {
      Error::Io(::std::sync::Arc::new(err))
   }
}
impl From<::std::string::FromUtf8Error> for Error {
   fn from(err: ::std::string::FromUtf8Error) -> Error {
      Error::Utf8(::std::sync::Arc::new(err))
   }
}
