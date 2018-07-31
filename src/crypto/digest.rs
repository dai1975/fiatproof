extern crate crypto;
use self::crypto::digest::Digest as _Digest;
use ::std::borrow::{Borrow, BorrowMut};

// The inconvinience of lack of implementing Default trait by Digest structs in rust-crypto crate,
// I define wrapper structs and trait of them.
pub trait Digest {
   fn output_bits(&self)  -> usize;
   fn output_bytes(&self) -> usize;
   fn block_size(&self)   -> usize;
   
   fn input(&mut self, input:&[u8]); //todo: accepts Iterator<&u8> or IntoIterator<...>
   fn result(&mut self, out: &mut [u8]);
   fn reset(&mut self);
}

pub trait DigestExt: Digest + Default {
   const BLOCK_SIZE:   usize;
   const OUTPUT_BITS:  usize;
   const OUTPUT_BYTES: usize = (Self::OUTPUT_BITS + 7) / 8;

   fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b(input).unwrap().as_slice())
   }
   fn result_box(&mut self) -> Box<[u8]> {
      let len = self.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   fn result_hex(&mut self) -> String {
      ::utils::b2h(self.result_box())
   }

   fn u8_to_box<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input(input.borrow());
      self.result_box()
   }
   fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex()
   }
   fn hex_to_box<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex(input.borrow());
      self.result_box()
   }
   fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex(input.borrow());
      self.result_hex()
   }

   fn _u8_to_box<T:Borrow<[u8]>>(input: T) -> Box<[u8]> { Self::default().u8_to_box(input) }
   fn _u8_to_hex<T:Borrow<[u8]>>(input: T) -> String    { Self::default().u8_to_hex(input) }
   fn _hex_to_box<T:Borrow<str>>(input: T) -> Box<[u8]> { Self::default().hex_to_box(input) }
   fn _hex_to_hex<T:Borrow<str>>(input: T) -> String    { Self::default().hex_to_hex(input) }
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
      impl DigestExt for $n {
         const OUTPUT_BITS: usize = $output_bits;
         const BLOCK_SIZE:  usize = $block_size;
      }
   };
}
def!(Sha256,    self::crypto::sha2::Sha256,         256, 512/8);
def!(Sha1,      self::crypto::sha1::Sha1,           160, 512/8);
def!(Ripemd160, self::crypto::ripemd160::Ripemd160, 160, 512/8);

pub struct Double< T1, T2 >(T1,T2) where T1:DigestExt, T2:DigestExt;

impl <T1,T2> Double<T1, T2> where T1:DigestExt, T2:DigestExt {
   pub fn new() -> Self {
      Double::<T1,T2>(T1::default(), T2::default())
   }
}
impl <T1,T2> Digest for Double<T1, T2> where T1:DigestExt, T2:DigestExt {
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
impl <T1,T2> Default for Double<T1, T2> where T1:DigestExt, T2:DigestExt {
   fn default() -> Self { Self::new() }
}
impl <T1,T2> DigestExt for Double<T1, T2> where T1:DigestExt, T2:DigestExt {
   const OUTPUT_BITS: usize = T2::OUTPUT_BITS;
   const BLOCK_SIZE:  usize = T1::BLOCK_SIZE;
}

pub type DHash256 = Double<Sha256, Sha256>;
pub type Hash160  = Double<Sha256, Ripemd160>;


#[test]
fn test_dhash256() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9";
      
   assert_eq!(32, DHash256::OUTPUT_BYTES);
   assert_eq!(expect, DHash256::_u8_to_hex(input));
}

#[test]
fn test_hash160() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "b7233a798e6ea977644ded49241c2b153a6617b9";

   assert_eq!(20, Hash160::OUTPUT_BYTES);
   assert_eq!(expect, Hash160::_u8_to_hex(input));
}

