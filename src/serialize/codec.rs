#[derive(Debug,Clone)]
pub struct CodecParam {
   version: i32,
   serialize_type: i32,
}

const SER_NET:i32     = 1 << 0;
const SER_DISK:i32    = 1 << 1;
const SER_GETHASH:i32 = 1 << 2;

impl CodecParam {
   pub fn new() -> Self {
      CodecParam {
         version: ::protocol::PROTOCOL_VERSION,
         serialize_type: 0,
      }
   }
   pub fn version(&self)     -> i32  { self.version }
   pub fn is_disk(&self)     -> bool { (self.serialize_type & SER_DISK) != 0 }
   pub fn is_net(&self)      -> bool { (self.serialize_type & SER_NET) != 0 }
   pub fn is_gethash(&self)  -> bool { (self.serialize_type & SER_GETHASH) != 0 }

   pub fn set_version(&mut self, v:i32) -> &mut Self { self.version = v; self }
   pub fn set_version_latest(&mut self) -> &mut Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn clear_type(&mut self)         -> &mut Self { self.serialize_type = 0; self }
   pub fn set_disk(&mut self)           -> &mut Self { self.serialize_type |= SER_DISK; self }
   pub fn set_net(&mut self)            -> &mut Self { self.serialize_type |= SER_NET; self }
   pub fn set_gethash(&mut self)        -> &mut Self { self.serialize_type |= SER_GETHASH; self }
}

impl Default for CodecParam {
   fn default() -> Self {
      CodecParam {
         version: ::protocol::PROTOCOL_VERSION,
         serialize_type: SER_NET,
      }
   }
}

