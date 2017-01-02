use ::Error;
use super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream};

pub struct LimitedString<'a>(&'a str, usize);

impl <'a, W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for LimitedString<'a> {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use std::cmp::min;
      use std::u32::MAX;
      let bytes = self.0.as_bytes();
      let size  = min(MAX as usize, min(self.1, bytes.len()));
      let mut r:usize = 0;
      r += try!(e.encode_varint(size as u64));
      r += try!(e.encode_array_u8(&bytes[0..size]));
      Ok(r)
   }
}

#[test]
fn test_limited_string() {
   use super::FixedBitcoinSerializer;
   let mut ser = FixedBitcoinSerializer::new(100);

   let s = "Hatsune Miku";
   assert_matches!(LimitedString(s, 7).encode(&mut ser), Ok(8));
   assert_matches!(LimitedString(s, 100).encode(&mut ser), Ok(13));
   assert_eq!(b"\x07Hatsune\x0CHatsune Miku", &ser.as_slice()[..21]);
}
