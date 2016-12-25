use std;
use ::BlockHeader;
use super::message::{ Message, Command };

#[derive(Debug,Default,Clone)]
pub struct HeadersMessageElement {
   pub header: BlockHeader,
}

#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< HeadersMessageElement >,
}

impl Message for HeadersMessage {
   const COMMAND: Command = Command { data: &[0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for HeadersMessageElement {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      self.header.fmt(f)
   }
}

impl std::fmt::Display for HeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Headers(len={})", self.headers.len())
   }
}

