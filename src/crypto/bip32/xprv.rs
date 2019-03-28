use ::std::borrow::Borrow;
use ::crypto::{digest, hmac};
use ::crypto::secp256k1::{
   public_key, PublicKey, Sec1Encoder, Sec1Decoder, 
   secret_key, SecretKey, SecretKeyRawEncoder,
};
use ::ui::secp256k1::SecretKeyUi;
use ::utils::Base58check;

use super::XPub;

pub struct XPrv {
   pub secret_key: SecretKey,
   pub xpub: XPub,
}

impl XPrv {
   pub fn from_seed<T:Borrow<[u8]>>(seed: T) -> ::Result<Self> {
      let seed:&[u8] = seed.borrow();
      if seed.len() < 16 {
         raise_bip32_error!(format!("seed is too short: {} < 16", seed.len()));
      }
      if 64 < seed.len() {
         raise_bip32_error!(format!("seed is too long: 64 < {}", seed.len()));
      }
      let lr = {
         let mut lr = [0u8; 64];
         let mut hmac = ::ui::create_hmac_sha512(b"Bitcoin seed");
         hmac.input(seed);
         hmac.raw_result(&mut lr);
         lr
      };

      let (ret_secret_key, ret_public_key) = {
         let skui = ::ui::secp256k1::SecretKeyUi::s_decode_raw(&lr[0..32])?;
         let pk = skui.to_public_key().into_public_key();
         let sk = skui.into_secret_key();
         (sk, pk)
      };
      
      let ret_chain_code = {
         let mut tmp = [0u8; 32];
         tmp.copy_from_slice(&lr[32..]);
         tmp
      };
      Ok(Self {
         secret_key: ret_secret_key,
         xpub: XPub {
            public_key: ret_public_key,
            chain_code: ret_chain_code,
            depth: 0,
            parent_fingerprint: [0,0,0,0],
            index: 0,
         }
      })
   }
   pub fn derive(&self, i:u32) -> ::Result<Self> {
      if self.xpub.depth == ::std::u8::MAX {
         raise_bip32_error!(format!("too deep"));
      }

      let mut hmac = ::ui::create_hmac_sha512(&self.xpub.chain_code[..]);
      if i & 0x80000000 == 0 {
         let tmp = Sec1Encoder::s_encode(true, &self.xpub.public_key);
         hmac.input(&tmp[..]);
      } else {
         hmac.input(&[0u8]);
         let tmp = SecretKeyRawEncoder::s_encode(&self.secret_key);
         hmac.input(&tmp[..]);
      }
      {
         let ibe = i.to_be();
         let buf: &[u8;4] = unsafe { ::std::mem::transmute(&ibe) };
         hmac.input(buf);
      }
      let mut lr = [0u8; 64];
      hmac.raw_result(&mut lr);

      let (ret_secret_key, ret_public_key) = {
         let mut sk = SecretKeyUi::s_decode_raw(&lr[0..32])?;
         let _ = sk.add_raw(&self.secret_key);
         let pk = sk.to_public_key().into_public_key();
         let sk = sk.into_secret_key();
         (sk, pk)
      };
      let ret_chain_code = {
         let mut tmp = [0u8; 32];
         tmp.copy_from_slice(&lr[32..]);
         tmp
      };
      Ok(Self {
         secret_key: ret_secret_key,
         xpub: XPub {
            public_key: ret_public_key,
            chain_code: ret_chain_code,
            depth: self.xpub.depth+1,
            parent_fingerprint: self.xpub.fingerprint(),
            index: i,
         }
      })
   }
}


pub struct Encoder {
   b58c: Base58check,
}
impl Encoder {
   pub fn new(b58c: Base58check) -> Self {
      Self {
         b58c: b58c,
      }
   }
   pub fn encode(&self, xprv: &XPrv) -> String {
      let mut buf = super::xpub::Encoder::encode_common(&xprv.xpub);
      
      buf[41] = 0x00;
      let tmp = SecretKeyRawEncoder::new().encode(&xprv.secret_key);
      (&mut buf[42..42+32]).clone_from_slice(&tmp);
      
      self.b58c.encode(&buf)
   }
}

pub struct Decoder {
   b58c: Base58check,
}
impl Decoder {
   pub fn new(b58c: Base58check) -> Self {
      Self {
         b58c: b58c
      }
   }
   
   pub fn decode(&self, s: &str) -> ::Result<XPrv> {
      let (bytes, ret_depth, ret_index, ret_parent_fingerprint, ret_chain_code) =
         super::xpub::Decoder::decode_common(&self.b58c, s)?;

      if bytes[41] != 0x00 {
         raise_bip32_error!(format!("malformed xprv data"));
      }
      let (ret_secret_key, ret_public_key) = {
         let skui = ::ui::SecretKeyUi::s_decode_raw(&bytes[42..42+32])?;
         let pk = skui.to_public_key().into_public_key();
         let sk = skui.into_secret_key();
         (sk, pk)
      };

      Ok(XPrv {
         secret_key: ret_secret_key,
         xpub: XPub {
            public_key: ret_public_key,
            chain_code: ret_chain_code,
            depth: ret_depth,
            parent_fingerprint: ret_parent_fingerprint,
            index: ret_index,
         },
      })
   }
}
