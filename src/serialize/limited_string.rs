use ::Error;
use super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee, SerializeError};

pub struct LimitedString<T:Default>(pub T, pub usize);

impl <T:Default> LimitedString<T> {
   pub fn new(s:usize) -> Self { LimitedString(T::default(), s) }
}

impl <'a, E:BitcoinEncoder> BitcoinEncodee<E> for LimitedString<&'a str> {
   fn encode(&self, e:&mut E) -> Result<usize, Error> {
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

impl <D:BitcoinDecoder> BitcoinDecodee<D> for LimitedString<String> {
   fn decode(&mut self, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;

      use std::u32::MAX;
      let mut len:u64 = 0;
      r += try!(d.decode_varint(&mut len));
      let len = len as usize;
      if self.1 < len || (MAX as usize) < len { serialize_error!("string is too long") }

      let mut v = vec![0u8; len];
      r += try!(d.decode_array_u8(v.as_mut_slice()));
      self.0 = try!(String::from_utf8(v));

      Ok(r)
   }
}

#[test]
fn test_encode_string() {
   use super::FixedBitcoinSerializer;
   let mut ser = FixedBitcoinSerializer::new(100);

   let s = "Hatsune Miku";
   assert_matches!(LimitedString(s, 7).encode(&mut ser), Ok(8));
   assert_matches!(LimitedString(s, 100).encode(&mut ser), Ok(13));
   assert_eq!(b"\x07Hatsune\x0CHatsune Miku", &ser.as_slice()[..21]);
}

#[test]
fn test_decode_string() {
   use super::SliceBitcoinDeserializer;

   let data:&[u8] = b"\x0CHatsune Miku";
   let mut des = SliceBitcoinDeserializer::new(data);

   let mut v = LimitedString::<String>::new(100);
   assert_matches!(des.decode(&mut v), Ok(13));
   assert_eq!("Hatsune Miku", v.0);

   des.rewind();
   v.1 = 7;
   assert_matches!(des.decode(&mut v), Err(_));
}
