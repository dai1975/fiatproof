use std;

#[derive(Debug,Default,Clone)]
pub struct FilterLoadMessage {
   pub data: Vec<u8>,
   pub hash_funcs: u32,
   pub tweak: u32,
   pub flags: u8,
}

impl std::fmt::Display for FilterLoadMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterLoad(data={:?},funcs={},tweak={},flags={})", self.data, self.hash_funcs, self.tweak, self.flags)
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for FilterLoadMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      r += try!(e.encode_u32le(self.hash_funcs));
      r += try!(e.encode_u32le(self.tweak));
      r += try!(e.encode_u8(self.flags));
      Ok(r)
   }
}
impl Decodee for FilterLoadMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      r += try!(d.decode_u32le(&mut self.hash_funcs));
      r += try!(d.decode_u32le(&mut self.tweak));
      r += try!(d.decode_u8(&mut self.flags));
      Ok(r)
   }
}
