use std;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}

impl Message for FilterAddMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x61, 0x64, 0x64, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

