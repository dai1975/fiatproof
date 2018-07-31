use std;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug,Clone)]
pub struct NetworkAddress {
   pub services:  u64,
   pub time:      u32,
   pub sockaddr:  SocketAddr,
}

impl Default for NetworkAddress {
   fn default() -> Self {
      NetworkAddress {
         services: 0,
         time:     0,
         sockaddr: SocketAddr::from_str("0.0.0.0:0").unwrap(),
      }
   }
}
impl std::fmt::Display for NetworkAddress {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "addr={}, time={}", self.sockaddr, self.time)
   }
}
impl NetworkAddress {
   pub fn new(services_:u64) -> Self {
      NetworkAddress{
         services: services_,
         time: 0,
         sockaddr: SocketAddr::from_str("127.0.0.1:0").unwrap(),
      }
   }
}

use ::serialize::{ WriteStream, ReadStream };
use ::bitcoin::encode::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};

pub struct NetworkAddressEncodee<'a>(pub &'a NetworkAddress, pub bool);
impl <'a> BitcoinEncodee for NetworkAddressEncodee<'a> {
   type P = ();
   fn encode(&self, _p:&Self::P, e:&BitcoinEncoder, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      let version = e.medium().version();
      
      if e.medium().is_disk() {
         r += try!(e.encode_i32le(ws, version));
      }
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if e.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !e.medium().is_hash() && self.1)
         {
            r += try!(e.encode_u32le(ws, self.0.time));
         }
      }
      r += try!(e.encode_u64le(ws, self.0.services));

      {
         use std::net::IpAddr;
         let v6 = match self.0.sockaddr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
         };
         r += try!(e.encode_octets(ws, &v6.octets()));
      }
      r += try!(e.encode_u16be(ws, self.0.sockaddr.port())); //network byte order
      Ok(r)
   }
}

#[derive(Default)]
pub struct NetworkAddressDecodee(pub NetworkAddress, pub bool);

impl BitcoinDecodee for NetworkAddressDecodee {
   type P = ();
   fn decode(&mut self, _p:&Self::P, d:&BitcoinDecoder, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut version = d.medium().version();
      
      if d.medium().is_disk() {
         r += try!(d.decode_i32le(rs, &mut version));
      }
      
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if d.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !d.medium().is_hash() && self.1)
         {
            r += try!(d.decode_u32le(rs, &mut self.0.time));
         }
      }

      r += try!(d.decode_u64le(rs, &mut self.0.services));

      {
         use std::net::{IpAddr, Ipv6Addr};
         let mut octets = [0u8; 16];
         r += try!(d.decode_octets(rs, &mut octets));
         let v6 = Ipv6Addr::from(octets);
         self.0.sockaddr.set_ip(match v6.to_ipv4() {
            Some(v4) => IpAddr::V4(v4),
            None     => IpAddr::V6(v6),
         });
      }
      
      {
         let mut port:u16 = 0;
         r += try!(d.decode_u16be(rs, &mut port));
         self.0.sockaddr.set_port(port);
      }
      Ok(r)
   }
}

#[test]
fn test_address() {
   use ::bitcoin::protocol::{NetworkAddress, NetworkAddressEncodee};
   use ::bitcoin::protocol::apriori::{NODE_FULL, ADDRESS_TIME_VERSION};
   use std::net::SocketAddr;
   use std::str::FromStr;
   
   let obj = NetworkAddress {
      services:  NODE_FULL,
      time:      0x01020304u32,
      sockaddr:  SocketAddr::from_str("10.0.0.1:8333").unwrap(),
   };

   let exp_time = [0x04, 0x03, 0x02, 0x01];
   let exp_addr = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0A, 0x00, 0x00, 0x01,
                   0x20, 0x8D];
   
   use ::serialize::{VecWriteStream};
   use ::bitcoin::serialize::{Medium, Encoder};
   let mut w = VecWriteStream::default();
   {
      let m = Medium::new("net").unwrap().set_version(ADDRESS_TIME_VERSION);
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(NetworkAddressEncodee(&obj, true).encode(&mut e), Ok(30usize));
      assert_matches!(NetworkAddressEncodee(&obj, false).encode(&mut e), Ok(26usize));
   }
   assert_eq!(exp_time, &w.get_ref()[0..4]);
   assert_eq!(exp_addr, &w.get_ref()[4..30]);
   assert_eq!(exp_addr, &w.get_ref()[30..56]);

   w.rewind();
   {
      let m = Medium::new("net").unwrap().set_version(ADDRESS_TIME_VERSION - 1);
      let mut e = Encoder::new(&mut w, &m);
      assert_matches!(NetworkAddressEncodee(&obj, true).encode(&mut e), Ok(26usize));
      assert_matches!(NetworkAddressEncodee(&obj, false).encode(&mut e), Ok(26usize));
   }
   assert_eq!(exp_addr, &w.get_ref()[0..26]);
   assert_eq!(exp_addr, &w.get_ref()[26..52]);
}
