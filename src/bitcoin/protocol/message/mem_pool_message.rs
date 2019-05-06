use std;

#[derive(Debug,Default,Clone)]
pub struct MemPoolMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for MemPoolMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6d, 0x65, 0x6d, 0x70, 0x6f, 0x6f, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for MemPoolMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MemPool()")
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for MemPoolMessage {
   type P = ();
   fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, _e:&BitcoinSerializer, _ws:&mut W) -> crate::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDeserializee for MemPoolMessage {
   type P = ();
   fn deserialize<R: std::io::Read +?Sized>(&mut self, _p:&Self::P, _d:&BitcoinDeserializer, _rs:&mut R) -> crate::Result<usize> {
      Ok(0usize)
   }
}
