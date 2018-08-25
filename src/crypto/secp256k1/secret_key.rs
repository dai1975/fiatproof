extern crate secp256k1;

pub struct SecretKey(secp256k1::key::SecretKey);

impl SecretKey {
   pub fn inner(&self) -> &secp256k1::key::SecretKey { &self.0 }
}
