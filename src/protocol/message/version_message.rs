use std;
extern crate time;
use super::message::{ Message, Command };
use super::super::{ Address };

#[derive(Debug,Clone)]
pub struct VersionMessage {
   pub version        : i32,
   pub services       : u64,
   pub timestamp      : i64,
   pub addr_recv      : Address,
   pub addr_from      : Address,
   pub nonce          : u64,
   pub user_agent     : String,
   pub start_height   : i32,
   pub relay          : bool,
}

impl Message for VersionMessage {
   const COMMAND: Command = Command { data: &[0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl Default for VersionMessage {
   fn default() -> VersionMessage {
      VersionMessage {
         version      : 0,
         services     : 0,
         timestamp    : time::get_time().sec,
         addr_recv    : Address::new(0),
         addr_from    : Address::new(0),
         nonce        : 0,
         user_agent   : String::from(::apriori::user_agent::USER_AGENT),
         start_height : 0,
         relay        : false,
      }
   }
}

impl std::fmt::Display for VersionMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Version(ver={}, blocks={}, us={}, them={}, ua={})", self.version, self.start_height, self.addr_recv, self.addr_from, self.user_agent)
   }
}

