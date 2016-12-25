use std;
use super::message::{ Message, Command };

#[derive(Debug,Default,Clone)]
pub struct GetAddrMessage;

impl Message for GetAddrMessage {
   const COMMAND: Command = Command { data: &[0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for GetAddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetAddr()")
   }
}

