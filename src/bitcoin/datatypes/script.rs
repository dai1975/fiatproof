use super::super::script::opcode::*;

// specify specific byte sequence as special scripts
#[inline]
pub fn parse_pay_to_script_hash(bytecode: &[u8]) -> Option<&[u8]> {
   if bytecode.len() == 23
      && bytecode[0] == OP_HASH160
      && bytecode[1] == 0x14
      && bytecode[22] == OP_EQUAL
   {
      return Some(&bytecode[2..22]);
   }
   None
}

#[inline]
pub fn parse_witness_script(bytecode0: &[u8], enable_p2sh:bool) -> Option<(u8,&[u8])> {
   // parse redeem script
   let bytecode = if !enable_p2sh {
      bytecode0
   } else if let Some(redeem) = parse_pay_to_script_hash(bytecode0) {
      redeem
   } else {
      bytecode0
   };
   
   if bytecode.len() < 4 { return None; }
   if 42 < bytecode.len() { return None; }
   let version = match bytecode[0] {
      OP_0 => 0u8,
      OP_1 ... OP_16 => (bytecode[0] - OP_1 + 1) as u8,
      _ => return None,
   };
   let len = match bytecode[2] {
      OP_PUSHDATAFIX_02 ... OP_PUSHDATAFIX_28 => (bytecode[2] - OP_PUSHDATAFIX_02 + 2) as usize,
      _ => return None,
   };
   
   if bytecode.len() < 2+len { return None; }
   let data = &bytecode[2 .. 2+len];
   Some((version, data))
}

#[derive(Debug,Default,Clone)]
pub struct Script {
   pub bytecode: Box<[u8]>,
}

impl Script {
   pub fn new<T:Into<Box<[u8]>>>(v:T) -> Self {
      Script { bytecode: v.into() }
   }
   pub fn new_null() -> Self {
      Script { bytecode: Vec::new().into_boxed_slice() }
   }
   
   pub fn set_null(&mut self) {
      self.bytecode = Vec::new().into_boxed_slice();
   }

   #[inline]
   pub fn bytecode(&self) -> &[u8] {
      self.bytecode.as_ref()
   }

   #[inline]
   pub fn parse_pay_to_script_hash(&self) -> Option<&[u8]> {
      parse_pay_to_script_hash(self.bytecode.as_ref())
   }
   #[inline]
   pub fn parse_witness_script(&self, enable_p2sh:bool) -> Option<(u8, &[u8])> {
      parse_witness_script(self.bytecode.as_ref(), enable_p2sh)
   }
}

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for Script {
   type P = bool; //true -> add size prefix
   fn serialize<W: std::io::Write>(&self, p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      if *p {
         e.serialize_var_octets(ws, &self.bytecode[..], std::usize::MAX)
      } else {
         e.serialize_octets(ws, &self.bytecode[..])
      }
   }
}
impl BitcoinDeserializee for Script {
   type P = Option<usize>; //None -> add size prefix
   fn deserialize<R: std::io::Read>(&mut self, p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      match *p {
         None => {
            let mut tmp = Vec::<u8>::new();
            let size = d.deserialize_var_octets(rs, &mut tmp, std::usize::MAX)?;
            self.bytecode = tmp.into_boxed_slice();
            Ok(size)
         },
         Some(len) => {
            let mut tmp = Vec::<u8>::with_capacity(len);
            unsafe { tmp.set_len(len); }
            let size = d.deserialize_octets(rs, tmp.as_mut_slice())?;
            self.bytecode = tmp.into_boxed_slice();
            Ok(size)
         }
      }
   }
}

impl std::fmt::Display for Script {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match crate::ui::bitcoin::serialize(self, &false).map(|b| crate::utils::b2h(b)) {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(_) => f.write_fmt(format_args!("err")),
      }
   }
}



#[test]
fn test_deserialize_script() {
   use super::{Script};

   let hexstring = "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab";
   let hexbytes  = crate::utils::h2b(hexstring).unwrap();
   
   let script = crate::ui::bitcoin::hex_to_script(hexstring).unwrap();

   assert_eq!(hexbytes.as_ref(), script.bytecode.as_ref());
}

   
