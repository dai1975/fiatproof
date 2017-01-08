use std;
use super::super::Inv;

#[derive(Debug,Default,Clone)]
pub struct NotFoundMessage {
   pub invs : Vec<Inv>,
}

impl std::fmt::Display for NotFoundMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "NotFound(len={}, 0={}", self.invs.len(), self.invs[0])
   }
}

