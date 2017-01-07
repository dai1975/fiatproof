extern crate crypto;
use self::crypto::digest::Digest;
use super::Hasher;

macro_rules! def {
   ($name:ident, $klass:path, $size:expr) => {
      pub struct $name {
         hasher: $klass
      }
      impl Default for $name {
         fn default() -> Self {
            $name {
               hasher: < $klass >::new(),
            }
         }
      }
      impl Hasher for $name {
         type Out = [u8; $size];
         
         fn reset(&mut self) { self.hasher.reset(); }
         fn input<T: ::std::convert::AsRef<[u8]>>(&mut self, data:T) {
            self.hasher.input(data.as_ref());
         }
         fn result(&mut self) -> Box<[u8]> {
            let mut out = box [0u8; $size];
            self.hasher.result(&mut *out);
            out
         }
      }
   };
}

def!(Sha256,    self::crypto::sha2::Sha256,         32);
def!(Ripemd160, self::crypto::ripemd160::Ripemd160, 20);

#[test]
fn test_sha256() {
   let input  = b"hello";
   let expect = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";

   assert_eq!(32, Sha256::size_of());
   assert_eq!(expect, Sha256::hexhash(input));
}

#[test]
fn test_ripemd160() {
   let input  = b"hello";
   let expect = "108f07b8382412612c048d07d13f814118445acd";

   assert_eq!(20, Ripemd160::size_of());
   assert_eq!(expect, Ripemd160::hexhash(input));
}

