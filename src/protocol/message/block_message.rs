use std;
use ::Block;
use super::message::{ Message, MessageCommand };

#[derive(Debug,Default,Clone)]
pub struct BlockMessage {
   pub block: Block,
}

impl Message for BlockMessage {
   const COMMAND: MessageCommand = MessageCommand { data: &[0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl std::fmt::Display for BlockMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Block({})", self.block)
   }
}

