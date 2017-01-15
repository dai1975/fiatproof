use std::convert::AsRef;

def_error! { FromBytesError }
def_error! { FromHexError }

#[macro_export]
macro_rules! frombytes_error {
   ($m:expr) => {
      try!( Err(::FromBytesError::new($m)) )
   }
}
#[macro_export]
macro_rules! fromhex_error {
   ($m:expr) => {
      try!( Err(::FromHexError::new($m)) )
   }
}

const B2H:&'static [u8] = b"0123456789abcdef";
pub fn b2h(bytes: &[u8]) -> String {
   let mut hex = Vec::<u8>::with_capacity(bytes.len() * 2);
   for b in bytes.iter() {
      hex.push(B2H[ (b >> 4)   as usize ]);
      hex.push(B2H[ (b & 0x0F) as usize ]);
   }
   unsafe { String::from_utf8_unchecked(hex) }
}

pub trait ToBytes {
   fn to_bytes(&self) -> ::Result<Vec<u8>>;

   fn to_hex(&self) -> ::Result<String> {
      self.to_bytes().map(|bytes| b2h(bytes.as_slice()))
   }
   fn to_rbytes(&self) -> ::Result<Vec<u8>> {
      self.to_bytes().map(|mut bytes| {
         bytes.reverse();
         bytes
      })
   }
   fn to_rhex(&self) -> ::Result<String> {
      self.to_rbytes().map(|bytes| b2h(bytes.as_slice()))
   }
}

pub fn h2b<S:AsRef<str>>(s:S) -> ::Result<Vec<u8>> {
   let s:&str = s.as_ref();
   if s.len() % 2 != 0 { fromhex_error!("input has odd length"); }
   let mut out = Vec::<u8>::with_capacity(s.len()/2);
   out.resize(s.len() / 2, 0u8);
   for (i,o) in out.iter_mut().enumerate() {
      let hex = &s[(i*2)..(i*2+2)];
      *o = try!(u8::from_str_radix(hex, 16));
   }
   Ok(out)
}

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

impl <T:AsRef<[u8]>> ToBytes for T {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      Ok( Vec::<u8>::from(self.as_ref()) )
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
   
pub trait ToDigest {
   fn to_digest_input(&self) -> ::Result<Vec<u8>>;
   fn to_dhash256(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, DHash256 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes))
   }
   fn to_hash160(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, Hash160 as H};
      let bytes = try!(self.to_digest_input());
      Ok(H::hash(bytes) )
   }
   fn to_dhash256_hex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_dhash256_rhex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_rhex() })
   }
   fn to_hash160_hex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_hash160_rhex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_rhex() })
   }
}


#[test]
fn text_tohex() {
   assert_eq!(b"Hatsune Miku".to_hex().unwrap(), "48617473756e65204d696b75");
}

#[test]
fn test_fromhex() {
   assert_eq!(     Vec::<u8>::with_hex("48617473756e65204d696b75").unwrap().as_slice(), b"Hatsune Miku");
   assert_matches!(Vec::<u8>::with_hex("48617473756e65204d696b7"), Err(_));
   assert_matches!(Vec::<u8>::with_hex("48617473756e65204d696b7x"), Err(_));
}
