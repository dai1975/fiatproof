use num::Zero;
use num::bigint::BigUint;
use num::cast::{FromPrimitive, ToPrimitive};

def_error! { BaseNError }
macro_rules! raise_base_n_error {
   ($m:expr) => {
      Err(crate::utils::BaseNError::new($m, 0))?
   }
}

pub use std::collections::HashMap;
pub struct BaseN {
   base: usize,
   index2char: Vec<char>,
   char2index: HashMap<char, usize>,
}

impl BaseN {
   pub fn new(table: &str) -> Self {
      let base = table.len();
      if base < 2 {
         panic!("table must be greater than 2")
      }
      let mut ret = Self {
         base: base,
         index2char: Vec::with_capacity(base),
         char2index: HashMap::with_capacity(base),
      };
      for (i, c) in table.chars().enumerate() {
         ret.index2char.push(c);
         ret.char2index.insert(c, i);
      }
      ret
   }
   pub fn base_size(&self) -> usize { self.base }

   //const TABLE:&'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
   pub fn encode(&self, bytes: &[u8]) -> String {
      let mut val:BigUint = BigUint::from_bytes_be(bytes);
      let mut ret = Vec::<char>::new();
      let zero = BigUint::zero();
      while zero < val {
         let i = val.clone() % self.base;
         ret.push(self.index2char[i.to_usize().unwrap()]);
         val /= self.base;
      }
      for b in bytes {
         if *b == 0 {
            ret.push(self.index2char[0]);
         } else {
            break;
         }
      }
      use std::iter::FromIterator;
      String::from_iter(ret.iter().rev())
   }
   pub fn decode(&self, s:&str) -> crate::Result<Box<[u8]>> {
      let mut val = BigUint::zero();
      let mut mul = BigUint::from_u8(1).unwrap();
      for c in s.chars().rev() {
         let oi = self.char2index.get(&c);
         if oi.is_none() { raise_base_n_error!(format!("not a serialize char: {}", c)); }
         val += mul.clone() * oi.unwrap();
         mul *= self.base;
      }
      let mut ret = Vec::<u8>::new();
      for c in s.chars() {
         if c == self.index2char[0] {
            ret.push(0u8);
         } else {
            break;
         }
      }
      if BigUint::zero() < val {
         ret.extend(val.to_bytes_be());
      }
      Ok(ret.into_boxed_slice())
   }
}

mod tests {
   use crate::utils::BaseN;
   
   #[test]
   fn test_encode_b58() {
      //                     0         1         2         3         4         5
      //                     0123456789012345678901234567890123456789012345678901234567
      let base = BaseN::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "Rt5zm"; // 281563422 = 22*58^4 + 51*58^4 + 4*58^4 + 57*58^4 + 44*58^0
      
      let result = base.encode(&data);
      assert_eq!(enc, result);
   }

   #[test]
   fn test_decode_b58() {
      let base = BaseN::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
      let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
      let enc = "Rt5zm"; // 281563422 = 22*58^4 + 51*58^4 + 4*58^4 + 57*58^4 + 44*58^0
      let result = base.decode(enc);
      assert_matches!(result, Ok(_));
      assert_eq!(data, result.unwrap().as_ref());
   }

   #[test]
   fn test_encode_b16() {
      let base = BaseN::new("0123456789ABCDEF");
      let data = "Hatsune Miku".as_bytes().iter().cloned().collect::<Vec<u8>>();
      let enc  = "48617473756E65204D696B75";
      let result = base.encode(data.as_slice());
      assert_eq!(enc, result);
   }

   #[test]
   fn test_decode_b16() {
      let base = BaseN::new("0123456789ABCDEF");
      let data = "Hatsune Miku".as_bytes().iter().cloned().collect::<Vec<u8>>();
      let enc  = "48617473756E65204D696B75";
      let result = base.decode(enc);
      assert_matches!(result, Ok(_));
      assert_eq!(data.as_slice(), result.unwrap().as_ref());
   }
}

