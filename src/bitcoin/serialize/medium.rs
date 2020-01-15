#[derive(Debug,Clone)]
pub struct Medium {
   medium:  u32,
}

// IO: 1bit
const MASK_IO:u32     = 3u32 << 0;
const MEDIUM_NET:u32  = 0u32 << 0;
const MEDIUM_DISK:u32 = 1u32 << 0;
const MEDIUM_HASH:u32 = 2u32 << 0;

impl Default for Medium {
   fn default() -> Self {
      Medium {
         medium:  0,
      }
   }
}
impl Medium { 
   pub fn is_net(&self)  -> bool { (self.medium & MASK_IO) == MEDIUM_NET }
   pub fn is_disk(&self) -> bool { (self.medium & MASK_IO) == MEDIUM_DISK }
   pub fn is_hash(&self) -> bool { (self.medium & MASK_IO) == MEDIUM_HASH }

   pub fn clear(mut self)    -> Self { self.medium = 0; self }
   pub fn set_net(mut self)  -> Self { self.medium = self.medium & !MASK_IO | MEDIUM_NET; self }
   pub fn set_disk(mut self) -> Self { self.medium = self.medium & !MASK_IO | MEDIUM_DISK; self }
   pub fn set_hash(mut self) -> Self { self.medium = self.medium & !MASK_IO | MEDIUM_HASH; self }

   pub fn new(line:&str) -> Result<Self, crate::ParseError> {
      let m0 = Medium { medium:0 };
      let m = line.split(',').fold(Ok(m0), |m,s| {
         match (m,s) {
            (Err(e), _)     => Err(e),
            (Ok(m), "disk") => Ok(m.set_disk()),
            (Ok(m), "net")  => Ok(m.set_net()),
            (Ok(m), "hash") => Ok(m.set_hash()),
            (Ok(m), "")     => Ok(m),
            (Ok(_), _)      => Err(parse_error!(format!("unknown medium {:?}", s))),
         }
      });
      m
   }
}

impl std::convert::From<&str> for Medium {
   fn from(v:&str) -> Self {
      Medium::new(v).unwrap()
   }
}


#[test]
fn test_set() {
   use super::Medium;
   let m = Medium { medium: 0};
   assert_eq!(m.medium, 0x00);
   assert_eq!(0u32 & !1u32 | 0u32, 0u32);
   assert_eq!((0u32 & 1u32) & 0u32, 0u32);
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);

   let m = m.set_net();
   assert_eq!(m.medium, MEDIUM_NET);
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);

   let m = m.set_disk();
   assert_eq!(m.medium, MEDIUM_DISK);
   assert_eq!(m.is_net(),  false);
   assert_eq!(m.is_disk(), true);
}

#[test]
fn test_new() {
   use super::Medium;
   let m = Medium::new("net").unwrap();
   assert_eq!(m.is_net(),  true);
   assert_eq!(m.is_disk(), false);
}
