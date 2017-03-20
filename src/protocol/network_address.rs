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
      write!(f, "addr={}, t={}", self.sockaddr, self.time)
   }
}

impl NetworkAddress {
   pub fn new(services_:u64) -> Self {
      NetworkAddress{
         services: services_,
         time:     0,
         sockaddr: SocketAddr::from_str("127.0.0.1:0").unwrap()
      }
   }
}


use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
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

#[test]
fn test_address() {
   use ::protocol::{NetworkAddress, NODE_FULL};
   use std::net::SocketAddr;
   use std::str::FromStr;
   
   let v = NetworkAddress {
      services:  NODE_FULL,
      time:      0x01020304u32,
      sockaddr:  SocketAddr::from_str("10.0.0.1:8333").unwrap(),
   };

   let exp_time = [0x04, 0x03, 0x02, 0x01];
   let exp_addr = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0A, 0x00, 0x00, 0x01,
                   0x20, 0x8D];
   
   use ::codec::{BitcoinEncodeStream, VecWriteStream, Media};
   let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
   assert_matches!(v.encode(&mut e, false), Ok(26usize));
   assert_eq!(exp_addr, &e.w.get_ref()[0..26]);

   e.w.rewind();
   e.update_media(|m| m.set_version(::protocol::ADDRESS_TIME_VERSION - 1));
   assert_matches!(v.encode(&mut e, true), Ok(26usize));
   assert_eq!(exp_addr, &e.w.get_ref()[0..26]);

   e.w.rewind();
   e.update_media(|m| m.set_version(::protocol::ADDRESS_TIME_VERSION));
   assert_matches!(v.encode(&mut e, true), Ok(30usize));
   assert_eq!(exp_time, &e.w.get_ref()[0..4]);
   assert_eq!(exp_addr, &e.w.get_ref()[4..30]);
}
