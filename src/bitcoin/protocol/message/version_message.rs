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

use super::message::{ Message, COMMAND_LENGTH };
impl Message for VersionMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];
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
         user_agent   : String::from(super::super::apriori::USER_AGENT),
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


use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for VersionMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_i32le(ws, self.version)?;
      r += e.serialize_u64le(ws, self.services)?;
      {
         use std::time::UNIX_EPOCH;
         use std::i64::MAX as i64_max;
         let t:u64 = match self.timestamp.duration_since(UNIX_EPOCH) {
            Ok(d)  => d.as_secs(),
            Err(_) => raise_serialize_error!("the timestamp is earler than epoch"),
         };
         if (i64_max as u64) < t {
            raise_serialize_error!("the timestamp is later than i64::MAX");
         }
         r += e.serialize_i64le(ws, t as i64)?;
      }
      r += self.addr_recv.serialize(&false, e, ws)?;
      r += self.addr_from.serialize(&false, e, ws)?;
      r += e.serialize_u64le(ws, self.nonce)?;
      {
         use super::super::apriori::MAX_SUBVERSION_LENGTH;
         r += e.serialize_var_string(ws, self.user_agent.as_str(), MAX_SUBVERSION_LENGTH)?;
      }
      r += e.serialize_i32le(ws, self.start_height)?;
      r += e.serialize_bool(ws, self.relay)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for VersionMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_i32le(rs, &mut self.version)?;
      r += d.deserialize_u64le(rs, &mut self.services)?;
      {
         let mut t:i64 = 0;
         r += d.deserialize_i64le(rs, &mut t)?;
         if t < 0 {
            raise_serialize_error!("the timestamp is earler than epoch")
         }
         use std::time::{UNIX_EPOCH, Duration};
         self.timestamp = UNIX_EPOCH + Duration::from_secs(t as u64);
      }
      r += self.addr_from.deserialize(&false, d, rs)?;
      r += self.addr_recv.deserialize(&false, d, rs)?;
      r += d.deserialize_u64le(rs, &mut self.nonce)?;
      {
         use super::super::apriori::MAX_SUBVERSION_LENGTH;
         r += d.deserialize_var_string(rs, &mut self.user_agent, MAX_SUBVERSION_LENGTH)?;
      }
      r += d.deserialize_i32le(rs, &mut self.start_height)?;
      r += d.deserialize_bool(rs, &mut self.relay)?;
      Ok(r)
   }
}


#[test]
fn test_version_message() {
   use ::bitcoin::protocol::{NetworkAddress};
   use ::bitcoin::protocol::apriori::NODE_FULL;
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

   use ::iostream::{VecWriteStream};
   use ::bitcoin::serialize::{Medium, Serializer};
   let mut w = VecWriteStream::default();
   {
      let m = Medium::new("net").unwrap();
      let e = Serializer::new(&m);
   // bitcoin-core rely on a state that version is not agreeed and set as 0 in sending or recving version message.
      assert_matches!(v.serialize(&(), &e, &mut w), Ok(98));
   }
   assert_eq!(&w.get_ref()[0..98], exp);

   // this impl impls for version message not to emit address time if runtime version is later than addr_time_version
   w.rewind();
   {
      use ::bitcoin::protocol::apriori::ADDRESS_TIME_VERSION;
      let m = Medium::new("net").unwrap().set_version(ADDRESS_TIME_VERSION);
      let e = Serializer::new(&m);
      assert_matches!(v.serialize(&(), &e, &mut w), Ok(98));
   }
   assert_eq!(&w.get_ref()[0..98], exp);
}

   
