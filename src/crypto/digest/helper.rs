use super::Digest;
use ::std::borrow::{Borrow, BorrowMut};

pub trait DigestHelpable {
   type D:Digest;
   fn create_digest() -> Self::D;
}

pub struct DigestHelper<DH:DigestHelpable> {
   pub digest: DH::D,
}

impl <DH:DigestHelpable> DigestHelper<DH> {
   pub fn new(d: DH::D) -> Self {
      Self { digest:d }
   }

   pub fn reset(&mut self) {
      self.digest.reset()
   }
   pub fn input(&mut self, input: &[u8]) {
      self.digest.input(input)
   }
   pub fn result(&mut self, out: &mut [u8]) {
      self.digest.result(out)
   }
   
   pub fn input_hex<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b(input).unwrap().as_ref())
   }
   pub fn input_hex_rev<T:Borrow<str>>(&mut self, input: T) {
      self.input(::utils::h2b_rev(input).unwrap().as_ref())
   }
   pub fn result_u8(&mut self) -> Box<[u8]> {
      let len = self.digest.output_bytes();
      let mut v = Vec::<u8>::with_capacity(len);
      unsafe { v.set_len(len); }
      self.result(v.as_mut_slice());
      v.into_boxed_slice()
   }
   pub fn result_hex(&mut self) -> String {
      ::utils::b2h(self.result_u8())
   }
   pub fn result_hex_rev(&mut self) -> String {
      ::utils::b2h_rev(self.result_u8())
   }

   pub fn u8_to_u8<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input(input.borrow());
      self.result_u8()
   }
   pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex()
   }
   pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String {
      self.reset();
      self.input(input.borrow());
      self.result_hex_rev()
   }
   pub fn hex_to_u8<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex(input.borrow());
      self.result_hex()
   }
   pub fn hex_to_u8_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_u8()
   }
   pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String {
      self.reset();
      self.input_hex_rev(input.borrow());
      self.result_hex()
   }
   
   pub fn s_output_bits() -> usize { DH::create_digest().output_bits() }
   pub fn s_output_bytes()  -> usize { DH::create_digest().output_bytes() }
   pub fn s_block_size()    -> usize { DH::create_digest().block_size() }
   
   pub fn s_u8_to_u8<T:Borrow<[u8]>>(input: T) -> Box<[u8]> {
      Self::new(DH::create_digest()).u8_to_u8(input)
   }
   pub fn s_u8_to_hex<T:Borrow<[u8]>>(input: T) -> String {
      Self::new(DH::create_digest()).u8_to_hex(input)
   }
   pub fn s_u8_to_hex_rev<T:Borrow<[u8]>>(input: T) -> String {
      Self::new(DH::create_digest()).u8_to_hex_rev(input)
   }
   pub fn s_hex_to_u8<T:Borrow<str>>(input: T) -> Box<[u8]> {
      Self::new(DH::create_digest()).hex_to_u8(input)
   }
   pub fn s_hex_to_hex<T:Borrow<str>>(input: T) -> String {
      Self::new(DH::create_digest()).hex_to_hex(input)
   }
   pub fn s_hex_to_u8_rev<T:Borrow<str>>(input: T) -> Box<[u8]> {
      Self::new(DH::create_digest()).hex_to_u8_rev(input)
   }
   pub fn s_hex_to_hex_rev<T:Borrow<str>>(input: T) -> String {
      Self::new(DH::create_digest()).hex_to_hex_rev(input)
   }
}

macro_rules! def_helper {
   ($n:ident, $d:path) => {
      impl ::crypto::digest::helper::DigestHelpable for $d {
         type D = $d;
         fn create_digest() -> Self::D {
            <$d>::new()
         }
      }
      pub type $n = ::crypto::digest::helper::DigestHelper<$d>;
   };
}
def_helper!(Sha512Helper,    super::Sha512);
def_helper!(Sha256Helper,    super::Sha256);
def_helper!(Ripemd160Helper, super::Ripemd160);


#[test]
fn test_sha512() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "3a9c593fc7d573a876aeec8303d4ef20cb62d055ee24f20334534b578b45dfd49924708385b9bbde280c2138f7f1dfd0ced554ad455a01b8ac8436043a2d6b5e";

   assert_eq!(64, super::Sha512Helper::s_output_bytes());
   assert_eq!(expect, super::Sha512Helper::s_u8_to_hex(input));
}
