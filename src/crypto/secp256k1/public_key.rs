extern crate secp256k1;
use super::Signature;
use ::error::Error;

pub struct PublicKey(secp256k1::key::PublicKey);

impl PublicKey {
   pub fn inner(&self) -> &secp256k1::key::PublicKey { &self.0 }
}


pub fn check_format(vch:&[u8]) -> ::Result<()> {
   let len = vch.len();
   if len < 33 {
      raise_secp256k1_error!(format!("too short: {}", len));
   }
   match vch[0] {
      0x02 => {
         if len != 33 {
            raise_secp256k1_error!(format!("compressed but not 33 bytes length: {}", len));
         }
      },
      0x03 => {
         if len != 33 {
            raise_secp256k1_error!(format!("compressed but not 33 bytes length: {}", len));
         }
      },
      0x04 => {
         if len != 65 {
            raise_secp256k1_error!(format!("uncompressed but not 65 bytes length: {}", len));
         }
      },
      _ => {
         raise_secp256k1_error!(format!("unexpected format type: {}", vch[0]));
      }
   }
   Ok(())
}

pub fn parse(vch: &[u8]) -> ::Result< PublicKey > {
   let ctx = secp256k1::Secp256k1::new();
   let inner = secp256k1::key::PublicKey::from_slice(&ctx, vch).map_err(|e| {
      use ::std::error::Error;
      secp256k1_error!(e.description())
   })?;
   Ok(PublicKey(inner))
}

pub fn verify(message: &[u8], signature: &Signature, pubkey:&PublicKey) -> ::Result<bool> {
   let message = secp256k1::Message::from_slice(message).map_err(|e| {
      use ::std::error::Error;
      secp256k1_error!(e.description())
   })?;
   let ctx = self::secp256k1::Secp256k1::new();
   let r = ctx.verify(&message, signature.inner(), pubkey.inner());
   Ok(r.is_ok())
}

