use ::{Error};
use super::super::super::{Encoder, WriteStream, SerializeError};
use super::super::{BitcoinEncoder, BitcoinEncodee};

macro_rules! serialize_error {
   ($msg:expr) => {
      try!( Err(SerializeError::new($msg.to_string())) )
   }
}

use ::protocol::MessageHeader;
impl BitcoinEncodee for MessageHeader {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.magic, w, ep));
      r += try!(e.encode_array_u8(self.command.data, w, ep));
      r += try!(e.encode_u32le(self.length, w, ep));
      r += try!(e.encode_u32le(self.checksum, w, ep));
      Ok(r)
   }
}

use ::protocol::NetworkAddress;
impl BitcoinEncodee for NetworkAddress {
   type P = bool;
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      use std::net::IpAddr;
      let mut r:usize = 0;
      if *vp && (::protocol::ADDRESS_TIME_VERSION <= ep.version) {
         r += try!(e.encode_u32le(self.time, w, ep));
      }
      r += try!(e.encode_u64le(self.services, w, ep));
      match self.sockaddr.ip() {
         IpAddr::V4(v4) => {
            r += try!(e.encode_array_u8(&[0,0,0,0,0,0,0,0,0,0,0xff,0xff], w, ep));
            r += try!(e.encode_array_u8(&v4.octets(), w, ep));
         },
         IpAddr::V6(v6) => {
            r += try!(e.encode_array_u8(&v6.octets(), w, ep));
         }
      }
      r += try!(e.encode_u16be(self.sockaddr.port(), w, ep)); //network byte order
      Ok(r)
   }
}

use ::protocol::{InvType, Inv};
impl BitcoinEncodee for InvType {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => serialize_error!("malformed inv type"),
      };
      e.encode_u32le(tmp, w, ep)
   }
}
impl BitcoinEncodee for Inv {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.invtype, &(), w, ep));
      r += try!(e.encode_uint256(&self.hash, w, ep));
      Ok(r)
   }
}

use ::protocol::VersionMessage;
const MAX_SUBVERSION_LENGTH:usize = 256;
impl BitcoinEncodee for VersionMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version, w, ep));
      r += try!(e.encode_u64le(self.services, w, ep));
      r += try!(e.encode_i64le(self.timestamp, w, ep));
      r += try!(e.encode(&self.addr_recv, &false, w, ep));
      r += try!(e.encode(&self.addr_from, &false, w, ep));
      r += try!(e.encode_u64le(self.nonce, w, ep));
      r += try!(e.encode_limited_string(self.user_agent.as_str(), MAX_SUBVERSION_LENGTH, w, ep));
      r += try!(e.encode_i32le(self.start_height, w, ep));
      r += try!(e.encode_bool(self.relay, w, ep));
      Ok(r)
   }
}

use ::protocol::VerAckMessage;
impl BitcoinEncodee for VerAckMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, _e:&mut E, _w:&mut W, _ep:&<E as Encoder>::P) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::AddrMessage;
impl BitcoinEncodee for AddrMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.addrs, &true, w, ep));
      Ok(r)
   }
}

use ::protocol::InvMessage;
impl BitcoinEncodee for InvMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs, &(), w, ep));
      Ok(r)
   }
}

use ::protocol::GetDataMessage;
impl BitcoinEncodee for GetDataMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs, &(), w, ep));
      Ok(r)
   }
}

use ::protocol::MerkleBlockMessage;
impl BitcoinEncodee for MerkleBlockMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.block, &(), w, ep));
      Ok(r)
   }
}

use ::protocol::GetBlocksMessage;
impl BitcoinEncodee for GetBlocksMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.locator, &(), w, ep));
      r += try!(e.encode_uint256(&self.hash_stop, w, ep));
      Ok(r)
   }
}

use ::protocol::GetHeadersMessage;
impl BitcoinEncodee for GetHeadersMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.locator, &(), w, ep));
      r += try!(e.encode_uint256(&self.hash_stop, w, ep));
      Ok(r)
   }
}

use ::protocol::TxMessage;
impl BitcoinEncodee for TxMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.tx, &(), w, ep));
      Ok(r)
   }
}

use ::protocol::HeadersMessage;
impl BitcoinEncodee for HeadersMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.headers, &(), w, ep));
      r += try!(e.encode_varint(0u64, w, ep));
      Ok(r)
   }
}

use ::protocol::BlockMessage;
impl BitcoinEncodee for BlockMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.block, &(), w, ep));
      Ok(r)
   }
}

use ::protocol::GetAddrMessage;
impl BitcoinEncodee for GetAddrMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, _e:&mut E, _w:&mut W, _ep:&<E as Encoder>::P) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::MemPoolMessage;
impl BitcoinEncodee for MemPoolMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, _e:&mut E, _w:&mut W, _ep:&<E as Encoder>::P) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::{PingMessage};
impl BitcoinEncodee for PingMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      if (::protocol::BIP0031_VERSION) < ep.version {
         r += try!(e.encode_u64le(self.nonce, w, ep));
      }
      Ok(r)
   }
}

use ::protocol::{PongMessage};
impl BitcoinEncodee for PongMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      if (::protocol::BIP0031_VERSION) < ep.version {
         r += try!(e.encode_u64le(self.nonce, w, ep));
      }
      Ok(r)
   }
}

use ::protocol::AlertMessage;
impl BitcoinEncodee for AlertMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.msg[..], w, ep));
      r += try!(e.encode_sequence_u8(&self.sig[..], w, ep));
      Ok(r)
   }
}

use ::protocol::NotFoundMessage;
impl BitcoinEncodee for NotFoundMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs[..], &(), w, ep));
      Ok(r)
   }
}

use ::protocol::FilterLoadMessage;
impl BitcoinEncodee for FilterLoadMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..], w, ep));
      r += try!(e.encode_u32le(self.hash_funcs, w, ep));
      r += try!(e.encode_u32le(self.tweak, w, ep));
      r += try!(e.encode_u8(self.flags, w, ep));
      Ok(r)
   }
}

use ::protocol::FilterAddMessage;
impl BitcoinEncodee for FilterAddMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..], w, ep));
      Ok(r)
   }
}

use ::protocol::FilterClearMessage;
impl BitcoinEncodee for FilterClearMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, _e:&mut E, _w:&mut W, _ep:&<E as Encoder>::P) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::RejectMessage;
impl BitcoinEncodee for RejectMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      let mut r:usize = 0;
      // https://en.bitcoin.it/wiki/Protocol_documentation#reject
      // ここは var_str で任意の文字列が入るようになっているようだ。
      // 現状、command に固定している API はよろしくない? command にしても、後続の 0 は送られるのだろうか。
      r += try!(e.encode_limited_string(self.command.as_str(), 100, w, ep));
      r += try!(e.encode_u8(self.code, w, ep));
      r += try!(e.encode_limited_string(self.reason.as_str(), RejectMessage::MAX_REJECT_MESSAGE_LENGTH, w, ep));
      // この後に拡張データがあるようだ。decode はメッセージヘッダのサイズを見ないと分からなさそう。
      Ok(r)
   }
}

use ::protocol::SendHeadersMessage;
impl BitcoinEncodee for SendHeadersMessage {
   type P = ();
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, _vp:&Self::P, _e:&mut E, _w:&mut W, _ep:&<E as Encoder>::P) -> Result<usize, Error> {
      Ok(0usize)
   }
}


#[test]
fn test_message_header() {
   use ::protocol::Message;
   use ::serialize::{FixedBitcoinSerializer, BitcoinEncodeParam};
   let m = MessageHeader {
      magic:    ::MAIN_PARAMS.magic,
      command:  <VersionMessage as Message>::COMMAND,
      length:   0x39,
      checksum: 0x12345678,
   };
   let mut ser = FixedBitcoinSerializer::new(100);
   let     ep  = BitcoinEncodeParam::new_net();
   assert_matches!(ser.serialize_bitcoin(&m, &(), &ep), Ok(24usize));
   assert_eq!([0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12], &ser.get_ref_ref()[0..24]);
}

#[test]
fn test_address() {
   use ::protocol::{NetworkAddress, NODE_NETWORK};
   use ::serialize::{FixedBitcoinSerializer, BitcoinEncodeParam};
   use std::net::SocketAddr;
   use std::str::FromStr;
   
   let m = NetworkAddress {
      services:  NODE_NETWORK,
      time:      0x01020304u32,
      sockaddr:  SocketAddr::from_str("10.0.0.1:8333").unwrap(),
   };
   let mut ser = FixedBitcoinSerializer::new(100);
   let mut ep  = BitcoinEncodeParam::new_net();

   let exp_time = [0x04, 0x03, 0x02, 0x01];
   let exp_addr = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0A, 0x00, 0x00, 0x01,
                   0x20, 0x8D];
   
   assert_matches!(ser.serialize_bitcoin(&m, &false, &ep), Ok(26usize));
   assert_eq!(exp_addr, &ser.get_ref_ref()[0..26]);

   ser.reset();
   ep.version = ::protocol::ADDRESS_TIME_VERSION - 1;
   assert_matches!(ser.serialize_bitcoin(&m, &true, &ep), Ok(26usize));
   assert_eq!(exp_addr, &ser.get_ref_ref()[0..26]);

   ser.reset();
   ep.version = ::protocol::ADDRESS_TIME_VERSION;
   assert_matches!(ser.serialize_bitcoin(&m, &true, &ep), Ok(30usize));
   assert_eq!(exp_time, &ser.get_ref_ref()[0..4]);
   assert_eq!(exp_addr, &ser.get_ref_ref()[4..30]);
}
