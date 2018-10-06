use ::std::error::Error;
use super::{Message, Secp256k1};

pub struct Helper {
   ctx: Secp256k1<super::secp256k1::All>,
}

impl Helper {
   pub fn verify(&self, pk: &super::PublicKey, msg: &[u8], sig: &super::Signature) -> ::Result<()> {
      let message = Message::from_slice(msg).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      let _ = self.ctx.verify(&message, sig, pk).map_err(|e| {
         secp256k1_error!(e.description())
      })?;
      Ok(())
   }
}
