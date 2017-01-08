use std;

#[derive(Debug,Default,Clone)]
pub struct VerAckMessage;

impl std::fmt::Display for VerAckMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "VerAck()")
   }
}

