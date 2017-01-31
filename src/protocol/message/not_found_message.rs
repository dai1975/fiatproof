use std;
use super::super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={}, 0={}", self.invs.len(), self.invs[0])
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for NotFoundMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invs.encode(e, (::std::usize::MAX,())));
      Ok(r)
   }
}
impl Decodee for NotFoundMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invs.decode(d, (::std::usize::MAX,())));
      Ok(r)
   }
}
