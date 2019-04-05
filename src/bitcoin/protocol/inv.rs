use std;
use ::bitcoin::datatypes::UInt256;

#[derive(Debug,Clone,PartialEq)]
pub enum InvType {
   Unknown       = 0,
   Tx            = 1,
   Block         = 2,
   FilteredBlock = 3,
}

impl Default for InvType {
   fn default() -> Self { InvType::Unknown }
}

impl std::fmt::Display for InvType {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match *self {
         InvType::Tx            => write!(f, "tx"),
         InvType::Block         => write!(f, "block"),
         InvType::FilteredBlock => write!(f, "filtered"),
         _ => write!(f, "{}", *self),
      }
   }
}
impl InvType {
   pub fn is_tx(&self)             -> bool { *self == InvType::Tx }
   pub fn is_block(&self)          -> bool { *self == InvType::Block }
   pub fn is_filtered_block(&self) -> bool { *self == InvType::FilteredBlock }
}


#[derive(Debug,Clone,Default)]
pub struct Inv {
   pub invtype: InvType,
   pub hash:    UInt256,
}
impl std::fmt::Display for Inv {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}:{}", self.invtype, self.hash)
   }
}
impl Inv {
   #[allow(dead_code)]
   pub fn new(invtype_:InvType, hash_: UInt256) -> Self {
      Inv {
         invtype: invtype_,
         hash:    hash_,
      }
   }
   pub fn new_tx(hash: UInt256)             -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)          -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filtered_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for InvType {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => raise_serialize_error!("malformed inv type"),
      };
      e.serialize_u32le(ws, tmp)
   }
}
impl BitcoinDeserializee for InvType {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += d.deserialize_u32le(rs, &mut tmp)?;
      *self = match tmp {
         1 => InvType::Tx,
         2 => InvType::Block,
         3 => InvType::FilteredBlock,
         _ => raise_serialize_error!("unexpected inv value"),
      };
      Ok(r)
   }
}

impl BitcoinSerializee for Inv {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += self.invtype.serialize(&(), e, ws)?;
      r += self.hash.serialize(&(), e, ws)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for Inv {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += self.invtype.deserialize(&(), d, rs)?;
      r += self.hash.deserialize(&(), d, rs)?;
      Ok(r)
   }
}
