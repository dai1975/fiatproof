use super::{BlockHeader, Tx};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub txs: Vec<Tx>,
   pub checked: bool,
}

impl std::fmt::Display for Block {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.txs.len())
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for Block {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.header.serialize(&(), e, ws)?;
      r += e.serialize_var_array(&(), ws, &self.txs, std::usize::MAX)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for Block {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += self.header.deserialize(&(), d, rs)?;
      r += d.deserialize_var_array(&(), rs, &mut self.txs, std::usize::MAX)?;
      Ok(r)
   }
}
