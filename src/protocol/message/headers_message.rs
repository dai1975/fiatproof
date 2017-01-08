use std;
use ::BlockHeader;

#[derive(Debug,Default,Clone)]
pub struct HeadersMessage {
   pub headers: Vec< BlockHeader >,
}

impl std::fmt::Display for HeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Headers(len={})", self.headers.len())
   }
}

