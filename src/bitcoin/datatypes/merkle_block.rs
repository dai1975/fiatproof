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


use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for MerkleBlock {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(e));
      r += try!(self.txn.encode(e));
      Ok(r)
   }
}
impl BitcoinDecodee for MerkleBlock {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(d));
      r += try!(self.txn.decode(d));
      Ok(r)
   }
}
