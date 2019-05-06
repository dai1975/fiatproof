use std;
use super::super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for NotFoundMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={}, 0={}", self.invs.len(), self.invs[0])
   }
}


use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for NotFoundMessage {
   type P = ();
   fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_var_array(&(), ws, &self.invs[..], std::usize::MAX)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for NotFoundMessage {
   type P = ();
   fn deserialize<R: std::io::Read +?Sized>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_var_array(&(), rs, &mut self.invs, std::usize::MAX)?;
      Ok(r)
   }
}
