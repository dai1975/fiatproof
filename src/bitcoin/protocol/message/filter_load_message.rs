use std;

#[derive(Debug,Default,Clone)]
pub struct FilterLoadMessage {
   pub data: Vec<u8>,
   pub hash_funcs: u32,
   pub tweak: u32,
   pub flags: u8,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for FilterLoadMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x00, 0x00];
}

impl std::fmt::Display for FilterLoadMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterLoad(data={:?},funcs={},tweak={},flags={})", self.data, self.hash_funcs, self.tweak, self.flags)
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for FilterLoadMessage {
   type P = ();
   fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_octets(ws, &self.data[..])?;
      r += e.serialize_u32le(ws, self.hash_funcs)?;
      r += e.serialize_u32le(ws, self.tweak)?;
      r += e.serialize_u8(ws, self.flags)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for FilterLoadMessage {
   type P = ();
   fn deserialize<R: std::io::Read +?Sized>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_octets(rs, &mut self.data)?;
      r += d.deserialize_u32le(rs, &mut self.hash_funcs)?;
      r += d.deserialize_u32le(rs, &mut self.tweak)?;
      r += d.deserialize_u8(rs, &mut self.flags)?;
      Ok(r)
   }
}
