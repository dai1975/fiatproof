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


use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for MerkleBlock {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.encode(&(), e, ws));
      r += try!(self.txn.encode(&(), e, ws));
      Ok(r)
   }
}
impl BitcoinDecodee for MerkleBlock {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.decode(&(), d, rs));
      r += try!(self.txn.decode(&(), d, rs));
      Ok(r)
   }
}
