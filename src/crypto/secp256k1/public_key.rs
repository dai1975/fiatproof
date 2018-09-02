extern crate secp256k1;
use super::Signature;
use ::std::error::Error;

pub struct PublicKey(secp256k1::key::PublicKey);

impl PublicKey {
   pub fn inner(&self) -> &secp256k1::key::PublicKey { &self.0 }

   pub fn verify(&self, message: &[u8], signature: &Signature) -> ::Result<()> {
      let message = secp256k1::Message::from_slice(message).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      let ctx = self::secp256k1::Secp256k1::new();
      let _ = ctx.verify(&message, signature.inner(), &self.0).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(())
   }
}

pub struct Sec1Encoder {
   compress:bool,
}
impl Sec1Encoder {
   pub fn new(compress: bool) -> Self {
      Self { compress:compress }
   }

   pub fn check(&self, pk:&PublicKey) -> Box<[u8]> {
      match self.compress {
         true  => Box::new(pk.inner().serialize()),
         false => Box::new(pk.inner().serialize_uncompressed()),
      }
   }
}

pub struct Sec1Decoder {
   compress: Option<bool>,
   hybrid:   bool,
}
impl Sec1Decoder {
   pub fn new(compress: Option<bool>, hybrid:bool) -> Self {
      Self { compress:compress, hybrid:hybrid }
   }

   pub fn check(&self, vch:&[u8]) -> ::Result<()> {
      let len = vch.len();
      if len == 0 {
         raise_secp256k1_error!("empty");
      }
      match vch[0] {
         2 | 3 if self.compress.unwrap_or(true) => {
            if len != 33 {
               raise_secp256k1_error!(format!("compressed but not 33 bytes length: {}", len));
            }
            Ok(())
         },
         4 if !self.compress.unwrap_or(false) => {
            if len != 65 {
               raise_secp256k1_error!(format!("compressed but not 65 bytes length: {}", len));
            }
            Ok(())
         },
         6 | 7 if !self.compress.unwrap_or(false) && self.hybrid => {
            if len != 65 {
               raise_secp256k1_error!(format!("compressed but not 65 bytes length: {}", len));
            }
            Ok(())
         },
         _ => {
            error_secp256k1_error!(format!("unexpected tag: {}", vch[0]))
         },
      }
   }
   
   pub fn decode(&self, vch:&[u8]) -> ::Result<PublicKey> {
      if self.compress.is_some() || !self.hybrid {
         self.check(vch)?;
      }
      let ctx = secp256k1::Secp256k1::new();
      let inner = secp256k1::key::PublicKey::from_slice(&ctx, vch).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(PublicKey(inner))
   }
}


