use std;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct SendHeadersMessage;


impl Message for SendHeadersMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x73, 0x65, 0x6e, 0x64, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00] };
}

impl std::fmt::Display for SendHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "SendHeaders()")
   }
}

