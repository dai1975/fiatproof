use super::apriori::COMMAND_LENGTH;

#[derive(Debug,Default,Clone)]
pub struct MessageHeader {
   pub magic:    u32,
   pub command:  [u8; COMMAND_LENGTH],
   pub length:   u32,
   pub checksum: u32,
}


use crate::iostream::{ WriteStream, ReadStream };
use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for MessageHeader {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_u32le(ws, self.magic)?;
      r += e.serialize_octets(ws, &self.command[..])?;
      r += e.serialize_u32le(ws, self.length)?;
      r += e.serialize_u32le(ws, self.checksum)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for MessageHeader {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> crate::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_u32le(rs, &mut self.magic)?;
      r += d.deserialize_octets(rs, &mut self.command[..])?;
      r += d.deserialize_u32le(rs, &mut self.length)?;
      r += d.deserialize_u32le(rs, &mut self.checksum)?;
      Ok(r)
   }
}


#[test]
fn test_message_header() {
   use super::apriori::COMMAND_LENGTH;
   const VERSION:[u8; COMMAND_LENGTH] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];
   let obj = MessageHeader {
      magic:    crate::bitcoin::presets::bitcoin_mainnet::CHAIN.magic,
      command:  VERSION,
      length:   0x39,
      checksum: 0x12345678,
   };

   let mut w = ::iostream::VecWriteStream::default();
   {
      let m = crate::bitcoin::serialize::Medium::new("net").unwrap();
      let e = crate::bitcoin::serialize::Serializer::new(&m);
      assert_matches!(obj.serialize(&(), &e, &mut w), Ok(24usize));
   }
   assert_eq!(&w.get_ref()[..24],
              [0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12]);
}
