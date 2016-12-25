use std;
use super::message::{ Message, Command };
use super::super::{ Address };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<Address>,
}
impl Message for AddrMessage {
   const COMMAND: Command = Command { data: &[0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}

