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


pub struct LimitedSequence<'a, T: 'a+ser::Serialize>(&'a Vec<T>, usize);
impl <'a,T: ser::Serialize> ser::Serialize for LimitedSequence<'a,T> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      let len = self.0.len();
      {
         let lim = self.1;
         if lim < len {
            ser_error!(format!("sequence exceeds limit: {} but {}", lim, len));
         }
      }
      self.0.serialize(s)
   }
}

pub struct LimitedString<'a>(&'a str, usize);
impl <'a> ser::Serialize for LimitedString<'a> {
   fn serialize<S: ser::Serializer>(&self, s:S) -> Result<S::Ok, S::Error> {
      // limited str は最大値越えてもエラーにはしない
      use std::cmp::min;
      use std::u32::MAX;
      let bytes = self.0.as_bytes();
      let size  = min(MAX as usize, min(self.1, bytes.len()));
      SizedOctets(&bytes[0..size]).serialize(s)
   }
}


#[test]
fn test_varint() {
   use serde::ser::Serialize;
   use ::serialize2::{Serializer, VecWriteStream, VarInt};
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(VarInt(0).serialize(&mut ser), Ok(1usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[0]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(VarInt(252).serialize(&mut ser), Ok(1usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[252]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(VarInt(253).serialize(&mut ser), Ok(3usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[253, 253, 0]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(VarInt(0x10000).serialize(&mut ser), Ok(5usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[254, 0x00, 0x00, 0x01, 0x00]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(VarInt(0x100000000).serialize(&mut ser), Ok(9usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]);
   }
}

#[test]
fn test_octets() {
   use serde::ser::Serialize;
   use ::serialize2::{Serializer, VecWriteStream, FixedOctets, SizedOctets};
   let octets = [0x01u8, 0x02u8, 0x03u8];
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(FixedOctets(&octets).serialize(&mut ser), Ok(3usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[0x01, 0x02, 0x03]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(SizedOctets(&octets).serialize(&mut ser), Ok(4usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[0x03, 0x01, 0x02, 0x03]);
   }
}

#[test]
fn test_limited_sequence() {
   use serde::ser::Serialize;
   use ::serialize2::{Serializer, VecWriteStream, LimitedSequence};
   let seq = vec![ 0x01u8, 0x02u8, 0x03u8 ];
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(LimitedSequence(&seq, 10).serialize(&mut ser), Ok(4usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                    &[0x03, 0x01, 0x02, 0x03]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(LimitedSequence(&seq, 2).serialize(&mut ser), Err(_));
   }
}

#[test]
fn test_limited_string() {
   use serde::ser::Serialize;
   use ::serialize2::{Serializer, VecWriteStream, LimitedSequence};
   let s = "HatsuneMiku";
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(LimitedString(&s, 100).serialize(&mut ser), Ok(12usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                 &[11, 0x48,0x61,0x74,0x73,0x75,0x6e,0x65,0x4d,0x69,0x6b,0x75]);
   }
   {
      let mut ser = Serializer::new(VecWriteStream::default());
      assert_matches!(LimitedString(&s, 5).serialize(&mut ser), Ok(6usize));
      assert_eq!(ser.into_inner().into_inner().as_slice(),
                 &[5, 0x48,0x61,0x74,0x73,0x75]);
   }
}

