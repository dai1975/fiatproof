use ::std::borrow::Borrow;
use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee, SerializeError};

use ::protocol::MessageHeader;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for MessageHeader {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.magic));
      r += try!(e.encode_array_u8(&self.command.data[..]));
      r += try!(e.encode_u32le(self.length));
      r += try!(e.encode_u32le(self.checksum));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for MessageHeader {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_u32le(&mut self.magic));
      r += try!(d.decode_array_u8(&mut self.command.data[..]));
      r += try!(d.decode_u32le(&mut self.length));
      r += try!(d.decode_u32le(&mut self.checksum));
      Ok(r)
   }
}

use ::protocol::NetworkAddress;
impl <E:BitcoinEncoder> BitcoinEncodee<E,bool> for NetworkAddress {
   fn encode<BP:Borrow<bool>+Sized>(&self, p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      let version = e.param().version();
      if e.param().is_disk() {
         r += try!(e.encode_i32le(version));
      }
      {
         use ::protocol::ADDRESS_TIME_VERSION;
         let encode_time = *p.borrow();
         if e.param().is_disk() ||
            (encode_time && !e.param().is_gethash() && (ADDRESS_TIME_VERSION <= version))
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
impl <D:BitcoinDecoder> BitcoinDecodee<D,bool> for NetworkAddress {
   fn decode<BP:Borrow<bool>+Sized>(&mut self, p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      let mut version = d.param().version();
      if d.param().is_disk() {
         r += try!(d.decode_i32le(&mut version));
      }
      {
         use ::protocol::ADDRESS_TIME_VERSION;
         let encode_time = *p.borrow();
         if d.param().is_disk() ||
            (encode_time && !d.param().is_gethash() && (ADDRESS_TIME_VERSION <= version))
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
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for InvType {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => serialize_error!("malformed inv type"),
      };
      e.encode_u32le(tmp)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for InvType {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += try!(d.decode_u32le(&mut tmp));
      *self = match tmp {
         1 => InvType::Tx,
         2 => InvType::Block,
         3 => InvType::FilteredBlock,
         _ => serialize_error!("unexpected inv value"),
      };
      Ok(r)
   }
}

impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for Inv {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.invtype.encode((), e));
      r += try!(self.hash.encode((), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for Inv {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.invtype.decode((), d));
      r += try!(self.hash.decode((), d));
      Ok(r)
   }
}

use ::protocol::VersionMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for VersionMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      r += try!(e.encode_u64le(self.services));
      {
         use std::time::UNIX_EPOCH;
         use std::i64::MAX as i64_max;
         let t:u64 = match self.timestamp.duration_since(UNIX_EPOCH) {
            Ok(d)  => d.as_secs(),
            Err(_) => serialize_error!("the timestamp is earler than epoch"),
         };
         if (i64_max as u64) < t {
            serialize_error!("the timestamp is later than i64::MAX");
         }
         r += try!(e.encode_i64le(t as i64));
      }
      r += try!(self.addr_recv.encode(false, e));
      r += try!(self.addr_from.encode(false, e));
      r += try!(e.encode_u64le(self.nonce));
      {
         use ::protocol::MAX_SUBVERSION_LENGTH;
         r += try!(self.user_agent.encode(MAX_SUBVERSION_LENGTH, e));
      }
      r += try!(e.encode_i32le(self.start_height));
      r += try!(e.encode_bool(self.relay));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for VersionMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      r += try!(d.decode_u64le(&mut self.services));
      {
         let mut t:i64 = 0;
         r += try!(d.decode_i64le(&mut t));
         if t < 0 {
            serialize_error!("the timestamp is earler than epoch")
         }
         use std::time::{UNIX_EPOCH, Duration};
         self.timestamp = UNIX_EPOCH + Duration::from_secs(t as u64);
      }
      r += try!(self.addr_recv.decode(false, d));
      r += try!(self.addr_from.decode(false, d));
      r += try!(d.decode_u64le(&mut self.nonce));
      {
         use ::protocol::MAX_SUBVERSION_LENGTH;
         r += try!(self.user_agent.decode(MAX_SUBVERSION_LENGTH, d));
      }
      r += try!(d.decode_i32le(&mut self.start_height));
      r += try!(d.decode_bool(&mut self.relay));
      Ok(r)
   }
}

use ::protocol::VerAckMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for VerAckMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, _e:&mut E) -> Result<usize, Error> {
      Ok(0usize)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for VerAckMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, _d:&mut D) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::AddrMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for AddrMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.encode((MAX_ADDR_SIZE,true), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for AddrMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_ADDR_SIZE;
      r += try!(self.addrs.decode((MAX_ADDR_SIZE,true), d));
      Ok(r)
   }
}

use ::protocol::InvMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for InvMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.encode((MAX_INV_SIZE,()), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for InvMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.decode((MAX_INV_SIZE,()), d));
      Ok(r)
   }
}

use ::protocol::GetDataMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for GetDataMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.encode((MAX_INV_SIZE,()), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for GetDataMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::MAX_INV_SIZE;
      r += try!(self.invs.decode((MAX_INV_SIZE,()), d));
      Ok(r)
   }
}

use ::protocol::MerkleBlockMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for MerkleBlockMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.block.encode((),e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for MerkleBlockMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.block.decode((),d));
      Ok(r)
   }
}

use ::protocol::GetBlocksMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for GetBlocksMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.locator.encode((), e));
      r += try!(self.hash_stop.encode((), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for GetBlocksMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.locator.decode((), d));
      r += try!(self.hash_stop.decode((), d));
      Ok(r)
   }
}

use ::protocol::GetHeadersMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for GetHeadersMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.locator.encode((), e));
      r += try!(self.hash_stop.encode((), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for GetHeadersMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.locator.decode((), d));
      r += try!(self.hash_stop.decode((), d));
      Ok(r)
   }
}

use ::protocol::TxMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for TxMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.tx.encode((),e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for TxMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.tx.decode((),d));
      Ok(r)
   }
}

use ::protocol::HeadersMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for HeadersMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.headers.encode((::std::usize::MAX,()), e));
      r += try!(e.encode_varint(0u64));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for HeadersMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.headers.decode((::std::usize::MAX,()), d));
      {
         let mut x:u64 = 0;
         r += try!(d.decode_varint(&mut x));
         if x != 0 { serialize_error!(format!("HeadersMessage seems to have block body: len={}", x)) }
      }
      
      Ok(r)
   }
}

use ::protocol::BlockMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for BlockMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.block.encode((), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for BlockMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.block.decode((), d));
      Ok(r)
   }
}

use ::protocol::GetAddrMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for GetAddrMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, _e:&mut E) -> Result<usize, Error> {
      Ok(0usize)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for GetAddrMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, _d:&mut D) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::MemPoolMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for MemPoolMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, _e:&mut E) -> Result<usize, Error> {
      Ok(0usize)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for MemPoolMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, _d:&mut D) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::{PingMessage};
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for PingMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < e.param().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for PingMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < d.param().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::{PongMessage};
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for PongMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < e.param().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for PongMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      use ::protocol::BIP0031_VERSION;
      if BIP0031_VERSION < d.param().version() {
         r += try!(d.decode_u64le(&mut self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::AlertMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for AlertMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.msg[..]));
      r += try!(e.encode_sequence_u8(&self.sig[..]));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for AlertMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.msg));
      r += try!(d.decode_sequence_u8(&mut self.sig));
      Ok(r)
   }
}

use ::protocol::NotFoundMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for NotFoundMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.invs.encode((::std::usize::MAX,()), e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for NotFoundMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.invs.decode((::std::usize::MAX,()), d));
      Ok(r)
   }
}

use ::protocol::FilterLoadMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for FilterLoadMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      r += try!(e.encode_u32le(self.hash_funcs));
      r += try!(e.encode_u32le(self.tweak));
      r += try!(e.encode_u8(self.flags));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for FilterLoadMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      r += try!(d.decode_u32le(&mut self.hash_funcs));
      r += try!(d.decode_u32le(&mut self.tweak));
      r += try!(d.decode_u8(&mut self.flags));
      Ok(r)
   }
}

use ::protocol::FilterAddMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for FilterAddMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for FilterAddMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(d.decode_sequence_u8(&mut self.data));
      Ok(r)
   }
}

use ::protocol::FilterClearMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for FilterClearMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, _e:&mut E) -> Result<usize, Error> {
      Ok(0usize)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for FilterClearMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, _d:&mut D) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::RejectMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for RejectMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, e:&mut E) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.command.encode(::std::usize::MAX, e));
      r += try!(e.encode_u8(self.code));
      r += try!(self.reason.encode(RejectMessage::MAX_REJECT_MESSAGE_LENGTH, e));
      Ok(r)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for RejectMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, d:&mut D) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(self.command.decode(::std::usize::MAX, d));
      r += try!(d.decode_u8(&mut self.code));
      r += try!(self.reason.decode(RejectMessage::MAX_REJECT_MESSAGE_LENGTH, d));
      // この後に拡張データがあるが、メッセージヘッダのサイズを見ないと分からない。
      Ok(r)
   }
}
   
use ::protocol::SendHeadersMessage;
impl <E:BitcoinEncoder> BitcoinEncodee<E,()> for SendHeadersMessage {
   fn encode<BP:Borrow<()>+Sized>(&self, _p:BP, _e:&mut E) -> Result<usize, Error> {
      Ok(0usize)
   }
}
impl <D:BitcoinDecoder> BitcoinDecodee<D,()> for SendHeadersMessage {
   fn decode<BP:Borrow<()>+Sized>(&mut self, _p:BP, _d:&mut D) -> Result<usize, Error> {
      Ok(0usize)
   }
}


#[test]
fn test_message_header() {
   use ::protocol::Message;
   use ::protocol::message_command::{MessageCommand, VERSION};
   use ::serialize::{FixedBitcoinSerializer};
   let m = MessageHeader {
      magic:    ::MAIN_PARAMS.magic,
      command:  MessageCommand { data: VERSION },
      length:   0x39,
      checksum: 0x12345678,
   };
   let mut ser = FixedBitcoinSerializer::new(100);
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
   use ::serialize::{FixedBitcoinSerializer};
   use std::net::SocketAddr;
   use std::str::FromStr;
   
   let m = NetworkAddress {
      services:  NODE_FULL,
      time:      0x01020304u32,
      sockaddr:  SocketAddr::from_str("10.0.0.1:8333").unwrap(),
   };
   let mut ser = FixedBitcoinSerializer::new(100);
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
   use ::serialize::{FixedBitcoinSerializer};
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
   let mut ser = FixedBitcoinSerializer::new(100);
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

   
