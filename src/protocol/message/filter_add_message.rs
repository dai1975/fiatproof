use std;

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}

impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for FilterAddMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      Ok(r)
   }
}
impl Decodee for FilterAddMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      Ok(r)
   }
}
