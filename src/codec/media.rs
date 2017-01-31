#[derive(Debug,Clone)]
pub struct Media {
   version: i32,
   media:   i32,
}

const MEDIA_NET:i32  = 1 << 0;
const MEDIA_DISK:i32 = 1 << 1;
const MEDIA_HASH:i32 = 1 << 2;
const MEDIA_DUMP:i32 = 1 << 3;

impl Default for Media {
   fn default() -> Self {
      Media {
         version: ::protocol::PROTOCOL_VERSION,
         media:   0,
      }
   }
}
impl Media { 
   pub fn version(&self) -> i32  { self.version }
   pub fn is_disk(&self) -> bool { (self.media & MEDIA_DISK) != 0 }
   pub fn is_net(&self)  -> bool { (self.media & MEDIA_NET) != 0 }
   pub fn is_hash(&self) -> bool { (self.media & MEDIA_HASH) != 0 }
   pub fn is_dump(&self) -> bool { (self.media & MEDIA_DUMP) != 0 }

   pub fn set_version(mut self, v:i32) -> Self { self.version = v; self }
   pub fn set_version_latest(mut self) -> Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn clear(mut self)              -> Self { self.media = 0; self }
   pub fn set_disk(mut self)           -> Self { self.media |= MEDIA_DISK; self }
   pub fn set_net(mut self)            -> Self { self.media |= MEDIA_NET; self }
   pub fn set_hash(mut self)           -> Self { self.media |= MEDIA_HASH; self }
   pub fn set_dump(mut self)           -> Self { self.media |= MEDIA_DUMP; self }

   pub fn unset_disk(mut self)         -> Self { self.media &= !MEDIA_DISK; self }
   pub fn unset_net(mut self)          -> Self { self.media &= !MEDIA_NET; self }
   pub fn unset_hash(mut self)         -> Self { self.media &= !MEDIA_HASH; self }
   pub fn unset_dump(mut self)         -> Self { self.media &= !MEDIA_DUMP; self }
}

