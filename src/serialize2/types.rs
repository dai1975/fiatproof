use serde::ser;
use super::Medium;

pub struct VarInt(u64);
impl ser::Serialize for VarInt {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      if self.0 < 253 {
         s.serialize_u8(self.0 as u8)
      } else if self.0 <= 0xFFFF {
         let tmp = try!(s.serialize_tuple(2));
         try!(tmp.serialize_element(&253u8));
         try!(tmp.serialize_element(&self.0 as &u16));
         tmp.end()
      } else if self.0 <= 0xFFFFFFFF {
         let tmp = try!(s.serialize_tuple(2));
         try!(tmp.serialize_element(&254u8));
         try!(tmp.serialize_element(&self.0 as &u32));
         tmp.end()
      } else {
         let tmp = try!(s.serialize_tuple(2));
         try!(tmp.serialize_element(&255u8));
         try!(tmp.serialize_element(&self.0 as &u64));
         tmp.end()
      }
   }
}

pub struct FixedOctets<M = Medium::NET>(&[u8]);
impl <M> ser::Serialize for FixedOctets<M> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      s.serialize_bytes(self.0)
   }
}

pub struct SizedOctets<M = Medium::NET>(&[u8]);
impl <M> ser::Serialize for SizedOctets<M> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      let tmp = try!(s.serialize_tuple(2));
      try!(tmp.serialize_element(VarInt(self.0.len())));
      try!(tmp.serialize_element(FixedOctets::<M>(self.0)));
      tmp.end()
   }
}
