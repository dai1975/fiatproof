use std;
use super::{ Message };
use super::super::{ command, Command, Address };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<Address>,
}
impl Message for AddrMessage {
   fn get_command(&self) -> Command { command::ADDR }
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}

