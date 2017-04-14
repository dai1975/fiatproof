use std::io::Cursor;
use serde::ser;
use super::Serializer;

struct VarInt(u64);
impl ser::Serialize for VarInt {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      if self.0 < 253 {
         s.serialize_u8(v as u8)
      } else if self.0 <= 0xFFFF {
         let bytes = [0u8;3];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(253u8)?;
            n += s.serialize_u16(self.0 as u16)?;
         }
         s.serialize_bytes(&bytes)
      } else if self.0 <= 0xFFFFFFFF {
         let bytes = [0u8;5];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(254u8)?;
            n += s.serialize_u32(self.0 as u32)?;
         }
         s.serialize_bytes(&bytes)
      } else {
         let bytes = [0u8;9];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(255u8)?;
            n += s.serialize_u64(self.0)?;
         }
         s.serialize_bytes(&bytes)
      }
   }
}

struct Sequence<T = Medium::NET>(&[u8]);
impl <T> ser::Serialize for Sequence<T> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      if self.0 < 253 {
         s.serialize_u8(v as u8)
      } else if v <= 0xFFFF {
         let bytes = [0u8;3];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(253u8)?;
            n += s.serialize_u16(v as u16)?;
         }
         s.serialize_bytes(&bytes)
      } else if v <= 0xFFFFFFFF {
         let bytes = [0u8;5];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(254u8)?;
            n += s.serialize_u32(v as u32)?;
         }
         s.serialize_bytes(&bytes)
      } else {
         let bytes = [0u8;9];
         {
            let mut n = 0usize;
            let s = super::Serializer::new(Cursor::new(&mut bytes));
            n += s.serialize_u8(255u8)?;
            n += s.serialize_u64(v)?;
         }
         s.serialize_bytes(&bytes)
      }
   }
}
