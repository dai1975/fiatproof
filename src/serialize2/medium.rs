/*
#[derive(Debug,Clone)]
pub struct Medium {
   version:   i32,
   medium:     u8,
}

const MEDIUM_NET:u8  = 1 << 0;
const MEDIUM_DISK:u8 = 1 << 1;
const MEDIUM_HASH:u8 = 1 << 2;
const MEDIUM_DUMP:u8 = 1 << 3;

impl Default for Medium {
   fn default() -> Self {
      Medium {
         version: ::protocol::PROTOCOL_VERSION,
         medium:   MEDIUM_NET,
      }
   }
}
impl Medium {
   pub fn set(&mut self, m:&Medium) {
      self.verson    = m.version;
      self.medium     = m.medium;
   }
   pub fn version(&self) -> i32  { self.version }
   pub fn is_disk(&self) -> bool { (self.medium & MEDIUM_DISK) != 0 }
   pub fn is_net(&self)  -> bool { (self.medium & MEDIUM_NET) != 0 }
   pub fn is_hash(&self) -> bool { (self.medium & MEDIUM_HASH) != 0 }
   pub fn is_dump(&self) -> bool { (self.medium & MEDIUM_DUMP) != 0 }

   pub fn set_version(&mut self, v:i32) -> &mut Self { self.version = v; self }
   pub fn set_version_latest(&mut self) -> &mut Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn clear(&mut self)              -> &mut Self { self.medium = 0; self }
   pub fn set_disk(&mut self)           -> &mut Self { self.medium |= MEDIUM_DISK; self }
   pub fn set_net(&mut self)            -> &mut Self { self.medium |= MEDIUM_NET; self }
   pub fn set_hash(&mut self)           -> &mut Self { self.medium |= MEDIUM_HASH; self }
   pub fn set_dump(&mut self)           -> &mut Self { self.medium |= MEDIUM_DUMP; self }

   pub fn unset_disk(&mut self)         -> &mut Self { self.medium &= !MEDIUM_DISK; self }
   pub fn unset_net(&mut self)          -> &mut Self { self.medium &= !MEDIUM_NET; self }
   pub fn unset_hash(&mut self)         -> &mut Self { self.medium &= !MEDIUM_HASH; self }
   pub fn unset_dump(&mut self)         -> &mut Self { self.medium &= !MEDIUM_DUMP; self }

   pub fn new(medium:&str, bo:&str) -> Self {
      let medium = match medium {
         "net"  => MEDIUM_NET,
         "disk" => MEDIUM_DISK,
         "hash" => MEDIUM_HASH,
         "dump" => MEDIUM_DUMP,
         _      => { debug_assert!(false); 0 }
      }
      Medium { 
         version: ::protocol::PROTOCOL_VERSION,
         medium:   medium,
      }
   }
}

