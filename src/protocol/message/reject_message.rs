use std;
use super::message::{ Message, Command };

pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;

pub const REJECT_MALFORMED:u8       = 0x01;
pub const REJECT_INVALID:u8         = 0x10;
pub const REJECT_OBSOLETE:u8        = 0x11;
pub const REJECT_DUPLICATE:u8       = 0x12;
pub const REJECT_NONSTANDARD:u8     = 0x40;
pub const REJECT_DUST:u8            = 0x41;
pub const REJECT_INSUFFICIENTFEE:u8 = 0x42;
pub const REJECT_CHECKPOINT:u8      = 0x43;


#[derive(Debug,Clone)]
pub struct RejectMessage {
   pub command : Command,
   pub code    : u8,
   pub reason  : String,
}

impl Message for RejectMessage {
   const COMMAND: Command = Command { data: &[0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl RejectMessage {
   pub fn new(msg_: &super::Message, code_:u8, reason_:&String) -> RejectMessage {
      RejectMessage {
         command: msg_.get_command(),
         code: code_,
         reason: reason_.clone(),
      }
   }
   pub fn get_command_str(&self) -> &str {
      let data = self.get_command().data;
      let s =
         match data.iter().position(|&x| x == 0) {
            Some(pos) => { &data[0..pos] }
            None      => { &data[..] }
         };
      std::str::from_utf8(s).unwrap()
   }
}

impl std::fmt::Display for RejectMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Reject(cmd={}, code={}, reason={})",
             self.get_command_str(), self.code, self.reason)
   }
}

