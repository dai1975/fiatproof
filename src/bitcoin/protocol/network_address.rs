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

use crate::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};

//pub struct NetworkAddressSerializee<'a>(pub &'a NetworkAddress, pub bool);
//impl <'a> BitcoinSerializee for NetworkAddressSerializee<'a> {
impl BitcoinSerializee for NetworkAddress {
   type P = bool; // whether output time or not
   fn serialize<W: std::io::Write>(&self, p:&Self::P, e:&BitcoinSerializer, ws:&mut W) -> crate::Result<usize> {
      let mut r:usize = 0;
      let version = e.medium().version();
      
      if e.medium().is_disk() {
         r += e.serialize_i32le(ws, version)?;
      }
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if e.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !e.medium().is_hash() && *p)
         {
            r += e.serialize_u32le(ws, self.time)?;
         }
      }
      r += e.serialize_u64le(ws, self.services)?;

      {
         use std::net::IpAddr;
         let v6 = match self.sockaddr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
         };
         r += e.serialize_octets(ws, &v6.octets())?;
      }
      r += e.serialize_u16be(ws, self.sockaddr.port())?; //network byte order
      Ok(r)
   }
}

//#[derive(Default)]
//pub struct NetworkAddressDeserializee(pub NetworkAddress, pub bool);
//impl BitcoinDeserializee for NetworkAddressDeserializee {

impl BitcoinDeserializee for NetworkAddress {
   type P = bool;
   fn deserialize<R: std::io::Read>(&mut self, p:&Self::P, d:&BitcoinDeserializer, rs:&mut R) -> crate::Result<usize> {
      let mut r:usize = 0;
      let mut version = d.medium().version();
      
      if d.medium().is_disk() {
         r += d.deserialize_i32le(rs, &mut version)?;
      }
      
      {
         use super::apriori::ADDRESS_TIME_VERSION;
         if d.medium().is_disk()
            || (ADDRESS_TIME_VERSION <= version && !d.medium().is_hash() && *p)
         {
            r += d.deserialize_u32le(rs, &mut self.time)?;
         }
      }

      r += d.deserialize_u64le(rs, &mut self.services)?;

      {
         use std::net::{IpAddr, Ipv6Addr};
         let mut octets = [0u8; 16];
         r += d.deserialize_octets(rs, &mut octets)?;
         let v6 = Ipv6Addr::from(octets);
         self.sockaddr.set_ip(match v6.to_ipv4() {
            Some(v4) => IpAddr::V4(v4),
            None     => IpAddr::V6(v6),
         });
      }
      
      {
         let mut port:u16 = 0;
         r += d.deserialize_u16be(rs, &mut port)?;
         self.sockaddr.set_port(port);
      }
      Ok(r)
   }
}

#[test]
fn test_address() {
   use crate::bitcoin::protocol::{NetworkAddress};
   use crate::bitcoin::protocol::apriori::{NODE_FULL, ADDRESS_TIME_VERSION};
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
   
   use crate::bitcoin::serialize::{Medium, Serializer};
   let mut v = Vec::<u8>::new();
   {
      let m = Medium::new("net").unwrap().set_version(ADDRESS_TIME_VERSION);
      let e = Serializer::new(&m);
      assert_matches!(obj.serialize(&true,  &e, &mut v), Ok(30usize));
      assert_matches!(obj.serialize(&false, &e, &mut v), Ok(26usize));
   }
   assert_eq!(exp_time, &v[0..4]);
   assert_eq!(exp_addr, &v[4..30]);
   assert_eq!(exp_addr, &v[30..56]);

   let mut v = Vec::<u8>::new();
   {
      let m = Medium::new("net").unwrap().set_version(ADDRESS_TIME_VERSION - 1);
      let e = Serializer::new(&m);
      assert_matches!(obj.serialize(&true,  &e, &mut v), Ok(26usize));
      assert_matches!(obj.serialize(&false, &e, &mut v), Ok(26usize));
   }
   assert_eq!(exp_addr, &v[0..26]);
   assert_eq!(exp_addr, &v[26..52]);
}
