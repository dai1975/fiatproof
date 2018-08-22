use ::std::borrow::Borrow;
use ::iostream::{ReadStream, SliceReadStream};
use ::bitcoin::serialize::{Medium, Deserializer, Deserializee};
use ::bitcoin::datatypes::{UInt256, Tx, Script};

pub struct BitcoinDeserializer { }

impl BitcoinDeserializer {
   pub fn deserialize<I: Borrow<[u8]>, D: Deserializee>(input: I, param:&D::P, ret: &mut D) -> ::Result<usize> {
      let mut rs = SliceReadStream::new(input);
      let med = Medium::new("net").unwrap();
      let dec = Deserializer::new(&med);
      ret.deserialize(param, &dec, &mut rs)
   }
   
   pub fn hex_to_uint256(h: &str) -> ::Result<UInt256> {
      let mut ret = UInt256::default();
      let b = ::utils::h2b_rev(h)?;
      let _ = BitcoinDeserializer::deserialize(b.as_slice(), &(), &mut ret)?;
      Ok(ret)
   }

   pub fn hex_to_tx(h: &str) -> ::Result<Tx> {
      let mut ret = Tx::default();
      let b = ::utils::h2b(h)?;
      let _ = BitcoinDeserializer::deserialize(b.as_slice(), &(), &mut ret)?;
      Ok(ret)
   }

   pub fn hex_to_script(h: &str) -> ::Result<Script> {
      let b = ::utils::h2b(h)?;
      let mut ret = {
         let mut v = Vec::<u8>::with_capacity(b.len());
         v.resize(b.len(), 0);
         Script::new(v)
      };
      let _ = BitcoinDeserializer::deserialize(b.as_slice(), &false, &mut ret)?;
      Ok(ret)
   }
}

