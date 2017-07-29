#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct UInt256 {
   pub data: [u8;32],
}

pub const ZERO:UInt256 = UInt256 { data: [0u8;32] };

impl ::std::hash::Hash for UInt256 {
   fn hash<H: ::std::hash::Hasher>(&self, state:&mut H) {
      state.write(&self.data[..]);
   }
}

impl UInt256 {
   pub fn new(d: &[u8;32]) -> UInt256 {
      let mut v = UInt256 { data: [0u8;32] };
      v.data.clone_from_slice(d);
      v
   }
   pub fn as_slice(&self) -> &[u8] {
      &self.data[..]
   }
}

impl ::std::ops::Index<usize> for UInt256 {
   type Output = u8;
   fn index(&self, i:usize) -> &u8 {
      &self.data[i]
   }
}
impl ::std::ops::IndexMut<usize> for UInt256 {
   fn index_mut(&mut self, i:usize) -> &mut u8 {
      &mut self.data[i]
   }
}
impl ::std::fmt::Display for UInt256 {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      /*
      use ::serialize::ToBytes;
      match self.to_rhex() {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(e) => f.write_fmt(format_args!("{:?}", e)),
      }
       */
      Ok(())
   }
}

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for UInt256 {
   fn encode(&self, enc:&mut BitcoinEncoder) -> ::Result<usize> {
      enc.encode_array_u8(&self.data[..])
   }
}
impl BitcoinDecodee for UInt256 {
   fn decode(&mut self, dec:&mut BitcoinDecoder) -> ::Result<usize> {
      dec.decode_array_u8(&mut self.data[..])
   }
}
/*
impl ::serialize::ToBytes for UInt256 {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      self.data.to_bytes()
   }
}
impl ::serialize::FromBytes for UInt256 {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      let s = s.as_ref();
      if s.len() != self.data.len() { frombytes_error!(format!("length mismatch: {} but {}", self.data.len(), s.len())); }
      self.data.clone_from_slice(s);
      Ok(())
   }
}
*/

/*
#[test]
fn test_str() {
   use ::serialize::WithBytes;
   let s = "00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81";
   let uint256 = UInt256::with_rhex(s).unwrap();

   let expect:[u8;32] = [
      0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
      0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
   ];
   assert_eq!(expect, uint256.data);

   let t = format!("{}", uint256);
   assert_eq!(s, t);
}

#[test]
fn test_encode() {
   use ::serialize::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
   let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
               0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let v = UInt256::new(&data);
   assert_matches!(v.encode(&mut e, ()), Ok(32));
   assert_eq!(&e.w.get_ref()[..32], &data[..]);
}

#[test]
fn test_decode() {
   use ::serialize::{BitcoinDecodeStream, SliceReadStream, Media};
   let data:Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                           0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F ];
   let mut d = BitcoinDecodeStream::new(SliceReadStream::new(data), Media::default().set_net());

   let mut v = UInt256::default();
   assert_matches!(v.decode(&mut d, ()), Ok(32));
   assert_eq!(&d.r.get_ref()[..32], &v.data[..]);
}
*/
