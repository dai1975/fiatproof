use std;
use crate::bitcoin::datatypes::MerkleBlock;

#[derive(Debug,Default,Clone)]
pub struct MerkleBlockMessage {
   pub block : MerkleBlock,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for MerkleBlockMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00];
}

impl std::fmt::Display for MerkleBlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MerkleBlock(block={})", self.block)
   }
}


use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for MerkleBlockMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.block.serialize(&(), e, ws)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for MerkleBlockMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.block.deserialize(&(), d, rs)?;
      Ok(r)
   }
}
