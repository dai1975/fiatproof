use std;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct MemPoolMessage;

impl Message for MemPoolMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x6d, 0x65, 0x6d, 0x70, 0x6f, 0x6f, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for MemPoolMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MemPool()")
   }
}
