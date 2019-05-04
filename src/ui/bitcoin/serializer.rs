use crate::bitcoin::serialize::{Medium, Serializer, Serializee};
use crate::bitcoin::datatypes::{UInt256, Tx, Script};

pub fn serialize<T: Serializee>(data: &T, param:&T::P) -> crate::Result<Box<[u8]>> {
   let mut ws = Vec::<u8>::new();
   let med = Medium::new("net")?;
   let enc = Serializer::new(&med);
   let _size = data.serialize(param, &enc, &mut ws)?;
   Ok(ws.into_boxed_slice())
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
pub fn tx_to_txid(data: &Tx) -> crate::Result<String> {
   let b = serialize(data, &())?;
   let h = crate::ui::create_dhash256().u8_to_hex_rev(b);
   Ok(h)
}


