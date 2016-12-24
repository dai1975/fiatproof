use std;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct SendHeadersMessage;


impl Message for SendHeadersMessage {
   fn get_command(&self) -> Command { command::SEND_HEADERS }
}

impl std::fmt::Display for SendHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "SendHeaders()")
   }
}

