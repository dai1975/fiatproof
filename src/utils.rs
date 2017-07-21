
def_error! { FromBytesError }
def_error! { FromHexError }

#[macro_export]
macro_rules! frombytes_error {
   ($m:expr) => {
      try!( Err(::utils::FromBytesError::new($m)) )
   }
}
#[macro_export]
macro_rules! fromhex_error {
   ($m:expr) => {
      try!( Err(::utils::FromHexError::new($m)) )
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

use ::std::convert::AsRef;
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
