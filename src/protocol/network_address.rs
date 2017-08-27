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

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};

pub struct NetworkAddressEncodee<'a>(&'a NetworkAddress, bool);
impl <'a> BitcoinEncodee for NetworkAddressEncodee<'a> {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      let version = e.medium().version();
      
      if e.medium().is_disk() {
         r += try!(e.encode_i32le(version));
      }
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if e.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !e.medium().is_hash() && self.1)
         {
            r += try!(e.encode_u32le(self.0.time));
         }
      }
      r += try!(e.encode_u64le(self.0.services));

      {
         use std::net::IpAddr;
         let v6 = match self.0.sockaddr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
         };
         r += try!(e.encode_octets(&v6.octets()));
      }
      r += try!(e.encode_u16be(self.0.sockaddr.port())); //network byte order
      Ok(r)
   }
}

pub struct NetworkAddressDecodee<'a>(&'a mut NetworkAddress, bool);
impl <'a> BitcoinDecodee for NetworkAddressDecodee<'a> {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut version = d.medium().version();
      
      if d.medium().is_disk() {
         r += try!(d.decode_i32le(&mut version));
      }
      
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if d.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !d.medium().is_hash() && self.1)
         {
            r += try!(d.decode_u32le(&mut self.0.time));
         }
      }

      r += try!(d.decode_u64le(&mut self.0.services));

      {
         use std::net::{IpAddr, Ipv6Addr};
         let mut octets = [0u8; 16];
         r += try!(d.decode_octets(&mut octets));
         let v6 = Ipv6Addr::from(octets);
         self.0.sockaddr.set_ip(match v6.to_ipv4() {
            Some(v4) => IpAddr::V4(v4),
            None     => IpAddr::V6(v6),
         });
      }
      
      {
         let mut port:u16 = 0;
         r += try!(d.decode_u16be(&mut port));
         self.0.sockaddr.set_port(port);
      }
      Ok(r)
   }
}

#[test]
fn test_address() {
   use ::protocol::{NetworkAddress, NetworkAddressEncodee, NetworkAddressDecodee};
   use ::protocol::apriori::{NODE_FULL, ADDRESS_TIME_VERSION};
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
   use ::serialize::bitcoin::{Medium, Encoder};
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
