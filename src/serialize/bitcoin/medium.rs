use ::error::ParseError;

#[derive(Debug,Clone)]
pub struct Medium {
   version: i32,
   medium:  u32,
}

// IO: 1bit
const MASK_IO:u32     = 0x01u32 << 0;
const MEDIUM_NET:u32  = 0x00u32 << 0;
const MEDIUM_DISK:u32 = 0x01u32 << 0;
// TRIM: 1bit
const MEDIUM_TRIM:u32 = 0x01u32 << 1;

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
   pub fn is_net(&self)  -> bool { (self.medium & MASK_IO) == MEDIUM_NET }
   pub fn is_disk(&self) -> bool { (self.medium & MASK_IO) == MEDIUM_DISK }
   pub fn is_trim(&self) -> bool { (self.medium & MEDIUM_TRIM) != 0 }

   pub fn set_version(mut self, v:i32) -> Self { self.version = v; self }
   //pub fn set_version_latest(mut self) -> Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn clear(mut self)    -> Self { self.medium = 0; self }
   pub fn set_net(mut self)  -> Self { self.medium = self.medium & !MASK_IO | MEDIUM_NET; self }
   pub fn set_disk(mut self) -> Self { self.medium = self.medium & !MASK_IO | MEDIUM_DISK; self }
   pub fn set_trim(mut self) -> Self { self.medium |= MEDIUM_TRIM; self }

   pub fn unset_trim(mut self)         -> Self { self.medium &= !MEDIUM_TRIM; self }

   pub fn new(line:&str) -> Result<Self, ::ParseError> {
      let m0 = Medium { version:Medium::default().version, medium:0 };
      let m = line.split(',').fold(Ok(m0), |m,s| {
         println!("medium...{}", s);
         match (m,s) {
            (Err(e), _) => Err(e),
            (Ok(m), "disk") => Ok(m.set_disk()),
            (Ok(m), "net")  => Ok(m.set_net()),
            (Ok(m), "trim") => Ok(m.set_trim()),
            (Ok(_), _)      => Err(::ParseError::new(format!("unknown medium {:?}", s))),
         }
      });
      m
   }
}


#[test]
fn test_set() {
   use super::Medium;
   let mut m = Medium { version:0, medium: 0};
   assert_eq!(m.medium, 0x00);
   assert_eq!(0u32 & !1u32 | 0u32, 0u32);
   assert_eq!((0u32 & 1u32) & 0u32, 0u32);
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);
   assert_eq!(m.is_trim(), false);

   let mut m = m.set_net();
   assert_eq!(m.medium, MEDIUM_NET);
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);
   assert_eq!(m.is_trim(), false);

   let mut m = m.set_disk();
   assert_eq!(m.medium, MEDIUM_DISK);
   assert_eq!(m.is_net(),  false);
   assert_eq!(m.is_disk(), true);
   assert_eq!(m.is_trim(), false);

   let mut m = m.set_trim();
   assert_eq!(m.medium, MEDIUM_DISK | MEDIUM_TRIM);
   assert_eq!(m.is_net(),  false);
   assert_eq!(m.is_disk(), true);
   assert_eq!(m.is_trim(), true);

   let mut m = m.set_net();
   assert_eq!(m.medium, MEDIUM_NET | MEDIUM_TRIM);
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);
   assert_eq!(m.is_trim(), true);
}

#[test]
fn test_new() {
   use super::Medium;
   let m = Medium::new("net,trim").unwrap();
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);
   assert_eq!(m.is_trim(), true);
}
