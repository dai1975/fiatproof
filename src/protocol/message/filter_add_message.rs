use std;

#[derive(Debug,Default,Clone)]
pub struct FilterAddMessage {
   pub data: Vec<u8>,
}

impl std::fmt::Display for FilterAddMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterAdd(data={:?})", self.data)
   }
}

