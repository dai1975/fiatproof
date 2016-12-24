use std;
extern crate time;
use super::{ Message };
use super::super::{ command, Command, Address };

#[derive(Debug,Clone)]
pub struct VersionMessage<'a> {
   pub version        : i32,
   pub services       : u64,
   pub timestamp      : i64,
   pub addr_recv      : Address,
   pub addr_from      : Address,
   pub nonce          : u64,
   pub user_agent     : &'a str,
   pub start_height   : i32,
   pub relay          : bool,
}

impl <'a> Message for VersionMessage<'a> {
   fn get_command(&self) -> Command { command::VERSION }
}

impl <'a> Default for VersionMessage<'a> {
   fn default() -> VersionMessage<'a> {
      VersionMessage {
         version      : 0,
         services     : 0,
         timestamp    : time::get_time().sec,
         addr_recv    : Address::new(0),
         addr_from    : Address::new(0),
         nonce        : 0,
         user_agent   : ::apriori::user_agent::USER_AGENT,
         start_height : 0,
         relay        : false,
      }
   }
}

impl <'a> std::fmt::Display for VersionMessage<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Version(ver={}, blocks={}, us={}, them={}, ua={})", self.version, self.start_height, self.addr_recv, self.addr_from, self.user_agent)
   }
}

