use std::borrow::Borrow;
use crate::bitcoin::serialize::{Medium, Deserializer, Deserializee};
use crate::bitcoin::datatypes::{UInt256, Tx, Script};

pub struct DeserializerBuilder {
   version: i32,
   medium: Medium,
   enable_segwit: bool,
}
impl DeserializerBuilder {
   pub fn new() -> Self {
      DeserializerBuilder {
         version: 0,
         medium: Medium::default(),
         enable_segwit: true,
      }
   }
   pub fn build(self) -> Deserializer {
      Deserializer {
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



pub fn deserialize<I: Borrow<[u8]>, D: Deserializee>(input: I, param:&D::P, ret: &mut D) -> crate::Result<usize> {
   let mut rs = input.borrow();
   let dec = DeserializerBuilder::new().medium("net").build();
   ret.deserialize(param, &dec, &mut rs)
}
   
pub fn hex_to_uint256(h: &str) -> crate::Result<UInt256> {
   let mut ret = UInt256::default();
   let b = crate::utils::h2b_rev(h)?;
   let _ = deserialize(b.as_ref(), &(), &mut ret)?;
   Ok(ret)
}

pub fn hex_to_tx(h: &str) -> crate::Result<Tx> {
   let mut ret = Tx::default();
   let b = crate::utils::h2b(h)?;
   let _ = deserialize(b.as_ref(), &(), &mut ret)?;
   Ok(ret)
}

pub fn hex_to_script(h: &str) -> crate::Result<Script> {
   let b = crate::utils::h2b(h)?;
   let mut ret = Script::new_null();
   let _ = deserialize(b.as_ref(), &Some(b.len()), &mut ret)?;
   Ok(ret)
}

