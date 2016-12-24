use std;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct MemPoolMessage;

impl Message for MemPoolMessage {
   fn get_command(&self) -> Command { command::MEM_POOL }
}

impl std::fmt::Display for MemPoolMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "MemPool()")
   }
}
