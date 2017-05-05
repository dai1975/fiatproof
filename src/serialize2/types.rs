use std::marker::PhantomData;
use serde::ser::{self, SerializeTuple};
use super::medium;

pub struct VarInt(pub u64);
impl ser::Serialize for VarInt {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      if self.0 < 253 {
         s.serialize_u8(self.0 as u8)
      } else if self.0 <= 0xFFFF {
         let mut tmp = try!(s.serialize_tuple(2));
         let _ = try!(tmp.serialize_element(&253u8));
         let _ = try!(tmp.serialize_element(&(self.0 as u16)));
         tmp.end()
      } else if self.0 <= 0xFFFFFFFF {
         let mut tmp = try!(s.serialize_tuple(2));
         let _ = try!(tmp.serialize_element(&254u8));
         let _ = try!(tmp.serialize_element(&(self.0 as u32)));
         tmp.end()
      } else {
         let mut tmp = try!(s.serialize_tuple(2));
         let _ = try!(tmp.serialize_element(&255u8));
         let _ = try!(tmp.serialize_element(&(self.0 as u64)));
         tmp.end()
      }
   }
}

pub struct FixedOctets<'a>(&'a [u8]);
impl <'a> ser::Serialize for FixedOctets<'a> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      s.serialize_bytes(self.0)
   }
}

pub struct SizedOctets<'a>(&'a [u8]);
impl <'a> ser::Serialize for SizedOctets<'a> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      let mut tmp = try!(s.serialize_tuple(2));
      try!(tmp.serialize_element(&VarInt(self.0.len() as u64)));
      try!(tmp.serialize_element(&FixedOctets(self.0)));
      tmp.end()
   }
}
