use crate::bitcoin::serialize::{Medium, Serializer, Serializee};
use crate::bitcoin::datatypes::{UInt256, Tx, Script};

pub fn serialize<T: Serializee>(data: &T, param:&T::P) -> crate::Result<Box<[u8]>> {
   let mut ws = Vec::<u8>::new();
   let med = Medium::new("net")?;
   let enc = Serializer::new(&med);
   let _size = data.serialize(param, &enc, &mut ws)?;
   Ok(ws.into_boxed_slice())
}
pub fn serialize_dhash256<T: Serializee>(data: &T, param:&T::P) -> crate::Result<[u8;32]> {
   use crate::crypto::digest::{ DigestWrite, DHash256 };
   let mut ws = DigestWrite::new(DHash256::new());
   let med = Medium::new("net")?;
   let enc = Serializer::new(&med);
   let _size = data.serialize(param, &enc, &mut ws)?;
   
   let out = [0u8;32];
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
   let b = serialize_dhash256(data, &())?;
   Ok(b)
}
pub fn tx_to_txid_hex(data: &Tx) -> crate::Result<String> {
   let b = tx_to_txid(data)?;
   let h = crate::utils::b2h(&b[..]);
   Ok(h)
}


