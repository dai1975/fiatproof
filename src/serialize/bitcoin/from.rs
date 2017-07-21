use ::std::convert::AsRef;
use ::utils::h2b;

// TODO: std::convert::From に合わせて、from は (T) -> Self で。
pub trait FromBytes {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()>;

   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      h2b(s).and_then(|bytes| self.from_bytes(&bytes))
   }
   fn from_rbytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      self.from_bytes(rev.as_slice())
   }
   fn from_rhex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      h2b(s).and_then(|bytes| self.from_rbytes(&bytes))
   }
}

impl FromBytes for Vec<u8> {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      let s = s.as_ref();
      self.resize(s.len(), 0u8);
      self.clone_from_slice(s);
      Ok(())
   }
}

pub trait WithBytes: Sized {
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

impl <T:FromBytes+Default> WithBytes for T {
   fn with_bytes<S:AsRef<[u8]>>(s:S) -> ::Result<Self> {
      let mut r = Self::default();
      r.from_bytes(s).map(|_| r)
   }
}


impl FromBytes for super::Decodee {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      use ::serialize::SliceReadStream;
      let mut r = SliceReadStream::new(s.as_ref());
      {
         use super::{Decoder, Medium};
         let mut d = Decoder::new(&mut r, &Medium::default());
         let _ = try!(self.decode(&mut d));
      }
      Ok(())
   }
}
