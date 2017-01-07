use super::{Error, GenericError};

#[derive(Debug)]
pub struct FromHexError_();
pub type FromHexError = GenericError<FromHexError_>;

pub trait ToHex {
   fn to_hex(&self) -> String;
}
pub trait FromHex {
   fn from_hex(&mut self, s:&str) -> Result<(), Error>;
}
pub trait WithHex: Sized {
   fn with_hex(&str) -> Result<Self, Error>;
}
impl <T:FromHex+Default> WithHex for T {
   fn with_hex(s:&str) -> Result<Self, Error> {
      let mut r = Self::default();
      r.from_hex(s).map(|_|{r})
   }
}

const B2H:&'static [u8] = b"0123456789abcdef";
impl ToHex for [u8] {
   fn to_hex(&self) -> String {
      let mut v = Vec::<u8>::with_capacity(self.len() * 2);
      for &b in self {
         v.push(B2H[ (b >> 4)   as usize ]);
         v.push(B2H[ (b & 0x0F) as usize ]);
      }
      unsafe { String::from_utf8_unchecked(v) }
   }
}

impl FromHex for [u8] {
   fn from_hex(&mut self, s:&str) -> Result<(), Error> {
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
   fn from_hex(&mut self, s:&str) -> Result<(), Error> {
      if s.len() % 2 != 0 { try!(Err(FromHexError::new("input has odd length"))) }
      self.resize(s.len()/2, 0);
      self.as_mut_slice().from_hex(s)
   }
}

#[test]
fn text_tohex() {
   assert_eq!(b"Hatsune Miku".to_hex(), "48617473756e65204d696b75");
}

#[test]
fn test_fromhex() {
   assert_eq!(     Vec::<u8>::with_hex("48617473756e65204d696b75").unwrap().as_slice(), b"Hatsune Miku");
   assert_matches!(Vec::<u8>::with_hex("48617473756e65204d696b7"), Err(_));
   assert_matches!(Vec::<u8>::with_hex("48617473756e65204d696b7x"), Err(_));
}
