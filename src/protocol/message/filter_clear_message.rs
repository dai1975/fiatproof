use std;

#[derive(Debug,Default,Clone)]
pub struct FilterClearMessage;

impl std::fmt::Display for FilterClearMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterClear()")
   }
}

use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for FilterClearMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for FilterClearMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
