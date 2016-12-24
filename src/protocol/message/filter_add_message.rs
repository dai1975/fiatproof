use std;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}

impl Message for FilterAddMessage {
   fn get_command(&self) -> Command { command::FILTER_ADD }
}

impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

