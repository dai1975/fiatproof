use super::MessageCommand;

#[derive(Debug,Default,Clone)]
pub struct MessageHeader {
   pub magic:    u32,
   pub command:  MessageCommand,
   pub length:   u32,
   pub checksum: u32,
}


use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for MessageHeader {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.magic));
      r += try!(e.encode_array_u8(&self.command.data[..]));
      r += try!(e.encode_u32le(self.length));
      r += try!(e.encode_u32le(self.checksum));
      Ok(r)
   }
}
impl Decodee for MessageHeader {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_u32le(&mut self.magic));
      r += try!(d.decode_array_u8(&mut self.command.data[..]));
      r += try!(d.decode_u32le(&mut self.length));
      r += try!(d.decode_u32le(&mut self.checksum));
      Ok(r)
   }
}


#[test]
fn test_message_header() {
   use ::protocol::message_command::{MessageCommand, VERSION};
   let v = MessageHeader {
      magic:    ::MAIN_PARAMS.magic,
      command:  MessageCommand { data: VERSION },
      length:   0x39,
      checksum: 0x12345678,
   };

   use ::serialize::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
   assert_matches!(v.encode(&mut e, ()), Ok(24usize));
   assert_eq!(&e.w.get_ref()[..24],
              [0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12]);
}
