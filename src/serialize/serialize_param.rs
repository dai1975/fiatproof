use ::protocol::PROTOCOL_VERSION;

pub const SER_NET:i32     = 1 << 0;
pub const SER_DISK:i32    = 1 << 1;
pub const SER_GETHASH:i32 = 1 << 2;

#[derive(Debug,Clone)]
pub struct SerializeParam {
   pub sertype:i32,
   pub version:i32,
}

impl SerializeParam {
   pub fn new(sertype_:i32, version_:i32) -> SerializeParam {
      SerializeParam {
         sertype: sertype_,
         version: version_,
      }
   }
   pub fn new_net() -> SerializeParam {
      SerializeParam {
         sertype: SER_NET,
         version: PROTOCOL_VERSION,
      }
   }
   pub fn new_gethash() -> SerializeParam {
      SerializeParam {
         sertype: SER_GETHASH,
         version: PROTOCOL_VERSION,
      }
   }
   pub fn new_gethash_ver(ver:i32) -> SerializeParam {
      SerializeParam {
         sertype: SER_GETHASH,
         version: ver,
      }
   }
}
