use ::protocol::PROTOCOL_VERSION;

pub const SER_NET:i32     = 1 << 0;
pub const SER_DISK:i32    = 1 << 1;
pub const SER_GETHASH:i32 = 1 << 2;

#[derive(Debug,Clone)]
pub struct BitcoinEncodeParam {
   pub sertype:i32,
   pub version:i32,
}

impl BitcoinEncodeParam {
   pub fn new(sertype_:i32, version_:i32) -> Self {
      BitcoinEncodeParam {
         sertype: sertype_,
         version: version_,
      }
   }
   pub fn new_net() -> Self {
      BitcoinEncodeParam {
         sertype: SER_NET,
         version: PROTOCOL_VERSION,
      }
   }
   pub fn new_gethash() -> Self {
      BitcoinEncodeParam {
         sertype: SER_GETHASH,
         version: PROTOCOL_VERSION,
      }
   }
   pub fn new_gethash_ver(ver:i32) -> Self {
      BitcoinEncodeParam {
         sertype: SER_GETHASH,
         version: ver,
      }
   }

   pub fn is_net(&self) -> bool {
      (self.sertype & SER_NET) == SER_NET
   }
   pub fn is_disk(&self) -> bool {
      (self.sertype & SER_DISK) == SER_DISK
   }
   pub fn is_gethash(&self) -> bool {
      (self.sertype & SER_GETHASH) == SER_GETHASH
   }
}
