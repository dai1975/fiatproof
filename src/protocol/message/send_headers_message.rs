use std;

#[derive(Debug,Default,Clone)]
pub struct SendHeadersMessage;

impl std::fmt::Display for SendHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "SendHeaders()")
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for SendHeadersMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for SendHeadersMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
