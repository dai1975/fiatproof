use ::{Error};
use super::super::{BitcoinEncoder, BitcoinEncodee, BitcoinSerializer, WriteStream, SerializeError, LimitedString};

use ::protocol::MessageHeader;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for MessageHeader {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.magic));
      r += try!(e.encode_array_u8(self.command.data));
      r += try!(e.encode_u32le(self.length));
      r += try!(e.encode_u32le(self.checksum));
      Ok(r)
   }
}

use ::protocol::NetworkAddress;
struct TimeNetworkAddress<'a>(&'a NetworkAddress, bool);
impl <'a, W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for TimeNetworkAddress<'a> {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use std::net::IpAddr;
      use ::protocol::ADDRESS_TIME_VERSION;
      let mut r:usize = 0;
      if e.param().is_disk() {
         let version = e.param().version();
         r += try!(e.encode_i32le(version));
      }
      if e.param().is_disk() || (self.1 && !e.param().is_gethash() && (ADDRESS_TIME_VERSION <= e.param().version())) {
         r += try!(e.encode_u32le(self.0.time));
      }
      r += try!(e.encode_u64le(self.0.services));
      match self.0.sockaddr.ip() {
         IpAddr::V4(v4) => {
            r += try!(e.encode_array_u8(&[0,0,0,0,0,0,0,0,0,0,0xff,0xff]));
            r += try!(e.encode_array_u8(&v4.octets()));
         },
         IpAddr::V6(v6) => {
            r += try!(e.encode_array_u8(&v6.octets()));
         }
      }
      r += try!(e.encode_u16be(self.0.sockaddr.port())); //network byte order
      Ok(r)
   }
}
   
use ::protocol::{InvType, Inv};
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for InvType {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => serialize_error!("malformed inv type"),
      };
      e.encode_u32le(tmp)
   }
}
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for Inv {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.invtype));
      r += try!(e.encode(&self.hash));
      Ok(r)
   }
}

use ::protocol::VersionMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for VersionMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::MAX_SUBVERSION_LENGTH;
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
      r += try!(e.encode(&TimeNetworkAddress(&self.addr_recv, false)));
      r += try!(e.encode(&TimeNetworkAddress(&self.addr_from, false)));
      r += try!(e.encode_u64le(self.nonce));
      r += try!(e.encode(&LimitedString(self.user_agent.as_str(), MAX_SUBVERSION_LENGTH)));
      r += try!(e.encode_i32le(self.start_height));
      r += try!(e.encode_bool(self.relay));
      Ok(r)
   }
}

use ::protocol::VerAckMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for VerAckMessage {
   fn encode(&self, _e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::AddrMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for AddrMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::MAX_ADDR_SIZE;
      let mut r:usize = 0;
      for a in self.addrs.iter().take(MAX_ADDR_SIZE) {
         r += try!(e.encode(&TimeNetworkAddress(a, true)));
      }
      Ok(r)
   }
}

use ::protocol::InvMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for InvMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::MAX_INV_SIZE;
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs[..MAX_INV_SIZE]));
      Ok(r)
   }
}

use ::protocol::GetDataMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for GetDataMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::MAX_INV_SIZE;
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs[..MAX_INV_SIZE]));
      Ok(r)
   }
}

use ::protocol::MerkleBlockMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for MerkleBlockMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.block));
      Ok(r)
   }
}

use ::protocol::GetBlocksMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for GetBlocksMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.locator));
      r += try!(e.encode(&self.hash_stop));
      Ok(r)
   }
}

use ::protocol::GetHeadersMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for GetHeadersMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.locator));
      r += try!(e.encode(&self.hash_stop));
      Ok(r)
   }
}

use ::protocol::TxMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for TxMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.tx));
      Ok(r)
   }
}

use ::protocol::HeadersMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for HeadersMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.headers));
      r += try!(e.encode_varint(0u64));
      Ok(r)
   }
}

use ::protocol::BlockMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for BlockMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode(&self.block));
      Ok(r)
   }
}

use ::protocol::GetAddrMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for GetAddrMessage {
   fn encode(&self, _e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::MemPoolMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for MemPoolMessage {
   fn encode(&self, _e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::{PingMessage};
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for PingMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::BIP0031_VERSION;
      let mut r:usize = 0;
      if BIP0031_VERSION < e.param().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::{PongMessage};
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for PongMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      use ::protocol::BIP0031_VERSION;
      let mut r:usize = 0;
      if BIP0031_VERSION < e.param().version() {
         r += try!(e.encode_u64le(self.nonce));
      }
      Ok(r)
   }
}

use ::protocol::AlertMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for AlertMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.msg[..]));
      r += try!(e.encode_sequence_u8(&self.sig[..]));
      Ok(r)
   }
}

use ::protocol::NotFoundMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for NotFoundMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence(&self.invs[..]));
      Ok(r)
   }
}

use ::protocol::FilterLoadMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for FilterLoadMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      r += try!(e.encode_u32le(self.hash_funcs));
      r += try!(e.encode_u32le(self.tweak));
      r += try!(e.encode_u8(self.flags));
      Ok(r)
   }
}

use ::protocol::FilterAddMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for FilterAddMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      r += try!(e.encode_sequence_u8(&self.data[..]));
      Ok(r)
   }
}

use ::protocol::FilterClearMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for FilterClearMessage {
   fn encode(&self, _e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      Ok(0usize)
   }
}

use ::protocol::RejectMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for RejectMessage {
   fn encode(&self, e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      let mut r:usize = 0;
      // https://en.bitcoin.it/wiki/Protocol_documentation#reject
      // ここは var_str で任意の文字列が入るようになっているようだ。
      // 現状、command に固定している API はよろしくない? command にしても、後続の 0 は送られるのだろうか。
      r += try!(e.encode(&LimitedString(self.command.as_str(), 100)));
      r += try!(e.encode_u8(self.code));
      r += try!(e.encode(&LimitedString(self.reason.as_str(), RejectMessage::MAX_REJECT_MESSAGE_LENGTH)));
      // この後に拡張データがあるようだ。decode はメッセージヘッダのサイズを見ないと分からなさそう。
      Ok(r)
   }
}
   
use ::protocol::SendHeadersMessage;
impl <W:WriteStream> BitcoinEncodee< BitcoinSerializer<W> > for SendHeadersMessage {
   fn encode(&self, _e:&mut BitcoinSerializer<W>) -> Result<usize, Error> {
      Ok(0usize)
   }
}


#[test]
fn test_message_header() {
   use ::protocol::Message;
   use ::serialize::{FixedBitcoinSerializer};
   let m = MessageHeader {
      magic:    ::MAIN_PARAMS.magic,
      command:  <VersionMessage as Message>::COMMAND,
      length:   0x39,
      checksum: 0x12345678,
   };
   let mut ser = FixedBitcoinSerializer::new(100);
   ser.mut_param().set_net();
   assert_matches!(ser.encode(&m), Ok(24usize));
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
   
   assert_matches!(ser.encode(&TimeNetworkAddress(&m, false)), Ok(26usize));
   assert_eq!(exp_addr, &ser.as_slice()[0..26]);

   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION - 1);
   assert_matches!(ser.encode(&TimeNetworkAddress(&m, true)), Ok(26usize));
   assert_eq!(exp_addr, &ser.as_slice()[0..26]);

   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION);
   assert_matches!(ser.encode(&TimeNetworkAddress(&m, true)), Ok(30usize));
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
   assert_matches!(ser.encode(&m), Ok(98));
   assert_eq!(exp, &ser.as_slice()[0..98]);

   // this impl impls for version message not to emit address time if runtime version is later than addr_time_version
   ser.rewind();
   ser.mut_param().set_version(::protocol::ADDRESS_TIME_VERSION);
   assert_matches!(ser.encode(&m), Ok(98));
   assert_eq!(exp, &ser.as_slice()[0..98]);
}

   
