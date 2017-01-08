use std;
use super::super::{ NetworkAddress };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<NetworkAddress>,
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}

