extern crate secp256k1;
use self::secp256k1::Secp256k1 as Context;
use self::secp256k1::key::SecretKey as SecretKey0;

extern crate rand;
fn random_32_bytes<R: rand::Rng>(rng: &mut R) -> [u8; 32] {
    let mut ret = [0u8; 32];
    rng.fill_bytes(&mut ret);
    ret
}

pub struct SecretKey(SecretKey0);

impl SecretKey {
   pub fn new() -> Self {
      let ctx = Context::new();
      let mut rng = rand::thread_rng();
      let mut data = random_32_bytes(&mut rng);
      let sk = loop {
         if let Ok(sk) = SecretKey0::from_slice(&ctx, &data) {
            break sk;
         }
         data = random_32_bytes(&mut rng);
      };
      SecretKey(sk)
   }

   pub fn inner(&self) -> &secp256k1::key::SecretKey { &self.0 }
   
}

pub struct Encoder {
}
impl Encoder {
   pub fn new() -> Self {
      Self { }
   }

   pub fn encode(&self, sk:&SecretKey) -> Box<[u8]> {
      let mut v = Vec::with_capacity(sk.0.len());
      v.clone_from_slice(&sk.0[..]);
      v.into_boxed_slice()
   }
}

pub struct Decoder {
}
impl Decoder {
   pub fn new() -> Self {
      Self { }
   }

   pub fn decode(&self, vch:&[u8]) -> ::Result<SecretKey> {
      let ctx = self::secp256k1::Secp256k1::new();
      let skey = secp256k1::key::SecretKey::from_slice(&ctx, vch)?;
      Ok(SecretKey(skey))
   }
}
