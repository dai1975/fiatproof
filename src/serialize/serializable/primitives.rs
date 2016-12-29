use super::super::{Serializable, Encoder, SerializeParam};
use ::Error;

impl Serializable for bool {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_bool(*self)
   }
}
impl Serializable for u8 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_u8(*self)
   }
}
impl Serializable for u16 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_u16(*self)
   }
}
impl Serializable for u32 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_u32(*self)
   }
}
impl Serializable for u64 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_u64(*self)
   }
}
impl Serializable for i8 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_i8(*self)
   }
}
impl Serializable for i16 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_i16(*self)
   }
}
impl Serializable for i32 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_i32(*self)
   }
}
impl Serializable for i64 {
   fn serialize(&self, enc: &mut Encoder, _sp: &SerializeParam) -> Result<usize, Error> {
      enc.encode_i64(*self)
   }
}

#[test]
fn test_primitives() {
   use super::super::{BitcoinSerializer, FixedWriteStream};
   let mut enc = BitcoinSerializer::new(FixedWriteStream::new(100));
   let sp = SerializeParam::new_net();
   assert_matches!(0x0102u16.serialize(&mut enc, &sp), Ok(2));
   assert_matches!(0x1112131415161718u64.serialize(&mut enc, &sp), Ok(8));
   assert_eq!([0x02, 0x01, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11], &enc.get_ref().get_ref()[0..10]);
}
