use std;

#[derive(Debug,Clone,Default)]
pub struct AlertMessage {
   pub msg: Vec<u8>,
   pub sig: Vec<u8>,
}

impl std::fmt::Display for AlertMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Alert(msg={:?}, sig={})", self.msg, self.sig.len())
   }
}

