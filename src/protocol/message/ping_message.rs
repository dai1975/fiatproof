use std;
extern crate rand;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct PingMessage
{
   pub nonce: u64,
}

impl Message for PingMessage {
   fn get_command(&self) -> Command { command::PING }
}

impl std::fmt::Display for PingMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Ping(nonce={})", self.nonce)
   }
}

impl PingMessage {
   pub fn reset_nonce(&mut self) {
      use self::rand::Rng;
      let mut rng = rand::os::OsRng::new().unwrap(); // This rng is cryptographic level, is it too secure?
      self.nonce = rng.next_u64();
   }
}

