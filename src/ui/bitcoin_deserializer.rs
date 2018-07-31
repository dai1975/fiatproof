use ::std::borrow::Borrow;
use ::serialize::{ReadStream, SliceReadStream};
use ::bitcoin::encode::{Medium, Decoder, Decodee};
use ::bitcoin::datatypes::{UInt256, Tx, Script};

pub struct BitcoinDeserializer { }

impl BitcoinDeserializer {
   pub fn deserialize<I: Borrow<[u8]>, D: Decodee>(input: I, param:&D::P, ret: &mut D) -> ::Result<usize> {
      let mut rs = SliceReadStream::new(input);
      let med = Medium::new("net").unwrap();
      let dec = Decoder::new(&med);
      ret.decode(param, &dec, &mut rs)
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
      let mut ret = Script::default();
      let b = ::utils::h2b(h)?;
      let _ = BitcoinDeserializer::deserialize(b.as_slice(), &false, &mut ret)?;
      Ok(ret)
   }
}

