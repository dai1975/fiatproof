use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockHeader {
   pub version: i32,
   pub hash_prev_block: UInt256,
   pub hash_merkle_root: UInt256,
   pub time: u32,
   pub bits: u32,
   pub nonce: u32,
}

impl ::std::fmt::Display for BlockHeader {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "BlockHeader(version={}, prev={}, merkle={}, time={}, bits={}, nonce={})",
             self.version, self.hash_prev_block, self.hash_merkle_root, self.time, self.bits, self.nonce)
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for BlockHeader {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.serialize_i32le(ws, self.version));
      r += try!(self.hash_prev_block.serialize(&(), e, ws));
      r += try!(self.hash_merkle_root.serialize(&(), e, ws));
      r += try!(e.serialize_u32le(ws, self.time));
      r += try!(e.serialize_u32le(ws, self.bits));
      r += try!(e.serialize_u32le(ws, self.nonce));
      Ok(r)
   }
}
impl BitcoinDeserializee for BlockHeader {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.deserialize_i32le(rs, &mut self.version));
      r += try!(self.hash_prev_block.deserialize(&(), d, rs));
      r += try!(self.hash_merkle_root.deserialize(&(), d, rs));
      r += try!(d.deserialize_u32le(rs, &mut self.time));
      r += try!(d.deserialize_u32le(rs, &mut self.bits));
      r += try!(d.deserialize_u32le(rs, &mut self.nonce));
      Ok(r)
   }
}
