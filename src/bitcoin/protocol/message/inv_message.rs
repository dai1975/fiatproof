use std;
use super::super::{ Inv };

#[derive(Debug,Default,Clone)]
pub struct InvMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for InvMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x69, 0x6e, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for InvMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "Inv(len={})", self.invs.len()),
         1 => write!(f, "Inv(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "Inv(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

use crate::iostream::{ WriteStream, ReadStream };
use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for InvMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += e.serialize_var_array(&(), ws, &self.invs[..], MAX_INV_SIZE)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for InvMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += d.deserialize_var_array(&(), rs, &mut self.invs, MAX_INV_SIZE)?;
      Ok(r)
   }
}
