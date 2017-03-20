use std;
use ::MerkleBlock;

#[derive(Debug,Default,Clone)]
pub struct MerkleBlockMessage {
   pub block : MerkleBlock,
}

impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}


use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for MerkleBlockMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for MerkleBlockMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(d, ()));
      Ok(r)
   }
}
