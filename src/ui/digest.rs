use ::crypto::{
   Digest, DigestExt,
   Sha1, Sha256, Ripemd160, DHash256, Hash160
};

pub struct UiDigest<T:DigestExt>(T);

// define new class to avoiding from writing use ::crypto::{Digest, DigestExt} statement.
impl <T:DigestExt> UiDigest<T> {
   #[inline] pub fn output_bits(&self)  -> usize { self.0.output_bits() }
   #[inline] pub fn output_bytes(&self) -> usize { self.0.output_bytes() }
   #[inline] pub fn block_size(&self)   -> usize { self.0.block_size() }
   
   #[inline] pub fn input(&mut self, input: &[u8]) { self.0.input(input) }
   #[inline] pub fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
   #[inline] pub fn reset(&mut self) { self.0.reset() }
   
   #[inline] pub fn input_hex(&mut self, input: &str) { self.0.input_hex(input) }
   #[inline] pub fn result_box(&mut self) -> Box<[u8]> { self.0.result_box() }
   #[inline] pub fn result_hex(&mut self) -> String { self.0.result_hex() }
   #[inline] pub fn u8_to_box(&mut self, input: &[u8]) -> Box<[u8]> { self.0.u8_to_box(input) }
   #[inline] pub fn u8_to_hex(&mut self, input: &[u8]) -> String { self.0.u8_to_hex(input) }
   #[inline] pub fn hex_to_box(&mut self, input: &str) -> Box<[u8]> { self.0.hex_to_box(input) }
   #[inline] pub fn hex_to_hex(&mut self, input: &str) -> String { self.0.hex_to_hex(input) }
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
   pub static ref DIGEST: Factory = {
      Factory()
   };
}

