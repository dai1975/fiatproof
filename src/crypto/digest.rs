extern crate crypto;
use self::crypto::digest::Digest as _Digest;

// The inconvinience of lack of implementing Default trait by Digest structs in rust-crypto crate,
// I define wrapper structs and trait of them.
pub trait Digest {
   fn output_bits(&self)  -> usize;
   fn output_bytes(&self) -> usize;
   fn block_size(&self)   -> usize;
   
   fn input(&mut self, input: &[u8]); //todo: accepts Iterator<&u8> or IntoIterator<...>
   fn result(&mut self, out: &mut [u8]);
   fn reset(&mut self);

   fn input_hex(&mut self, input: &str) {
      self.input(::handy::h2b(input).unwrap().as_slice())
   }
   fn result_box(&mut self) -> Box<[u8]> {
      let len = self.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   fn result_hex(&mut self) -> String {
      ::handy::b2h(&self.result_box())
   }
}

pub trait UnsafeDigest: Digest + Default + Clone {
   const BLOCK_SIZE:   usize;
   const OUTPUT_BITS:  usize;
   const OUTPUT_BYTES: usize = (Self::OUTPUT_BITS + 7) / 8;
}   

pub struct DigestHelper<T:UnsafeDigest>(T);

impl <T:UnsafeDigest> Default for DigestHelper<T> {
   fn default() -> Self { DigestHelper::<T>(T::default()) }
}

impl <T:UnsafeDigest> DigestHelper<T> {
   #[inline] fn output_bits(&self)  -> usize { self.0.output_bits() }
   #[inline] fn output_bytes(&self) -> usize { self.0.output_bytes() }
   #[inline] fn block_size(&self)   -> usize { self.0.block_size() }
   #[inline] fn input(&mut self, input: &[u8]) { self.0.input(input) }
   #[inline] fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
   #[inline] fn reset(&mut self) { self.0.reset() }
   
   pub fn input_hex(&mut self, input: &str) { self.0.input_hex(input) }
   pub fn result_box(&mut self) -> Box<[u8]> { self.0.result_box() }
   pub fn result_hex(&mut self) -> String { self.0.result_hex() }

   pub fn u8_to_box(&mut self, input: &[u8]) -> Box<[u8]> {
      self.input(input);
      self.result_box()
   }
   pub fn u8_to_hex(&mut self, input: &[u8]) -> String {
      self.input(input);
      self.result_hex()
   }
   pub fn hex_to_box(&mut self, input: &str) -> Box<[u8]> {
      self.input_hex(input);
      self.result_box()
   }
   pub fn hex_to_hex(&mut self, input: &str) -> String {
      self.input_hex(input);
      self.result_hex()
   }

   pub fn _u8_to_box(input: &[u8]) -> Box<[u8]> { Self::default().u8_to_box(input) }
   pub fn _u8_to_hex(input: &[u8]) -> String    { Self::default().u8_to_hex(input) }
   pub fn _hex_to_box(input: &str) -> Box<[u8]> { Self::default().hex_to_box(input) }
   pub fn _hex_to_hex(input: &str) -> String    { Self::default().hex_to_hex(input) }
}

   

macro_rules! def {
   ($n:ident, $t:path, $output_bits:expr, $block_size:expr) => {
      pub struct $n($t);
      impl $n {
         pub fn new() -> Self { $n(<$t>::new()) }
      }
      impl Digest for $n {
         fn output_bits(&self)  -> usize { $output_bits }
         fn output_bytes(&self) -> usize { ($output_bits + 7) /8 }
         fn block_size(&self)   -> usize { $block_size }
         fn input(&mut self, input: &[u8]) { self.0.input(input) }
         fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
         fn reset(&mut self) { self.0.reset() }
      }
      impl Default for $n {
         fn default() -> Self { <$n>::new() }
      }
      impl Clone for $n {
         fn clone(&self) -> Self { $n(self.0.clone()) }
      }
      impl UnsafeDigest for $n {
         const OUTPUT_BITS: usize = $output_bits;
         const BLOCK_SIZE:  usize = $block_size;
      }
   };
}
def!(Sha256,    self::crypto::sha2::Sha256,         256, 512/8);
def!(Sha1,      self::crypto::sha1::Sha1,           160, 512/8);
def!(Ripemd160, self::crypto::ripemd160::Ripemd160, 160, 512/8);

pub struct Double< T1, T2 >(T1,T2) where T1:UnsafeDigest, T2:UnsafeDigest;

impl <T1,T2> Double<T1, T2> where T1:UnsafeDigest, T2:UnsafeDigest {
   pub fn new() -> Self {
      Double::<T1,T2>(T1::default(), T2::default())
   }
   pub fn new_with(o1:T1, o2:T2) -> Self {
      Double::<T1,T2>(o1,o2)
   }
}
impl <T1,T2> Digest for Double<T1, T2> where T1:UnsafeDigest, T2:UnsafeDigest {
   fn output_bits(&self)  -> usize { T2::OUTPUT_BITS }
   fn output_bytes(&self) -> usize { T2::OUTPUT_BYTES }
   fn block_size(&self)   -> usize { T1::BLOCK_SIZE }
   
   fn reset(&mut self) {
      self.0.reset();
      self.1.reset();
   }
   fn input(&mut self, input: &[u8]) {
      self.0.input(input);
   }
   fn result(&mut self, out: &mut [u8]) {
      self.1.input(self.0.result_box().as_ref());
      self.1.result(out);
   }
}
impl <T1,T2> Default for Double<T1, T2> where T1:UnsafeDigest, T2:UnsafeDigest {
   fn default() -> Self { Self::new() }
}
impl <T1,T2> Clone for Double<T1, T2> where T1:UnsafeDigest, T2:UnsafeDigest {
   fn clone(&self) -> Self { Self::new_with(self.0.clone(), self.1.clone()) }
}
impl <T1,T2> UnsafeDigest for Double<T1, T2> where T1:UnsafeDigest, T2:UnsafeDigest {
   const OUTPUT_BITS: usize = T2::OUTPUT_BITS;
   const BLOCK_SIZE:  usize = T1::BLOCK_SIZE;
}

pub type DHash256 = Double<Sha256, Sha256>;
pub type Hash160  = Double<Sha256, Ripemd160>;


#[test]
fn test_dhash256() {
   let input  = b"Hatsune Miku";
   let expect = "e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9";
      
   assert_eq!(32, DHash256::OUTPUT_BYTES);
   assert_eq!(expect, DigestHelper::<DHash256>::_u8_to_hex(input));
}

#[test]
fn test_hash160() {
   let input  = b"Hatsune Miku";
   let expect = "b7233a798e6ea977644ded49241c2b153a6617b9";

   assert_eq!(20, Hash160::OUTPUT_BYTES);
   assert_eq!(expect, DigestHelper::<Hash160>::_u8_to_hex(input));
}

