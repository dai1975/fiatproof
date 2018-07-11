
pub struct Factory {
}

use ::crypto::{ Digest, DigestHelper, Sha1, Sha256, Ripemd160, DHash256, Hash160 };
impl Factory {
   pub fn new() -> Self {
      Self { }
   }
   pub fn create_sha1(&self) -> Sha1 { Sha1::new() }
   pub fn create_sha256(&self) -> Sha256 { Sha256::new() }
   pub fn create_ripemd160(&self) -> Ripemd160 { Ripemd160::new() }
   pub fn create_dhash256(&self) -> DHash256 { DHash256::new() }
   pub fn create_hash160(&self) -> Hash160 { Hash160::new() }

   pub fn create(&self, name:&str) -> Box<DigestHelper<Digest>> {
      match name {
         "sha1"      => Box::new(DigestHelper::<Sha1>::new()),
         //"sha256"    => Box::new(DigestHelper::<Sha1>::new()),
         //"ripemd160" => Box::new(DigestHelper::<Sha1>::new()),
         //"dhash256"  => Box::new(DigestHelper::<Sha1>::new()),
         //"hash160"   => Box::new(DigestHelper::<Sha1>::new()),
         _ => panic!(format!("unknown algorithm: {}", name)),
      }
   }
}

lazy_static! {
   pub static ref DIGEST: Factory = {
      Factory::new()
   };
}

