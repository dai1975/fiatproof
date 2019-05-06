use super::{Script};

pub type Amount = i64;

#[allow(dead_code)] const COIN:Amount = 100000000;
#[allow(dead_code)] const CENT:Amount = 1000000;
#[allow(dead_code)] const MAX_MONEY:Amount = 21000000 * COIN;

#[derive(Debug,Default,Clone)]
pub struct TxOut {
   pub value:         Amount,
   pub script_pubkey: Script,
}

impl TxOut {
   pub fn new_null() -> TxOut {
      TxOut {
         value: -1,
         script_pubkey: Script::new_null(),
      }
   }
   pub fn set_null(&mut self) {
      self.value = -1;
      self.script_pubkey.set_null();
   }
   pub fn is_null(&self) -> bool {
      self.value == -1
   }
}

impl std::fmt::Display for TxOut {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "TxOut(val={}, pubkey={})", self.value, self.script_pubkey)
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for TxOut {
   type P = ();
   fn serialize<W: std::io::Write +?Sized>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_i64le(ws, self.value)?;
      r += self.script_pubkey.serialize(&true, e, ws)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for TxOut {
   type P = ();
   fn deserialize<R: std::io::Read +?Sized>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_i64le(rs, &mut self.value)?;
      r += self.script_pubkey.deserialize(&None, d, rs)?;
      Ok(r)
   }
}
