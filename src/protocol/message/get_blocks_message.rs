use std;
use ::{UInt256, BlockLocator};
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

impl Message for GetBlocksMessage {
   fn get_command(&self) -> Command { command::GET_BLOCKS }
}

impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

