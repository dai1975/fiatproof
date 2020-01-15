use crate::bitcoin::serialize::{Medium, Serializer, Serializee};
use crate::bitcoin::datatypes::{UInt256, Tx, Script};

pub struct SerializerBuilder {
   version: i32,
   medium: Medium,
   enable_segwit: bool,
}
impl SerializerBuilder {
   pub fn new() -> Self {
      SerializerBuilder {
         version: 0,
         medium: Medium::default(),
         enable_segwit: true,
      }
   }
   pub fn build(self) -> Serializer {
      Serializer {
         version: self.version,
         medium: self.medium,
         enable_segwit: self.enable_segwit,
      }
   }
   pub fn version(self, v:i32) -> Self {
      Self {
         version: v,
         medium: self.medium,
         enable_segwit: self.enable_segwit,
      }
   }
   pub fn medium<T: std::convert::Into<Medium>>(self, m: T) -> Self {
      Self {
         version: self.version,
         medium: m.into(),
         enable_segwit: self.enable_segwit,
      }
   }
   pub fn segwit(self, v: bool) -> Self {
      Self {
         version: self.version,
         medium: self.medium,
         enable_segwit: v,
      }
   }
}


pub fn serialize<T: Serializee>(data: &T, param:&T::P) -> crate::Result<Box<[u8]>> {
   let mut ws = Vec::<u8>::new();
   let enc = SerializerBuilder::new().medium("net").build();
   let _size = data.serialize(param, &enc, &mut ws)?;
   Ok(ws.into_boxed_slice())
}
pub fn serialize_dhash256<T: Serializee>(data: &T, param:&T::P, segwit:bool) -> crate::Result<[u8;32]> {
   use crate::crypto::digest::{ DigestWrite, DHash256 };
   let mut ws = DigestWrite::new(DHash256::new());
   let enc = SerializerBuilder::new().medium("net").segwit(segwit).build();
   let _size = data.serialize(param, &enc, &mut ws)?;
   
   use crypto::digest::Digest;
   let mut out = [0u8;32];
   ws.result(&mut out);
   Ok(out)
}

pub fn uint256_to_hex(data: &UInt256) -> crate::Result<String> {
   let b = serialize(data, &())?;
   let h = crate::utils::b2h_rev(b);
   Ok(h)
}
pub fn script_to_hex(data: &Script) -> crate::Result<String> {
   let b = serialize(data, &false)?;
   let h = crate::utils::b2h(b);
   Ok(h)
}
pub fn tx_to_hex(data: &Tx) -> crate::Result<String> {
   let b = serialize(data, &())?;
   let h = crate::utils::b2h(b);
   Ok(h)
}
pub fn tx_to_txid(data: &Tx) -> crate::Result<[u8;32]> {
   let b = serialize_dhash256(data, &(), false)?;
   Ok(b)
}
pub fn tx_to_txid_uint256(data: &Tx) -> crate::Result<UInt256> {
   let b = serialize_dhash256(data, &(), false)?;
   Ok(UInt256::new(&b))
}
pub fn tx_to_txid_hex(data: &Tx) -> crate::Result<String> {
   let b = tx_to_txid(data)?;
   let h = crate::utils::b2h_rev(&b[..]);
   Ok(h)
}
pub fn tx_to_wtxid(data: &Tx) -> crate::Result<[u8;32]> {
   let b = serialize_dhash256(data, &(), true)?;
   Ok(b)
}
pub fn tx_to_wtxid_hex(data: &Tx) -> crate::Result<String> {
   let b = tx_to_wtxid(data)?;
   let h = crate::utils::b2h(&b[..]);
   Ok(h)
}


