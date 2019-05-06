use std::borrow::Borrow;
use crate::bitcoin::serialize::{Medium, Deserializer, Deserializee};
use crate::bitcoin::datatypes::{UInt256, Tx, Script};

pub fn deserialize<I: Borrow<[u8]>, D: Deserializee>(input: I, param:&D::P, ret: &mut D) -> crate::Result<usize> {
   let mut rs = input.borrow();
   let med = Medium::new("net").unwrap();
   let dec = Deserializer::new(&med);
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

