use std::convert::AsRef;

pub trait ToHex {
   fn to_hex(&self) -> ::Result<String>;
}
pub trait ToReverseHex {
   fn to_reverse_hex(&self) -> ::Result<String>;
}
pub trait ToBytes {
   fn to_bytes(&self) -> ::Result<Vec<u8>>;
}

def_error! { FromHexError }
def_error! { FromBytesError }

pub trait FromHex {
   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()>;
}
pub trait WithHex: Sized {
   fn with_hex<S:AsRef<str>>(s:S) -> ::Result<Self>;
}
pub trait FromBytes {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()>;
}
pub trait WithBytes: Sized {
   fn with_bytes<S:AsRef<[u8]>>(s:S) -> ::Result<Self>;
}


const B2H:&'static [u8] = b"0123456789abcdef";
impl ToHex for [u8] {
   fn to_hex(&self) -> ::Result<String> {
      let mut v = Vec::<u8>::with_capacity(self.len() * 2);
      for &b in self {
         v.push(B2H[ (b >> 4)   as usize ]);
         v.push(B2H[ (b & 0x0F) as usize ]);
      }
      Ok(unsafe { String::from_utf8_unchecked(v) })
   }
}
impl ToReverseHex for [u8] {
   fn to_reverse_hex(&self) -> ::Result<String> {
      let mut v = Vec::<u8>::with_capacity(self.len() * 2);
      for i in 0..self.len() {
         let b = self[self.len() - i - 1];
         v.push(B2H[ (b >> 4)   as usize ]);
         v.push(B2H[ (b & 0x0F) as usize ]);
      }
      Ok(unsafe { String::from_utf8_unchecked(v) })
   }
}
impl ToHex for Vec<u8> {
   fn to_hex(&self) -> ::Result<String> { self.as_slice().to_hex() }
}
impl ToReverseHex for Vec<u8> {
   fn to_reverse_hex(&self) -> ::Result<String> { self.as_slice().to_reverse_hex() }
}
impl ToHex for Box<[u8]> {
   fn to_hex(&self) -> ::Result<String> { self.as_ref().to_hex() }
}
impl ToReverseHex for Box<[u8]> {
   fn to_reverse_hex(&self) -> ::Result<String> { self.as_ref().to_reverse_hex() }
}
impl <T:ToBytes> ToHex for T {
   fn to_hex(&self) -> ::Result<String> {
      self.to_bytes().and_then(|bytes| { bytes.to_hex() })
   }
}


impl FromHex for [u8] {
   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      let s:&str = s.as_ref();
      if s.len() % 2 != 0 { try!(Err(FromHexError::new("input has odd length"))) }
      if s.len() / 2 != self.len() { try!(Err(FromHexError::new("input has odd length"))) }
      for (i,o) in self.iter_mut().enumerate() {
         let hex = &s[(i*2)..(i*2+2)];
         *o = try!(u8::from_str_radix(hex, 16));
      }
      Ok(())
   }
}
impl FromHex for Vec<u8> {
   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      let s:&str = s.as_ref();
      if s.len() % 2 != 0 { try!(Err(FromHexError::new("input has odd length"))) }
      self.resize(s.len()/2, 0);
      self.as_mut_slice().from_hex(s)
   }
}
impl <T:FromBytes> FromHex for T {
   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      Vec::<u8>::with_hex(s).and_then(|v| { self.from_bytes(v.as_slice()) })
   }
}
impl <T:FromHex+Default> WithHex for T {
   fn with_hex<S:AsRef<str>>(s:S) -> ::Result<Self> {
      let mut r = Self::default();
      r.from_hex(s).map(|_|{r})
   }
}
impl <T:FromBytes+Default> WithBytes for T {
   fn with_bytes<S:AsRef<[u8]>>(s:S) -> ::Result<Self> {
      let mut r = Self::default();
      r.from_bytes(s).map(|_|{r})
   }
}



pub trait ToHash {
   fn to_hash_input(&self) -> ::Result<Vec<u8>>;
   fn to_dhash256(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, DHash256 as H};
      let bytes = try!(self.to_hash_input());
      Ok(H::hash(bytes))
   }
   fn to_hash160(&self) -> ::Result<Box<[u8]>> {
      use ::crypto::{Hasher, Hash160 as H};
      let bytes = try!(self.to_hash_input());
      Ok(H::hash(bytes) )
   }
   fn to_dhash256_hex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_dhash256_reverse_hex(&self) -> ::Result<String> {
      self.to_dhash256().and_then(|bytes| { bytes.to_reverse_hex() })
   }
   fn to_hash160_hex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_hex() })
   }
   fn to_hash160_reverse_hex(&self) -> ::Result<String> {
      self.to_hash160().and_then(|bytes| { bytes.to_reverse_hex() })
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
