use std;
use crate::bitcoin::datatypes::UInt256;
use super::super::{ Inv, InvType };

#[derive(Debug,Default,Clone)]
pub struct GetDataMessage {
   pub invs : Vec<Inv>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for GetDataMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x67, 0x65, 0x74, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl GetDataMessage {
   pub fn new(invtype:InvType, hash: UInt256) -> Self {
      GetDataMessage {
         invs: vec![ Inv::new(invtype, hash) ]
      }
   }
   pub fn new_tx(hash: UInt256)           -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)        -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filter_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}

impl std::fmt::Display for GetDataMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.invs.len() {
         0 => write!(f, "GetData(len={})", self.invs.len()),
         1 => write!(f, "GetData(len={}, 0={})", self.invs.len(), self.invs[0]),
         l => write!(f, "GetData(len={}, 0={}, ...{})", self.invs.len(), self.invs[0], self.invs[l-1])
      }
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for GetDataMessage {
   type P = ();
   fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += e.serialize_var_array(&(), ws, &self.invs[..], MAX_INV_SIZE)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for GetDataMessage {
   type P = ();
   fn deserialize<R: std::io::Read +?Sized>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      use super::super::apriori::MAX_INV_SIZE;
      r += d.deserialize_var_array(&(), rs, &mut self.invs, MAX_INV_SIZE)?;
      Ok(r)
   }
}
