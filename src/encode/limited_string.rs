use ::std::borrow::Borrow;
use super::{Encodee, EncodeStream, Decodee, DecodeStream};

impl <'a> Encodee for &'a str {
   type P = usize;
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize> {
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
impl Encodee for String {
   type P = usize;
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize> {
      self.as_str().encode(e,p)
   }
}

impl Decodee for String {
   type P = usize;
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, p:BP) -> ::Result<usize> {
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
   use super::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());

   let s = "Hatsune Miku";
   assert_matches!(s.encode(&mut e, 7), Ok(8));
   assert_matches!(s.encode(&mut e, 100), Ok(13));
   assert_eq!(b"\x07Hatsune\x0CHatsune Miku", &e.w.get_ref()[..21]);
}

#[test]
fn test_decode_string() {
   use super::{BitcoinDecodeStream, SliceReadStream, Media};
   let data:&[u8] = b"\x0CHatsune Miku";
   let mut d = BitcoinDecodeStream::new(SliceReadStream::new(data), Media::default().set_net());

   let mut s = String::default();
   assert_matches!(s.decode(&mut d, 100), Ok(13));
   assert_eq!("Hatsune Miku", s);

   d.r.rewind();
   assert_matches!(s.decode(&mut d, 7), Err(_));
}
