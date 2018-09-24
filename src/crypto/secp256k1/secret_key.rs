extern crate secp256k1;
use self::secp256k1::Secp256k1 as Context;
use self::secp256k1::key::SecretKey as SecretKey0;
use super::PublicKey;

extern crate rand;
fn random_32_bytes<R: rand::Rng>(rng: &mut R) -> [u8; 32] {
    let mut ret = [0u8; 32];
    rng.fill_bytes(&mut ret);
    ret
}

#[derive(Debug,Clone)]
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

   pub fn to_public_key(&self) -> PublicKey {
      let ctx = Context::new();
      let inner = secp256k1::key::PublicKey::from_secret_key(&ctx, &self.0);
      PublicKey(inner)
   }
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

pub struct Decoder { }
impl Decoder {
   pub fn new() -> Self {
      Self { }
   }

   /**
    * fail in case of
    *  - vch.len() != 32
    *  - n <= vch
    *  - vch == 0
    * see SecretKey::from_slice and secp256k1_ec_seckey_verify
    */
   pub fn decode(&self, vch:&[u8]) -> ::Result<SecretKey> {
      let ctx = self::secp256k1::Secp256k1::new();
      let skey = secp256k1::key::SecretKey::from_slice(&ctx, vch)?;
      Ok(SecretKey(skey))
   }
}


pub struct Base58checkEncoder<'a> {
   is_compressed: bool,
   b58c: &'a ::utils::Base58check,
}
impl <'a> Base58checkEncoder<'a> {
   pub fn new(b58c: &'a ::utils::Base58check, is_compressed:bool) -> Self {
      Self {
         is_compressed:is_compressed,
         b58c: b58c,
      }
   }

   pub fn encode(&self, sk:&SecretKey) -> String {
      let bytes = Encoder::new().encode(sk);
      if self.is_compressed {
         let mut v = Vec::from(bytes.as_ref());
         v.push(1);
         self.b58c.encode(v.as_slice())
      } else {
         self.b58c.encode(bytes.as_ref())
      }
   }
}

pub struct Base58checkDecoder<'a> {
   b58c: &'a ::utils::Base58check,
}
impl <'a> Base58checkDecoder<'a> {
   pub fn new(b58c: &'a ::utils::Base58check) -> Self {
      Self {
         b58c: b58c,
      }
   }

   pub fn decode_base58check(&self, s: &str) -> ::Result<(Box<[u8]>, bool)> {
      //check base58check and version bytes is match
      let bytes = self.b58c.decode(s)?; 
      //check 32bytes or 33bytes compression format
      let is_compressed = if bytes.len() == 32 {
         Ok(false)
      } else if bytes.len() == 33 && bytes[32] == 1 {
         Ok(true)
      } else {
         error_secp256k1_error!("malformed secret key base58check")
      }?;
      Ok((bytes, is_compressed))
   }
   
   pub fn decode(&self, s: &str) -> ::Result<SecretKey> {
      let (bytes, _is_compressed) = self.decode_base58check(s)?;
      let dec = Decoder::new();
      dec.decode(&bytes[0..32])
   }
}

