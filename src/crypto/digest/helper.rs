use super::Digest;
use ::std::borrow::{Borrow, BorrowMut};

pub fn input_hex<D:Digest, T:Borrow<str>>(d: &mut D, input: T) {
   d.input(::utils::h2b(input).unwrap().as_ref())
}

pub fn input_hex_rev<D:Digest, T:Borrow<str>>(d: &mut D, input: T) {
   d.input(::utils::h2b_rev(input).unwrap().as_ref())
}

pub fn result_u8<D:Digest>(d: &mut D) -> Box<[u8]> {
   let len = d.output_bytes();
   let mut v = Vec::<u8>::with_capacity(len);
   unsafe { v.set_len(len); }
   d.result(v.as_mut_slice());
   v.into_boxed_slice()
}

pub fn result_hex<D:Digest>(d: &mut D) -> String {
   ::utils::b2h(result_u8(d))
}
pub fn result_hex_rev<D:Digest>(d: &mut D) -> String {
   ::utils::b2h_rev(result_u8(d))
}

pub fn u8_to_u8<D:Digest, T:Borrow<[u8]>>(d: &mut D, input: T) -> Box<[u8]> {
   d.reset();
   d.input(input.borrow());
   result_u8(d)
}
pub fn u8_to_hex<D:Digest, T:Borrow<[u8]>>(d: &mut D, input: T) -> String {
   d.reset();
   d.input(input.borrow());
   result_hex(d)
}
pub fn u8_to_hex_rev<D:Digest, T:Borrow<[u8]>>(d: &mut D, input: T) -> String {
   d.reset();
   d.input(input.borrow());
   result_hex_rev(d)
}
pub fn hex_to_u8<D:Digest, T:Borrow<str>>(d: &mut D, input: T) -> Box<[u8]> {
   d.reset();
   input_hex(d, input.borrow());
   result_u8(d)
}
pub fn hex_to_hex<D:Digest, T:Borrow<str>>(d: &mut D, input: T) -> String {
   d.reset();
   input_hex(d, input.borrow());
   result_hex(d)
}
pub fn hex_to_u8_rev<D:Digest, T:Borrow<str>>(d: &mut D, input: T) -> Box<[u8]> {
   d.reset();
   input_hex_rev(d, input.borrow());
   result_u8(d)
}
pub fn hex_to_hex_rev<D:Digest, T:Borrow<str>>(d: &mut D, input: T) -> String {
   d.reset();
   input_hex_rev(d, input.borrow());
   result_hex(d)
}

#[test]
fn test_sha512() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "3a9c593fc7d573a876aeec8303d4ef20cb62d055ee24f20334534b578b45dfd49924708385b9bbde280c2138f7f1dfd0ced554ad455a01b8ac8436043a2d6b5e";

   let mut d = ::crypto::digest::Sha512::new();
   assert_eq!(64, d.output_bytes());
   assert_eq!(expect, super::u8_to_hex(&mut d, input));
}
