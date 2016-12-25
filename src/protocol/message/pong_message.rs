use std;
use super::message::{ Message, Command };
use super::PingMessage;

#[derive(Debug,Default,Clone)]
pub struct PongMessage
{
   pub nonce: u64,
}

impl Message for PongMessage {
   const COMMAND: Command = Command { data: &[0x70, 0x6f, 0x6e, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for PongMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Pong(nonce={})", self.nonce)
   }
}

impl PongMessage {
   pub fn new(ping:&PingMessage) -> PongMessage {
      PongMessage{ nonce: ping.nonce }
   }
}

