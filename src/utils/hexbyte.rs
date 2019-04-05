use ::std::borrow::Borrow;

def_error! { HexByteError }

macro_rules! raise_hexbyte_error {
   ($m:expr) => {
      Err(::utils::HexByteError::new($m, 0))?
   }
}

const B2H:&'static [u8] = b"0123456789abcdef";
pub fn b2h<T:Borrow<[u8]>>(bytes: T) -> String {
   let bytes = bytes.borrow();
   let mut hex = Vec::<u8>::with_capacity(bytes.len() * 2);
   for b in bytes.iter() {
      hex.push(B2H[ (b >> 4)   as usize ]);
      hex.push(B2H[ (b & 0x0F) as usize ]);
   }
   unsafe { String::from_utf8_unchecked(hex) }
}
pub fn b2h_rev<T:Borrow<[u8]>>(bytes: T) -> String {
   let bytes = bytes.borrow();
   let mut hex = Vec::<u8>::with_capacity(bytes.len() * 2);
   for b in bytes.iter().rev() {
      hex.push(B2H[ (b >> 4)   as usize ]);
      hex.push(B2H[ (b & 0x0F) as usize ]);
   }
   unsafe { String::from_utf8_unchecked(hex) }
}

pub fn h2b<S:Borrow<str>>(s:S) -> ::Result<Box<[u8]>> {
   let s:&str = s.borrow();
   if s.len() % 2 != 0 { raise_hexbyte_error!("input has odd length"); }
   let mut out = Vec::<u8>::with_capacity(s.len()/2);
   out.resize(s.len() / 2, 0u8);
   for (i,o) in out.iter_mut().enumerate() {
      let hex = &s[(i*2)..(i*2+2)];
      *o = u8::from_str_radix(hex, 16)?;
   }
   Ok(out.into_boxed_slice())
}
pub fn h2b_rev<S:Borrow<str>>(s:S) -> ::Result<Box<[u8]>> {
   let s:&str = s.borrow();
   if s.len() % 2 != 0 { raise_hexbyte_error!("input has odd length"); }
   let mut out = Vec::<u8>::with_capacity(s.len()/2);
   out.resize(s.len() / 2, 0u8);
   for (i,o) in out.iter_mut().rev().enumerate() {
      let hex = &s[(i*2)..(i*2+2)];
      *o = u8::from_str_radix(hex, 16)?;
   }
   Ok(out.into_boxed_slice())
}

#[test]
fn test_b2h() {
   let b = "Hatsune Miku".as_bytes();
   let h = b2h(b);
   assert_eq!("48617473756e65204d696b75", h);
}

#[test]
fn test_b2h_rev() {
   let b = "Hatsune Miku".as_bytes();
   let h = b2h_rev(b);
   assert_eq!("756b694d20656e7573746148", h);
}

#[test]
fn test_h2b() {
   let h = "48617473756e65204d696b75";
   let b = h2b(h).unwrap();
   assert_eq!("Hatsune Miku".as_bytes(), b.as_ref());
}

#[test]
fn test_h2b_rev() {
   let h = "756b694d20656e7573746148";
   let b = h2b_rev(h).unwrap();
   assert_eq!("Hatsune Miku".as_bytes(), b.as_ref());
}
