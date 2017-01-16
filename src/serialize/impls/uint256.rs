use ::std::borrow::Borrow;
use ::{Error, UInt256};
use super::super::{Encoder, Encodee, Decoder, Decodee};

impl <E:Encoder> Encodee<E,()> for UInt256 {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      e.encode_uint256(self)
   }
}

impl <D:Decoder> Decodee<D,()> for UInt256 {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      d.decode_uint256(self)
   }
}

#[test]
fn test_encode() {
   use super::super::FixedSerializer;
   let mut ser = FixedSerializer::new(100);
   let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
               0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let v = UInt256::new(&data);
   assert_matches!(v.encode((), &mut ser), Ok(32));
   assert_eq!(data, ser.as_slice()[..32]);
}

#[test]
fn test_decode() {
   use super::super::SliceDeserializer;
   let data:Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                           0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let mut des = SliceDeserializer::new(data);
   let mut v = UInt256::default();
   assert_matches!(v.decode((), &mut des), Ok(32));
   assert_eq!(des.as_slice()[..32], v.data);
}
