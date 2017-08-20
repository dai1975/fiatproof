use ::std::convert::AsRef;
use ::utils::h2b;

// TODO: std::convert::From に合わせて、from は (T) -> Self で。
pub trait FromBytes<T> where T:?Sized {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize>;

   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<usize> {
      h2b(s).and_then(|bytes| self.from_bytes(&bytes))
   }
   fn from_rbytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      self.from_bytes(rev.as_slice())
   }
   fn from_rhex<S:AsRef<str>>(&mut self, s:S) -> ::Result<usize> {
      h2b(s).and_then(|bytes| self.from_rbytes(&bytes))
   }
}

pub trait WithBytes<T>: Sized where T:?Sized {
   fn with_bytes<S:AsRef<[u8]>>(s:S) -> ::Result<Self>;
   fn with_hex<S:AsRef<str>>(s:S) -> ::Result<Self> {
      h2b(s).and_then(|bytes| Self::with_bytes(&bytes))
   }
   fn with_rbytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<Self> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      Self::with_bytes(rev.as_slice())
   }
   fn with_rhex<S:AsRef<str>>(s:S) -> ::Result<Self> {
      h2b(s).and_then(|mut bytes| {
         bytes.reverse();
         Self::with_bytes(&bytes)
      })
   }
}

impl <T,X> WithBytes<T> for X where T:?Sized, X:FromBytes<T>+Default {
   fn with_bytes<S:AsRef<[u8]>>(s:S) -> ::Result<Self> {
      let mut r = Self::default();
      r.from_bytes(s).map(|_| r)
   }
}


