use std;
use super::super::{ NetworkAddress };

#[derive(Debug,Clone)]
pub struct VersionMessage {
   pub version        : i32,
   pub services       : u64,
   pub timestamp      : std::time::SystemTime,
   pub addr_recv      : NetworkAddress,
   pub addr_from      : NetworkAddress,
   pub nonce          : u64,
   pub user_agent     : String,
   pub start_height   : i32,
   pub relay          : bool,
}

impl Default for VersionMessage {
   fn default() -> VersionMessage {
      VersionMessage {
         version      : 0,
         services     : 0,
         timestamp    : std::time::UNIX_EPOCH,
         addr_recv    : NetworkAddress::new(0),
         addr_from    : NetworkAddress::new(0),
         nonce        : 0,
         user_agent   : String::from(::apriori::user_agent::USER_AGENT),
         start_height : 0,
         relay        : false,
      }
   }
}

impl std::fmt::Display for VersionMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Version(ver={}, blocks={}, us={}, them={}, ua={})", self.version, self.start_height, self.addr_recv, self.addr_from, self.user_agent)
   }
}


use ::std::borrow::Borrow;
use ::serialize::{EncodeStream, Encodee, DecodeStream, Decodee};
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


#[test]
fn test_version_message() {
   use ::protocol::{NetworkAddress, NODE_FULL};
   use ::std::net::SocketAddr;
   use ::std::str::FromStr;
   use ::std::time::{Duration, UNIX_EPOCH};
   
   let v = VersionMessage {
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

   use ::serialize::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net().set_version(0));
   // bitcoin-core rely on a state that version is not agreeed and set as 0 in sending or recving version message.

   assert_matches!(v.encode(&mut e, ()), Ok(98));
   assert_eq!(&e.w.get_ref()[0..98], exp);

   // this impl impls for version message not to emit address time if runtime version is later than addr_time_version
   e.w.rewind();
   e.update_media(|m| m.set_version(::protocol::ADDRESS_TIME_VERSION));
   assert_matches!(v.encode(&mut e, ()), Ok(98));
   assert_eq!(&e.w.get_ref()[0..98], exp);
}

   
