use super::super::script::opcode::*;

#[derive(Debug,Default,Clone)]
pub struct Witness {
   pub data_vec: Vec<Box<[u8]>>,
}

impl Witness {
   pub fn new() -> Self {
      Self { data_vec: Vec::new() }
   }
   pub fn new_with(v:Vec<Box<[u8]>>) -> Self {
      Self { data_vec: v }
   }
   
   pub fn clear(&mut self) {
      self.data_vec.clear();
   }

   #[inline]
   pub fn data(&self, i:usize) -> &[u8] {
      self.data_vec[i].as_ref()
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for Witness {
   type P = (); //true -> add size prefix
   fn serialize<W: std::io::Write>(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_var_int(ws, self.data_vec.len() as u64)?;
      for i in 0..self.data_vec.len() {
         r += e.serialize_var_octets(ws, self.data_vec[i].as_ref(), std::usize::MAX)?;
      }
      Ok(r)
   }
}
impl BitcoinDeserializee for Witness {
   type P = (); //None -> add size prefix
   fn deserialize<R: std::io::Read>(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      let mut size = 0u64;
      r += d.deserialize_var_int(rs, &mut size)?;
      self.data_vec = Vec::with_capacity(size as usize);
      for _i in 0..size {
         let mut tmp = Vec::<u8>::new();
         r += d.deserialize_var_octets(rs, &mut tmp, std::usize::MAX)?;
         self.data_vec.push(tmp.into_boxed_slice());
      }
      Ok(r)
   }
}

impl std::fmt::Display for Witness {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match crate::ui::bitcoin::serialize(self, &()).map(|b| crate::utils::b2h(b)) {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(_) => f.write_fmt(format_args!("err")),
      }
   }
}



#[test]
fn test_deserialize_script() {
   use super::{WItness};

   /*
   let hexstring = "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab";
   let hexbytes  = crate::utils::h2b(hexstring).unwrap();
   
   let script = crate::ui::bitcoin::hex_to_script(hexstring).unwrap();

   assert_eq!(hexbytes.as_ref(), script.bytecode.as_ref());
    */
}

   
