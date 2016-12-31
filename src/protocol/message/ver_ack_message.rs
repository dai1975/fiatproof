use std;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct VerAckMessage;

impl Message for VerAckMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for VerAckMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "VerAck()")
   }
}

