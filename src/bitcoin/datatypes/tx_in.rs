use super::{UInt256, Script};

#[derive(Debug,Default,Clone,Eq,PartialEq,PartialOrd,Ord)]
pub struct TxOutPoint {
   pub txid: UInt256,
   pub n:    u32,
}
impl TxOutPoint {
   pub fn new_null() -> Self {
      TxOutPoint {
         txid: UInt256::new_null(),
         n: std::u32::MAX
      }
   }
   pub fn set_null(&mut self) {
      self.txid.set_null();
      self.n = std::u32::MAX;
   }
   pub fn is_null(&self) -> bool {
      self.n == std::u32::MAX && self.txid.is_null()
   }
}

#[derive(Debug,Default,Clone)]
pub struct TxIn {
   pub prevout:    TxOutPoint,
   pub script_sig: Script,
   pub sequence:   u32,
}


impl TxIn {
   // Sequence 型を作るべきか
   pub const SEQUENCE_FINAL:u32                 = 0xFFFFFFFFu32;
   pub const SEQUENCE_LOCKTIME_DISABLE_FLAG:u32 = (1 << 31);
   pub const SEQUENCE_LOCKTIME_TYPE_FLAG:u32    = (1 << 22);
   pub const SEQUENCE_LOCKTIME_MASK:u32         = 0x0000FFFFu32;
   pub const SEQUENCE_GRANULARITY:u32           = 9;
   pub fn is_sequence_final(&self) -> bool {
      self.sequence == Self::SEQUENCE_FINAL
   }
   pub fn is_locktime_enable(&self) -> bool {
      (self.sequence & Self::SEQUENCE_LOCKTIME_DISABLE_FLAG) == 0
   }
   pub fn is_locktime_type(&self) -> bool {
      (self.sequence & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0
   }
   pub fn compare_sequence_locktime(l:u32, r:u32) -> Option<bool> {
      let l_is_blocktime = (l & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      let r_is_blocktime = (r & Self::SEQUENCE_LOCKTIME_TYPE_FLAG) != 0;
      if l_is_blocktime ^ r_is_blocktime {
         None
      } else {
         Some((l & Self::SEQUENCE_LOCKTIME_MASK) > (r & Self::SEQUENCE_LOCKTIME_MASK))
      }
   }

   pub fn get_locktime_time(&self) -> Option<u64> {
      let v:u64 = ((self.sequence & Self::SEQUENCE_LOCKTIME_MASK) as u64) << Self::SEQUENCE_GRANULARITY;
      if v == 0 { None } else { Some(v-1) }
   }
   pub fn get_locktime_height(&self) -> Option<u32> {
      let v:u32 = self.sequence & Self::SEQUENCE_LOCKTIME_MASK;
      if v == 0 { None } else { Some(v-1) }
   }
}

impl std::fmt::Display for TxOutPoint {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "OutPoint(txid={}, n={})", self.txid, self.n)
   }
}
impl std::fmt::Display for TxIn {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxIn(prevout={}, sig={}, seq={})", self.prevout, self.script_sig, self.sequence)
   }
}


use crate::iostream::{ WriteStream, ReadStream };
use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for TxOutPoint {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.txid.serialize(&(), e, ws)?;
      r += e.serialize_u32le(ws, self.n)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for TxOutPoint {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.txid.deserialize(&(), d, rs)?;
      r += d.deserialize_u32le(rs, &mut self.n)?;
      Ok(r)
   }
}

impl BitcoinSerializee for TxIn {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.prevout.serialize(&(), e, ws)?;
      r += self.script_sig.serialize(&true, e, ws)?;
      r += e.serialize_u32le(ws, self.sequence)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for TxIn {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.prevout.deserialize(&(), d, rs)?;
      r += self.script_sig.deserialize(&None, d, rs)?;
      r += d.deserialize_u32le(rs, &mut self.sequence)?;
      Ok(r)
   }
}

