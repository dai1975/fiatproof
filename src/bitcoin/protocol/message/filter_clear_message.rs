use std;

#[derive(Debug,Default,Clone)]
pub struct FilterClearMessage;

use super::message::{ Message, COMMAND_LENGTH };
impl Message for FilterClearMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x00];
}

impl std::fmt::Display for FilterClearMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterClear()")
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for FilterClearMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, _e:&BitcoinSerializer, _ws:&mut WriteStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl BitcoinDeserializee for FilterClearMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, _d:&BitcoinDeserializer, _rs:&mut ReadStream) -> ::Result<usize> {
      Ok(0usize)
   }
}
