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


use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for MerkleBlock {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.serialize(&(), e, ws));
      r += try!(self.txn.serialize(&(), e, ws));
      Ok(r)
   }
}
impl BitcoinDeserializee for MerkleBlock {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.deserialize(&(), d, rs));
      r += try!(self.txn.deserialize(&(), d, rs));
      Ok(r)
   }
}
