use std;
use ::Block;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}

impl Message for BlockMessage {
   fn get_command(&self) -> Command { command::BLOCK }
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}

