use ::iostream::{WriteStream, VecWriteStream};
use ::bitcoin::serialize::{Medium, Serializer, Serializee};
use ::bitcoin::datatypes::{UInt256, Tx, Script};

pub struct BitcoinSerializer { }

impl BitcoinSerializer {
   pub fn serialize<T: Serializee>(data: &T, param:&T::P) -> ::Result<Box<[u8]>> {
      let mut ws = VecWriteStream::default();
      let med = Medium::new("net")?;
      let enc = Serializer::new(&med);
      let _size = data.serialize(param, &enc, &mut ws)?;
      Ok(ws.into_inner().into_boxed_slice())
   }

   pub fn uint256_to_hex(data: &UInt256) -> ::Result<String> {
      let b = BitcoinSerializer::serialize(data, &())?;
      let h = ::utils::b2h_rev(b);
      Ok(h)
   }
   pub fn script_to_hex(data: &Script) -> ::Result<String> {
      let b = BitcoinSerializer::serialize(data, &false)?;
      let h = ::utils::b2h(b);
      Ok(h)
   }
   pub fn tx_to_hex(data: &Tx) -> ::Result<String> {
      let b = BitcoinSerializer::serialize(data, &())?;
      let h = ::utils::b2h(b);
      Ok(h)
   }
   pub fn tx_to_txid(data: &Tx) -> ::Result<String> {
      let b = ::ui::BitcoinSerializer::serialize(data, &())?;
      let h = ::ui::DIGEST.create_dhash256().u8_to_hex_rev(b);
      Ok(h)
   }
   
}

