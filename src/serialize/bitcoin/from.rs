use ::std::convert::AsRef;
use ::utils::h2b;

// TODO: std::convert::From に合わせて、from は (T) -> Self で。
pub trait FromBytes {
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


impl <T> FromBytes for T where T: super::Decodee {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize> {
      use ::serialize::SliceReadStream;
      let mut r = SliceReadStream::new(s.as_ref());
      use super::{Decoder, Medium};
      let mut d = Decoder::new(&mut r, &Medium::default());
      self.decode(&mut d)
   }
}

#[cfg(test)]
mod tests {
   use ::serialize::bitcoin::{Decoder, Decodee};
   #[derive(Default)] //WithByte needs Default
   struct Foo { s: String }
   impl Decodee for Foo {
      fn decode(&mut self, d:&mut Decoder) -> ::Result<usize> {
         d.decode_string(&mut self.s, None)
      }
   }
   
   #[test]
   fn test_from_bytes() {
      let buf:&[u8] = &[
         12,
         0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75,
      ];
      {
         use ::serialize::bitcoin::{FromBytes};
         let mut f = Foo::default();
         assert_matches!(f.from_bytes(&buf), Ok(13));
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
      {
         use ::serialize::bitcoin::{WithBytes};
         let f = Foo::with_bytes(&buf).unwrap();
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
   }
}
