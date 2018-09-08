use ::std::borrow::Borrow;
use ::iostream::{ReadStream, SliceReadStream};
use ::bitcoin::serialize::{Medium, Deserializer, Deserializee};
use ::bitcoin::datatypes::{UInt256, Tx, Script};

pub fn deserialize<I: Borrow<[u8]>, D: Deserializee>(input: I, param:&D::P, ret: &mut D) -> ::Result<usize> {
   let mut rs = SliceReadStream::new(input);
   let med = Medium::new("net").unwrap();
   let dec = Deserializer::new(&med);
   ret.deserialize(param, &dec, &mut rs)
}
   
pub fn hex_to_uint256(h: &str) -> ::Result<UInt256> {
   let mut ret = UInt256::default();
   let b = ::utils::h2b_rev(h)?;
   let _ = deserialize(b.as_ref(), &(), &mut ret)?;
   Ok(ret)
}

pub fn hex_to_tx(h: &str) -> ::Result<Tx> {
   let mut ret = Tx::default();
   let b = ::utils::h2b(h)?;
   let _ = deserialize(b.as_ref(), &(), &mut ret)?;
   Ok(ret)
}

pub fn hex_to_script(h: &str) -> ::Result<Script> {
   let b = ::utils::h2b(h)?;
   let mut ret = Script::new_null();
   let _ = deserialize(b.as_ref(), &Some(b.len()), &mut ret)?;
   Ok(ret)
}

