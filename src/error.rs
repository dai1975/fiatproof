use ::std::convert::Into;
use ::std::marker::PhantomData;
use ::bitcoin;

#[derive(Debug,Clone)]
pub struct GenericError<T> {
   msg: String,
   pub code: u32,
   pub backtrace: String,
   phantom: PhantomData<T>,
}

impl <T> GenericError<T> {
   pub fn new<S:Into<String>>(s: S, code:u32) -> Self {
      GenericError {
         msg: s.into(),
         code:code,
         backtrace: format!("{:?}", ::backtrace::Backtrace::new()),
         phantom: PhantomData::<T>::default() }
   }
}

impl <T: ::std::fmt::Debug> ::std::error::Error for GenericError<T> {
   fn description(&self) -> &str {
      &*self.msg
   }
}
impl <T> ::std::fmt::Display for GenericError<T> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
      //write!(f, "{}: {}", unsafe { ::std::intrinsics::type_name::<T>() }, self.msg)
      write!(f, "{}", self.msg)
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

def_error! { ParseError }
#[macro_export]
macro_rules! parse_error {
   ($m:expr) => {
      ::error::ParseError::new($m, 0)
   }
}
#[macro_export]
macro_rules! raise_parse_error {
   ($m:expr) => {
      try!( Err( parse_error!($m) ))
   }
}


def_error! { UnknownError }
#[macro_export]
macro_rules! unknown_error {
   ($m:expr) => {
      ::error::UnknownError::new($m, 0)
   }
}
#[macro_export]
macro_rules! raise_unknown_error {
   ($m:expr) => {
      try!( Err( unknown_error!($m) ))
   }
}

macro_rules! def_error_convert {
   ( $( ($to:ident, $from:ty) ),* ,) => {
      #[derive(Debug,Clone)]
      pub enum Error {
         $(
            $to($from),
         )*
      }
      impl ::std::error::Error for Error {
         fn description(&self) -> &str {
            match self { $(
               &Error::$to(ref from) => from.description(),
            )* }
         }
      }
      impl ::std::fmt::Display for Error {
         fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            match self { $(
               &Error::$to(ref from) => write!(f, "{}", from),
            )* }
         }
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
   (Secp256k1,    ::crypto::secp256k1::Secp256k1Error),
   (Parse,        ParseError),
   (Unknown,      UnknownError),
   (BaseNError,   ::utils::BaseNError),
   (Base58checkError, ::utils::Base58checkError),
   (HexByte,      ::utils::HexByteError),
   (BitcoinSerialize,          ::bitcoin::serialize::SerializeError),
   (BitcoinDeserialize,          ::bitcoin::serialize::DeserializeError),
   (BitcoinScript,          ::bitcoin::script::Error),
   (BitcoinParseScript,     ::bitcoin::script::ParseError),
   (BitcoinInterpretScript, ::bitcoin::script::InterpretError),
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

