use std;
use super::{ Message };
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct VerAckMessage;

impl Message for VerAckMessage {
   fn get_command(&self) -> Command { command::VER_ACK }
}

impl std::fmt::Display for VerAckMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "VerAck()")
   }
}

