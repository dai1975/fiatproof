use std;
use super::{Message, PingMessage};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct PongMessage
{
   pub nonce: u64,
}

impl Message for PongMessage {
   fn get_command(&self) -> Command { command::PONG }
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

