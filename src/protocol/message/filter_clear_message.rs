use std;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct FilterClearMessage;

impl Message for FilterClearMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x00] };
}

impl std::fmt::Display for FilterClearMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterClear()")
   }
}

