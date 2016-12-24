use std;
use ::BlockHeader;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct HeadersMessageElement {
   pub header: BlockHeader,
}

#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< HeadersMessageElement >,
}

impl Message for HeadersMessage {
   fn get_command(&self) -> Command { command::HEADERS }
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

