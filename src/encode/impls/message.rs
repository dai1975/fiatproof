use ::std::borrow::Borrow;
use super::super::{EncodeStream, Encodee, DecodeStream, Decodee};

use ::protocol::MessageHeader;
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

use ::protocol::NetworkAddress;
impl Encodee for NetworkAddress {
   type P = bool;
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let version = e.media().version();
      if e.media().is_disk() {
         r += try!(e.encode_i32le(version));
      }
      {
         use ::protocol::ADDRESS_TIME_VERSION;
         let encode_time = *p.borrow();
         if e.media().is_disk() ||
            (encode_time && !e.media().is_hash() && (ADDRESS_TIME_VERSION <= version))
         {
            r += try!(e.encode_u32le(self.time));
         }
      }
      r += try!(e.encode_u64le(self.services));

      {
         use std::net::IpAddr;
         let v6 = match self.sockaddr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
         };
         r += try!(e.encode_array_u8(&v6.octets()));
      }
      r += try!(e.encode_u16be(self.sockaddr.port())); //network byte order
      Ok(r)
   }
}
impl Decodee for NetworkAddress {
   type P = bool;
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut version = d.media().version();
      if d.media().is_disk() {
         r += try!(d.decode_i32le(&mut version));
      }
      {
         use ::protocol::ADDRESS_TIME_VERSION;
         let encode_time = *p.borrow();
         if d.media().is_disk() ||
            (encode_time && !d.media().is_hash() && (ADDRESS_TIME_VERSION <= version))
         {
            r += try!(d.decode_u32le(&mut self.time));
         }
      }

      r += try!(d.decode_u64le(&mut self.services));

      {
         use std::net::{IpAddr, Ipv6Addr};
         let mut octets = [0u8; 16];
         r += try!(d.decode_array_u8(&mut octets));
         let v6 = Ipv6Addr::from(octets);
         self.sockaddr.set_ip(match v6.to_ipv4() {
            Some(v4) => IpAddr::V4(v4),
            None     => IpAddr::V6(v6),
         });
      }
      
      {
         let mut port:u16 = 0;
         r += try!(d.decode_u16be(&mut port));
         self.sockaddr.set_port(port);
      }
      Ok(r)
   }
}

use ::protocol::{InvType, Inv};
impl Encodee for InvType {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => encode_error!("malformed inv type"),
      };
      e.encode_u32le(tmp)
   }
}
impl Decodee for InvType {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += try!(d.decode_u32le(&mut tmp));
      *self = match tmp {
         1 => InvType::Tx,
         2 => InvType::Block,
         3 => InvType::FilteredBlock,
         _ => encode_error!("unexpected inv value"),
      };
      Ok(r)
   }
}

impl Encodee for Inv {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invtype.encode(e, ()));
      r += try!(self.hash.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for Inv {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invtype.decode(d, ()));
      r += try!(self.hash.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::VersionMessage;
impl Encodee for VersionMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_u64le(self.services));
      {
         use std::time::UNIX_EPOCH;
         use std::i64::MAX as i64_max;
         let t:u64 = match self.timestamp.duration_since(UNIX_EPOCH) {
            Ok(d)  => d.as_secs(),
            Err(_) => encode_error!("the timestamp is earler than epoch"),
         };
         if (i64_max as u64) < t {
            encode_error!("the timestamp is later than i64::MAX");
         }
         r += try!(e.encode_i64le(t as i64));
      }
      r += try!(self.addr_recv.encode(e, false));
      r += try!(self.addr_from.encode(e, false));
      r += try!(e.encode_u64le(self.nonce));
      {
         use ::protocol::MAX_SUBVERSION_LENGTH;
         r += try!(self.user_agent.encode(e, MAX_SUBVERSION_LENGTH));
      }
      r += try!(e.encode_i32le(self.start_height));
      r += try!(e.encode_bool(self.relay));
      Ok(r)
   }
}
impl Decodee for VersionMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(d.decode_u64le(&mut self.services));
      {
         let mut t:i64 = 0;
         r += try!(d.decode_i64le(&mut t));
         if t < 0 {
            encode_error!("the timestamp is earler than epoch")
         }
         use std::time::{UNIX_EPOCH, Duration};
         self.timestamp = UNIX_EPOCH + Duration::from_secs(t as u64);
      }
      r += try!(self.addr_recv.decode(d, false));
      r += try!(self.addr_from.decode(d, false));
      r += try!(d.decode_u64le(&mut self.nonce));
      {
         use ::protocol::MAX_SUBVERSION_LENGTH;
         r += try!(self.user_agent.decode(d, MAX_SUBVERSION_LENGTH));
      }
      r += try!(d.decode_i32le(&mut self.start_height));
      r += try!(d.decode_bool(&mut self.relay));
      Ok(r)
   }
}

use ::protocol::VerAckMessage;
impl Encodee for VerAckMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for VerAckMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}

use ::protocol::AddrMessage;
impl Encodee for AddrMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.encode(e, (MAX_ADDR_SIZE,true)));
      Ok(r)
   }
}
impl Decodee for AddrMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.decode(d, (MAX_ADDR_SIZE,true)));
      Ok(r)
   }
}

use ::protocol::InvMessage;
impl Encodee for InvMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.encode(e, (MAX_INV_SIZE,())));
      Ok(r)
   }
}
impl Decodee for InvMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.decode(d, (MAX_INV_SIZE,())));
      Ok(r)
   }
}

use ::protocol::GetDataMessage;
impl Encodee for GetDataMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.encode(e, (MAX_INV_SIZE,())));
      Ok(r)
   }
}
impl Decodee for GetDataMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.decode(d, (MAX_INV_SIZE,())));
      Ok(r)
   }
}

use ::protocol::MerkleBlockMessage;
impl Encodee for MerkleBlockMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for MerkleBlockMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::GetBlocksMessage;
impl Encodee for GetBlocksMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.encode(e, ()));
      r += try!(self.hash_stop.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for GetBlocksMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.decode(d, ()));
      r += try!(self.hash_stop.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::GetHeadersMessage;
impl Encodee for GetHeadersMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.encode(e, ()));
      r += try!(self.hash_stop.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for GetHeadersMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.locator.decode(d, ()));
      r += try!(self.hash_stop.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::TxMessage;
impl Encodee for TxMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.tx.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for TxMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.tx.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::HeadersMessage;
impl Encodee for HeadersMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.headers.encode(e, (::std::usize::MAX,())));
      r += try!(e.encode_varint(0u64));
      Ok(r)
   }
}
impl Decodee for HeadersMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.headers.decode(d, (::std::usize::MAX,())));
      {
         let mut x:u64 = 0;
         r += try!(d.decode_varint(&mut x));
         if x != 0 { encode_error!(format!("HeadersMessage seems to have block body: len={}", x)) }
      }
      
      Ok(r)
   }
}

use ::protocol::BlockMessage;
impl Encodee for BlockMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for BlockMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.block.decode(d, ()));
      Ok(r)
   }
}

use ::protocol::GetAddrMessage;
impl Encodee for GetAddrMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for GetAddrMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}

use ::protocol::MemPoolMessage;
impl Encodee for MemPoolMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for MemPoolMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}

use ::protocol::{PingMessage};
impl Encodee for PingMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < e.media().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl Decodee for PingMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < d.media().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::{PongMessage};
impl Encodee for PongMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < e.media().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl Decodee for PongMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < d.media().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::AlertMessage;
impl Encodee for AlertMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.msg[..]));
      r += try!(e.encode_sequence_u8(&self.sig[..]));
      Ok(r)
   }
}
impl Decodee for AlertMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.msg));
      r += try!(d.decode_sequence_u8(&mut self.sig));
      Ok(r)
   }
}

use ::protocol::NotFoundMessage;
impl Encodee for NotFoundMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invs.encode(e, (::std::usize::MAX,())));
      Ok(r)
   }
}
impl Decodee for NotFoundMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invs.decode(d, (::std::usize::MAX,())));
      Ok(r)
   }
}

use ::protocol::FilterLoadMessage;
impl Encodee for FilterLoadMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      r += try!(e.encode_u32le(self.hash_funcs));
      r += try!(e.encode_u32le(self.tweak));
      r += try!(e.encode_u8(self.flags));
      Ok(r)
   }
}
impl Decodee for FilterLoadMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      r += try!(d.decode_u32le(&mut self.hash_funcs));
      r += try!(d.decode_u32le(&mut self.tweak));
      r += try!(d.decode_u8(&mut self.flags));
      Ok(r)
   }
}

use ::protocol::FilterAddMessage;
impl Encodee for FilterAddMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      Ok(r)
   }
}
impl Decodee for FilterAddMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      Ok(r)
   }
}

use ::protocol::FilterClearMessage;
impl Encodee for FilterClearMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for FilterClearMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}

use ::protocol::RejectMessage;
impl Encodee for RejectMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.command.encode(e, ::std::usize::MAX));
      r += try!(e.encode_u8(self.code));
      r += try!(self.reason.encode(e, RejectMessage::MAX_REJECT_MESSAGE_LENGTH));
      Ok(r)
   }
}
impl Decodee for RejectMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.command.decode(d, ::std::usize::MAX));
      r += try!(d.decode_u8(&mut self.code));
      r += try!(self.reason.decode(d, RejectMessage::MAX_REJECT_MESSAGE_LENGTH));
      // この後に拡張データがあるが、メッセージヘッダのサイズを見ないと分からない。
      Ok(r)
   }
}
   
use ::protocol::SendHeadersMessage;
impl Encodee for SendHeadersMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}
impl Decodee for SendHeadersMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, _p:BP) -> ::Result<usize> {
      Ok(0usize)
   }
}


#[test]
fn test_message_header() {
   use ::protocol::message_command::{MessageCommand, VERSION};
   use ::encode::{FixedEncodeStream};
   let m = MessageHeader {
      magic:    ::MAIN_PARAMS.magic,
      command:  MessageCommand { data: VERSION },
      length:   0x39,
      checksum: 0x12345678,
   };
   let mut ser = FixedEncodeStream::new(100);
   ser.mut_param().set_net();
   assert_matches!(m.encode((), &mut ser), Ok(24usize));
   assert_eq!([0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12], &ser.as_slice()[0..24]);
}

#[test]
fn test_address() {
   use ::protocol::{NetworkAddress, NODE_FULL};
   use ::encode::{FixedEncodeStream};
   use std::net::SocketAddr;
   use std::str::FromStr;
   
   let m = NetworkAddress {
      services:  NODE_FULL,
      time:      0x01020304u32,
      sockaddr:  SocketAddr::from_str("10.0.0.1:8333").unwrap(),
   };
   let mut ser = FixedEncodeStream::new(100);
   ser.mut_param().set_net();

   let exp_time = [0x04, 0x03, 0x02, 0x01];
   let exp_addr = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0A, 0x00, 0x00, 0x01,
                   0x20, 0x8D];
   
   assert_matches!(m.encode(false, &mut ser), Ok(26usize));
   assert_eq!(exp_addr, &ser.as_slice()[0..26]);

   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION - 1);
   assert_matches!(m.encode(true, &mut ser), Ok(26usize));
   assert_eq!(exp_addr, &ser.as_slice()[0..26]);

   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION);
   assert_matches!(m.encode(true, &mut ser), Ok(30usize));
   assert_eq!(exp_time, &ser.as_slice()[0..4]);
   assert_eq!(exp_addr, &ser.as_slice()[4..30]);
}

#[test]
fn test_version_message() {
   use ::protocol::{NetworkAddress, NODE_FULL};
   use ::encode::{FixedEncodeStream};
   use std::net::SocketAddr;
   use std::str::FromStr;
   use std::time::{Duration, UNIX_EPOCH};
   
   let m = VersionMessage {
      version:      70012,
      services:     NODE_FULL,
      timestamp:    UNIX_EPOCH + Duration::from_secs(0x0001020304050607u64),
      addr_recv:    NetworkAddress {
         services:  NODE_FULL,
         time:      0x01020304u32,
         sockaddr: SocketAddr::from_str("10.0.0.1:8333").unwrap(),
      },
      addr_from:    NetworkAddress {
         services:  NODE_FULL,
         time:      0x01020304u32,
         sockaddr:  SocketAddr::from_str("192.168.0.1:18333").unwrap(),
      },
      nonce:        0x08090A0B0C0D0E0Fu64,
      user_agent:   "Hatsune Miku".to_string(),
      start_height: 723333,
      relay:        true,
   };
   let mut ser = FixedEncodeStream::new(100);
   ser.mut_param().set_net().set_version(0);
   // bitcoin-core rely on a state that version is not agreeed and set as 0 in sending or recving version message.

   let exp:&[u8] = &[
      0x7C, 0x11, 0x01, 0x00,
      0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
      0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0A, 0x00, 0x00, 0x01, 0x20, 0x8D,
      0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xC0, 0xA8, 0x00, 0x01, 0x47, 0x9D,
      0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08,
      0x0C, 0x48, 0x61, 0x74, 0x73, 0x75, 0x6E, 0x65, 0x20, 0x4D, 0x69, 0x6B, 0x75,
      0x85, 0x09, 0x0b, 0x00,
      0x01,
   ];
   assert_matches!(m.encode((), &mut ser), Ok(98));
   assert_eq!(exp, &ser.as_slice()[0..98]);

   // this impl impls for version message not to emit address time if runtime version is later than addr_time_version
   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION);
   assert_matches!(m.encode((), &mut ser), Ok(98));
   assert_eq!(exp, &ser.as_slice()[0..98]);
}

   
