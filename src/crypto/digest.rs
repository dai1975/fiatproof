extern crate crypto;

// The inconvinience of lack of implementing Default trait by Digest structs in rust-crypto crate,
// I define wrapper structs and trait of them.
pub trait Digest: Default {
   //functions of self::crypto::digest::Digest;
   fn input(&mut self, input: &[u8]);
   fn result(&mut self, out: &mut [u8]);
   fn reset(&mut self);
   fn output_bits(&self) -> usize;
   fn block_size(&self) -> usize;
   fn output_bytes(&self) -> usize { (self.output_bits()+7) / 8 }
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
   ($n:ident, $t:path) => {
      pub struct $n($t);
      impl $n {
         fn new() -> Self { $n(<$t>::new()) }
      }
      impl Default for $n {
         fn default() -> Self { <$n>::new() }
      }
      impl Digest for $n {
         fn input(&mut self, input: &[u8]) { self.0.input(input) }
         fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
         fn reset(&mut self) { self.0.reset() }
         fn output_bits(&self) -> usize { self.0.output_bits() }
         fn block_size(&self) -> usize { self.0.block_size() }
         fn output_bytes(&self) -> usize { self.0.output_bytes() }
         fn input_str(&mut self, input: &str) { self.0.input_str(input) }
         fn result_str(&mut self) -> String { self.0.result_str() }
      }
   };
}
def!(Sha256,    self::crypto::sha2::Sha256);
def!(Sha1,      self::crypto::sha1::Sha1);
def!(Ripemd160, self::crypto::ripemd160::Ripemd160);


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
   fn output_bits(&self) -> usize {
      self.d2.output_bits()
   }
   fn block_size(&self) -> usize {
      self.d1.block_size()
   }

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

