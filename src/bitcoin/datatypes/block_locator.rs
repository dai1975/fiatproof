use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}

use crate::iostream::{ WriteStream, ReadStream };
use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for BlockLocator {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      if !e.medium().is_hash() {
         let v:i32 = e.medium().version();
         r += e.serialize_i32le(ws, v)?;
      }
      r += e.serialize_var_array(&(), ws, &self.haves, std::usize::MAX)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for BlockLocator {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      if !d.medium().is_hash() {
         let mut v:i32 = 0;
         r += d.deserialize_i32le(rs, &mut v)?;
      }
      r += d.deserialize_var_array(&(), rs, &mut self.haves, std::usize::MAX)?;
      Ok(r)
   }
}

