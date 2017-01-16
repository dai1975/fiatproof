use ::std::borrow::Borrow;
use ::Error;
use super::{Encoder, Encodee, Decoder, Decodee};

impl <'a,E:Encoder> Encodee<E,usize> for &'a str {
   fn encode<BP:Borrow<usize>+Sized>(&self, p:BP, e:&mut E) -> Result<usize, Error> {
      use std::cmp::min;
      use std::u32::MAX;
      let bytes = self.as_bytes();
      let size  = min(MAX as usize, min(*p.borrow(), bytes.len()));
      let mut r:usize = 0;
      r += try!(e.encode_varint(size as u64));
      r += try!(e.encode_array_u8(&bytes[0..size]));
      Ok(r)
   }
}
impl <E:Encoder> Encodee<E,usize> for String {
   fn encode<BP:Borrow<usize>+Sized>(&self, p:BP, e:&mut E) -> Result<usize, Error> {
      self.as_str().encode(p,e)
   }
}

impl <D:Decoder> Decodee<D,usize> for String {
   fn decode<BP:Borrow<usize>+Sized>(&mut self, p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;

      use std::u32::MAX;
      let lim:usize = ::std::cmp::min(*p.borrow(), MAX as usize);
      let mut len:u64 = 0;
      r += try!(d.decode_varint(&mut len));
      let len = len as usize;
      if lim < len { encode_error!("string is too long") }

      let mut v = vec![0u8; len];
      r += try!(d.decode_array_u8(v.as_mut_slice()));
      *self = try!(String::from_utf8(v));

      Ok(r)
   }
}

#[test]
fn test_encode_string() {
   use super::FixedEncodeStream;
   let mut ser = FixedEncodeStream::new(100);

   let s = "Hatsune Miku";
   assert_matches!(s.encode(7, &mut ser), Ok(8));
   assert_matches!(s.encode(100, &mut ser), Ok(13));
   assert_eq!(b"\x07Hatsune\x0CHatsune Miku", &ser.as_slice()[..21]);
}

#[test]
fn test_decode_string() {
   use super::SliceDecodeStream;

   let data:&[u8] = b"\x0CHatsune Miku";
   let mut des = SliceDecodeStream::new(data);

   let mut s = String::default();
   assert_matches!(s.decode(100, &mut des), Ok(13));
   assert_eq!("Hatsune Miku", s);

   des.rewind();
   assert_matches!(s.decode(7, &mut des), Err(_));
}
