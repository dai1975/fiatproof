use std;
use crate::bitcoin::datatypes::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetBlocksMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for GetBlocksMessage {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.locator.serialize(&(), e, ws)?;
      r += self.hash_stop.serialize(&(), e, ws)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for GetBlocksMessage {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.locator.deserialize(&(), d, rs)?;
      r += self.hash_stop.deserialize(&(), d, rs)?;
      Ok(r)
   }
}
