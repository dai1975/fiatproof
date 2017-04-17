use serde::de;
use super::{ReadStream};

pub struct Deserializer<R:ReadStream> {
   r: R,
   tmp_size: usize,
}

impl <R:ReadStream> Deserializer<R> {
   pub fn new(r:R) -> Self {
      Self { r:r }
   }
   pub fn into_inner(self) -> R {
      self.r
   }
   
   fn deserialize_varint(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut x:u8 = 0;
      try!(r.read_u8(&mut x));
      if x < 253 {
         *v = x as u64;
         Ok(1)
      } else if x == 253 {
         let mut y:u16 = 0;
         try!(r.read_u16le(&mut y));
         *v = y as u64;
         Ok(3)
      } else if x == 254 {
         let mut y:u32 = 0;
         try!(r.read_u32le(&mut y));
         *v = y as u64;
         Ok(5)
      } else {
         try!(r.read_u64le(v));
         Ok(9)
      }
   }      
}


