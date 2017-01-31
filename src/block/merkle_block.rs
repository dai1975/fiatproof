use super::{BlockHeader, PartialMerkleTree};

#[derive(Debug,Default,Clone)]
pub struct MerkleBlock {
   pub header: BlockHeader,
   pub txn:    PartialMerkleTree,
}

impl ::std::fmt::Display for MerkleBlock {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "MerkleBlock(header={}, txn={})", self.header, self.txn)
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};

impl Encodee for MerkleBlock {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(e, ()));
      r += try!(self.txn.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for MerkleBlock {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(d, ()));
      r += try!(self.txn.decode(d, ()));
      Ok(r)
   }
}
