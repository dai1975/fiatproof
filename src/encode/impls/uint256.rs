use ::std::borrow::Borrow;
use ::{UInt256};
use super::super::{EncodeStream, Encodee, DecodeStream, Decodee};

impl Encodee for UInt256 {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      e.encode_array_u8(&self.data[..])
   }
}

impl Decodee for UInt256 {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      d.decode_array_u8(&mut self.data[..])
   }
}

#[test]
fn test_encode() {
   use ::encode::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
   let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
               0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let v = UInt256::new(&data);
   assert_matches!(v.encode(&mut e, ()), Ok(32));
   assert_eq!(&e.w.get_ref()[..32], &data[..]);
}

#[test]
fn test_decode() {
   use ::encode::{BitcoinDecodeStream, SliceReadStream, Media};
   let data:Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                           0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let mut d = BitcoinDecodeStream::new(SliceReadStream::new(data), Media::default().set_net());

   let mut v = UInt256::default();
   assert_matches!(v.decode(&mut d, ()), Ok(32));
   assert_eq!(&d.r.get_ref()[..32], &v.data[..]);
}
