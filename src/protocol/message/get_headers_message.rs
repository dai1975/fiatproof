use std;
use ::{UInt256, BlockLocator};

#[derive(Debug,Default)]
pub struct GetHeadersMessage {
   pub locator   : BlockLocator,
   pub hash_stop : UInt256,
}

impl GetHeadersMessage {
   pub fn new(hash: &UInt256) -> GetHeadersMessage {
      GetHeadersMessage {
         locator   : BlockLocator::default(),
         hash_stop : hash.clone(),
      }
   }
}

impl std::fmt::Display for GetHeadersMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetHeaders(locator={}, stop={})", self.locator, self.hash_stop)
   }
}

