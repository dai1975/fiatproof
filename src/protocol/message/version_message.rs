use std;
use super::super::{ NetworkAddress };

#[derive(Debug,Clone)]
pub struct VersionMessage {
   pub version        : i32,
   pub services       : u64,
   pub timestamp      : std::time::SystemTime,
   pub addr_recv      : NetworkAddress,
   pub addr_from      : NetworkAddress,
   pub nonce          : u64,
   pub user_agent     : String,
   pub start_height   : i32,
   pub relay          : bool,
}

impl Default for VersionMessage {
   fn default() -> VersionMessage {
      VersionMessage {
         version      : 0,
         services     : 0,
         timestamp    : std::time::UNIX_EPOCH,
         addr_recv    : NetworkAddress::new(0),
         addr_from    : NetworkAddress::new(0),
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

