extern crate crypto;

// The inconvinience of lack of implementing Default trait by Digest structs in rust-crypto crate,
// I define wrapper structs and trait of them.
pub trait Digest: Default {
   const OUTPUT_BITS:  usize;
   const OUTPUT_BYTES: usize = (Self::OUTPUT_BITS + 7) / 8;
   const BLOCK_SIZE:   usize;
   
   fn input(&mut self, input: &[u8]);
   fn result(&mut self, out: &mut [u8]);
   fn reset(&mut self);
   fn output_bits(&self)  -> usize { Self::OUTPUT_BITS }
   fn output_bytes(&self) -> usize { Self::OUTPUT_BYTES }
   fn block_size(&self)   -> usize { Self::BLOCK_SIZE }
   
   fn input_str(&mut self, input: &str) {
      self.input(::handy::h2b(input).unwrap().as_slice())
   }
   fn result_str(&mut self) -> String {
      ::handy::b2h(&self.result_box())
   }

   // add some utility functions
   fn result_box(&mut self) -> Box<[u8]> {
      let len = self.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   fn digest_box(input: &[u8]) -> Box<[u8]> {
      let mut hasher = box Self::default();
      hasher.input(input);
      hasher.result_box()
   }
   fn digest_str(input: &[u8]) -> String {
      let mut hasher = box Self::default();
      hasher.input(input);
      hasher.result_str()
   }
}

use self::crypto::digest::Digest as _Digest;
macro_rules! def {
   ($n:ident, $t:path, $output_bits:expr, $block_size:expr) => {
      pub struct $n($t);
      impl $n {
         fn new() -> Self { $n(<$t>::new()) }
      }
      impl Default for $n {
         fn default() -> Self { <$n>::new() }
      }
      impl Digest for $n {
         const OUTPUT_BITS: usize = $output_bits;
         const BLOCK_SIZE:  usize = $block_size;
         fn input(&mut self, input: &[u8]) { self.0.input(input) }
         fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
         fn reset(&mut self) { self.0.reset() }
         fn input_str(&mut self, input: &str) { self.0.input_str(input) }
         fn result_str(&mut self) -> String { self.0.result_str() }
      }
   };
}
def!(Sha256,    self::crypto::sha2::Sha256,         256, 512/8);
def!(Sha1,      self::crypto::sha1::Sha1,           160, 512/8);
def!(Ripemd160, self::crypto::ripemd160::Ripemd160, 160, 512/8);

pub struct Double< T1, T2 > where T1:Digest, T2:Digest {
   d1: T1,
   d2: T2,
}
impl <T1,T2> Double<T1, T2> where T1:Digest, T2:Digest {
   fn new() -> Self {
      Self { d1:T1::default(), d2:T2::default() }
   }
}
impl <T1,T2> Default for Double<T1, T2> where T1:Digest, T2:Digest {
   fn default() -> Self { Self::new() }
}
impl <T1,T2> Digest for Double<T1, T2> where T1:Digest, T2:Digest {
   const OUTPUT_BITS: usize = T2::OUTPUT_BITS;
   const BLOCK_SIZE:  usize = T1::BLOCK_SIZE;

   fn reset(&mut self) {
      self.d1.reset();
      self.d2.reset();
   }
   fn input(&mut self, input: &[u8]) {
      self.d1.input(input);
   }
   fn result(&mut self, out: &mut [u8]) {
      self.d2.input(&self.d1.result_box());
      self.d2.result(out);
   }
}

pub type DHash256 = Double<Sha256, Sha256>;
pub type Hash160  = Double<Sha256, Ripemd160>;


#[test]
fn test_dhash256() {
   let input  = b"Hatsune Miku";
   let expect = "e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9";
      
   assert_eq!(32, DHash256::OUTPUT_BYTES);
   assert_eq!(expect, DHash256::digest_str(input));
}

#[test]
fn test_hash160() {
   let input  = b"Hatsune Miku";
   let expect = "b7233a798e6ea977644ded49241c2b153a6617b9";

   assert_eq!(20, Hash160::OUTPUT_BYTES);
   assert_eq!(expect, Hash160::digest_str(input));
}

