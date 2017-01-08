use std;
use ::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetBlocksMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

impl std::fmt::Display for GetBlocksMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetBlocks(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

