#[derive(Debug,Clone)]
pub struct Medium {
   version: i32,
   medium:  i32,
}

const MEDIUM_NET:i32  = 1 << 0;
const MEDIUM_DISK:i32 = 1 << 1;
const MEDIUM_HASH:i32 = 1 << 2;
const MEDIUM_DUMP:i32 = 1 << 3;

impl Default for Medium {
   fn default() -> Self {
      Medium {
         //version: ::protocol::PROTOCOL_VERSION,
         version: 0,
         medium:   0,
      }
   }
}
impl Medium { 
   pub fn version(&self) -> i32  { self.version }
   pub fn is_disk(&self) -> bool { (self.medium & MEDIUM_DISK) != 0 }
   pub fn is_net(&self)  -> bool { (self.medium & MEDIUM_NET) != 0 }
   pub fn is_hash(&self) -> bool { (self.medium & MEDIUM_HASH) != 0 }
   pub fn is_dump(&self) -> bool { (self.medium & MEDIUM_DUMP) != 0 }

   pub fn set_version(mut self, v:i32) -> Self { self.version = v; self }
   //pub fn set_version_latest(mut self) -> Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn clear(mut self)              -> Self { self.medium = 0; self }
   pub fn set_disk(mut self)           -> Self { self.medium |= MEDIUM_DISK; self }
   pub fn set_net(mut self)            -> Self { self.medium |= MEDIUM_NET; self }
   pub fn set_hash(mut self)           -> Self { self.medium |= MEDIUM_HASH; self }
   pub fn set_dump(mut self)           -> Self { self.medium |= MEDIUM_DUMP; self }

   pub fn unset_disk(mut self)         -> Self { self.medium &= !MEDIUM_DISK; self }
   pub fn unset_net(mut self)          -> Self { self.medium &= !MEDIUM_NET; self }
   pub fn unset_hash(mut self)         -> Self { self.medium &= !MEDIUM_HASH; self }
   pub fn unset_dump(mut self)         -> Self { self.medium &= !MEDIUM_DUMP; self }

   pub fn new(s:&str) -> Self {
      match s {
         "net"  => Medium::default().set_net(),
         "disk" => Medium::default().set_disk(),
         "hash" => Medium::default().set_hash(),
         "dump" => Medium::default().set_dump(),
         _      => Medium::default(),
      }
   }
}

