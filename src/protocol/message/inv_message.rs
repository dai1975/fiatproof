use std;
use super::super::{ Inv };

#[derive(Debug,Default,Clone)]
pub struct InvMessage {
   pub invs : Vec<Inv>,
}

impl std::fmt::Display for InvMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "Inv(len={})", self.invs.len()),
         1 => write!(f, "Inv(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "Inv(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for InvMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.encode(e, (MAX_INV_SIZE,())));
      Ok(r)
   }
}
impl Decodee for InvMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.decode(d, (MAX_INV_SIZE,())));
      Ok(r)
   }
}
