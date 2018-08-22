use ::crypto::{
   Digest, DigestExt,
   Sha1, Sha256, Ripemd160, DHash256, Hash160
};
use ::std::borrow::{Borrow, BorrowMut};

pub struct UiDigest<X:DigestExt>(X);

// define new class to avoiding from writing "use ::crypto::{Digest, DigestExt}" statement.
impl <X:DigestExt> UiDigest<X> {
   #[inline] pub fn output_bits(&self)  -> usize { self.0.output_bits() }
   #[inline] pub fn output_bytes(&self) -> usize { self.0.output_bytes() }
   #[inline] pub fn block_size(&self)   -> usize { self.0.block_size() }
   
   #[inline] pub fn input<T:Borrow<[u8]>>(&mut self, input: T) { self.0.input(input.borrow()) }
   #[inline] pub fn result<T:BorrowMut<[u8]>>(&mut self, mut out: T) { self.0.result(out.borrow_mut()) }
   #[inline] pub fn reset(&mut self) { self.0.reset() }
   
   #[inline] pub fn input_hex(&mut self, input: &str) { self.0.input_hex(input) }
   #[inline] pub fn input_hex_rev(&mut self, input: &str) { self.0.input_hex_rev(input) }
   #[inline] pub fn result_box(&mut self) -> Box<[u8]> { self.0.result_box() }
   #[inline] pub fn result_hex(&mut self) -> String { self.0.result_hex() }
   #[inline] pub fn result_hex_rev(&mut self) -> String { self.0.result_hex_rev() }
   
   #[inline] pub fn u8_to_box<T:Borrow<[u8]>>(&mut self, input: T) -> Box<[u8]> { self.0.u8_to_box(input) }
   #[inline] pub fn u8_to_hex<T:Borrow<[u8]>>(&mut self, input: T) -> String { self.0.u8_to_hex(input) }
   #[inline] pub fn u8_to_hex_rev<T:Borrow<[u8]>>(&mut self, input: T) -> String { self.0.u8_to_hex_rev(input) }
   #[inline] pub fn hex_to_box<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> { self.0.hex_to_box(input) }
   #[inline] pub fn hex_to_hex<T:Borrow<str>>(&mut self, input: T) -> String { self.0.hex_to_hex(input) }
   #[inline] pub fn hex_to_box_rev<T:Borrow<str>>(&mut self, input: T) -> Box<[u8]> { self.0.hex_to_box_rev(input) }
   #[inline] pub fn hex_to_hex_rev<T:Borrow<str>>(&mut self, input: T) -> String { self.0.hex_to_hex_rev(input) }
}

macro_rules! deffn {
   ($fname:ident, $t:ident) => {
      pub fn $fname(&self) -> UiDigest<$t> { UiDigest::<$t>($t::new()) }
   }
}

pub struct Factory();
impl Factory {
   deffn! { create_sha1,      Sha1 }
   deffn! { create_sha256,    Sha256 }
   deffn! { create_ripemd160, Ripemd160 }
   deffn! { create_dhash256,  DHash256 }
   deffn! { create_hash160,   Hash160 }

   pub fn create(&self, name:&str) -> Box<Digest> {
      match name {
         "sha1"      => Box::new(Sha1::new()),
         "sha256"    => Box::new(Sha256::new()),
         "ripemd160" => Box::new(Ripemd160::new()),
         "dhash256"  => Box::new(DHash256::new()),
         "hash160"   => Box::new(Hash160::new()),
         _ => panic!(format!("unknown algorithm: {}", name)),
      }
   }
}

lazy_static! {
   pub static ref DIGEST: Factory = Factory();
}

