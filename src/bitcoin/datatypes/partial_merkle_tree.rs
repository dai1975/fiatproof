use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct PartialMerkleTree {
   pub n_transactions: u32,
   pub bits: bit_vec::BitVec,
   pub hashes: Vec<UInt256>,
}

impl std::fmt::Display for PartialMerkleTree {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "PartialMerkleTree(n={}, bits={:?}, hash={:?})", self.n_transactions, self.bits, self.hashes)
   }
}


use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};

macro_rules! reverse_u8 {
   ($exp:expr) => {{
      let x:u8 = $exp;
      let x:u8 = ((x & 0x55) << 1) | ((x & 0xAA) >> 1);
      let x:u8 = ((x & 0x33) << 2) | ((x & 0xCC) >> 2);
      let x:u8 = (x << 4) | (x >> 4);
      x
   }}
}

impl BitcoinSerializee for PartialMerkleTree {
   type P = ();
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_u32le(ws, self.n_transactions)?;
      {
         let mut bytes:Vec<u8> = self.bits.to_bytes();
         for byte in &mut bytes {
            *byte = reverse_u8!(*byte);
         }
         r += e.serialize_var_octets(ws, &bytes, std::usize::MAX)?;
      }
      r += e.serialize_var_array(&(), ws, &self.hashes, std::usize::MAX)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for PartialMerkleTree {
   type P = ();
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_u32le(rs, &mut self.n_transactions)?;
      {
         let mut bytes:Vec<u8> = Vec::new();
         r += d.deserialize_var_octets(rs, &mut bytes, std::usize::MAX)?;

         for byte in bytes.iter_mut() {
            *byte = reverse_u8!(*byte);
         }
         self.bits = bit_vec::BitVec::from_bytes(bytes.as_slice());
      }
      r += d.deserialize_var_array(&(), rs, &mut self.hashes, std::usize::MAX)?;
      Ok(r)
   }
}
