use std;
use ::Block;

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for BlockMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for BlockMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(d, ()));
      Ok(r)
   }
}
