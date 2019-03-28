use ::crypto::{digest, hmac};
use ::crypto::secp256k1::{PublicKey, Sec1Encoder, Sec1Decoder, SecretKeyRawDecoder};
use ::utils::Base58check;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct XPub {
   pub public_key: PublicKey,
   pub chain_code: [u8;32],
   pub depth:      u8,
   pub parent_fingerprint: [u8; 4],
   pub index:      u32,
}

impl XPub {
   pub fn fingerprint(&self) -> [u8; 4] {
      let sec  = Sec1Encoder::s_encode(true, &self.public_key);
      let hash = ::ui::create_hash160().u8_to_u8(sec);
      [ hash[0], hash[1], hash[2], hash[3] ]
   }
   pub fn derive(&self, i:u32) -> ::Result<Self> {
      if 0x7FFFFFFF < i {
         raise_bip32_error!(format!("harden public child is not defined: i=0x{:x}", i));
      }
      if self.depth == ::std::u8::MAX {
         raise_bip32_error!(format!("too deep"));
      }

      let lr = {
         use self::hmac::Mac;
         let mut hmac = ::ui::create_hmac_sha512(&self.chain_code[..]);
         {
            let tmp = Sec1Encoder::s_encode(true, &self.public_key);
            hmac.input(&tmp[..]);
         }
         {
            let ibe = i.to_be();
            let buf: &[u8;4] = unsafe { ::std::mem::transmute(&ibe) };
            hmac.input(buf);
         }
         let mut lr = [0u8; 64];
         hmac.raw_result(&mut lr);
         lr
      };
      let ret_public_key = {
         let mut pk = ::ui::PublicKeyUi::new(self.public_key.clone());
         let     sk = ::ui::SecretKeyUi::s_decode_raw(&lr[0..32])?;
         let _  = pk.add_secret_key(&sk)?;
         //let sk = secret_key::RawDecoder::new().decode(&lr[0..32])?;
         //let _  = public_key::Helper::new().add_secret_key(&mut pk, &sk)?;
         pk.into_public_key()
      };
      let ret_chain_code = {
         let mut tmp = [0u8; 32];
         tmp.copy_from_slice(&lr[32..]);
         tmp
      };
      Ok(Self {
         public_key: ret_public_key,
         chain_code: ret_chain_code,
         depth: self.depth+1,
         parent_fingerprint: self.fingerprint(),
         index: i,
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
   pub fn encode(&self, xpub: &XPub) -> String {
      let mut buf = Self::encode_common(xpub);
      
      let tmp = Sec1Encoder::s_encode(true, &xpub.public_key);
      (&mut buf[41..41+33]).clone_from_slice(&tmp);
      
      self.b58c.encode(&buf)
   }
   pub fn encode_common(xpub:&XPub) -> [u8; 74] {
      let mut buf = [0u8; 1+4+4+32+33]; //74bytes
      buf[0] = xpub.depth;
      if 0 < buf[0] {
         (&mut buf[1..5]).clone_from_slice(&xpub.parent_fingerprint[..]);
         let ibe = xpub.index.to_be();
         let tmp: &[u8;4] = unsafe { ::std::mem::transmute(&ibe) };
         (&mut buf[5..9]).clone_from_slice(tmp);
      }
      (&mut buf[9..9+32]).clone_from_slice(&xpub.chain_code[..]);
      buf
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
   
   pub fn decode(&self, s: &str) -> ::Result<XPub> {
      let (bytes, ret_depth, ret_index, ret_parent_fingerprint, ret_chain_code) =
         Self::decode_common(&self.b58c, s)?;
      let ret_public_key = Sec1Decoder::new(Some(true), false).decode(&bytes[41..41+33])?;
      Ok(XPub {
         public_key: ret_public_key,
         depth: ret_depth,
         index: ret_index,
         parent_fingerprint: ret_parent_fingerprint,
         chain_code: ret_chain_code,
      })
   }
   
   pub fn decode_common(b58c: &Base58check, s: &str) -> ::Result<(Box<[u8]>, u8, u32, [u8;4], [u8;32])> {
      let bytes = b58c.decode(s)?; 
      if bytes.len() != 74 {
         raise_bip32_error!(format!("length mismatch: {}", bytes.len()));
      }

      let ret_depth = bytes[0];
      let ret_parent_fingerprint = {
         let mut tmp = [0u8; 4];
         tmp.clone_from_slice(&bytes[1..5]);
         tmp
      };
      let ret_index = {
         let mut tmp:u32 = 0;
         let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
         buf.clone_from_slice(&bytes[5..9]);
         u32::from_be(tmp)
      };
      let ret_chain_code = {
         let mut tmp = [0u8; 32];
         tmp.clone_from_slice(&bytes[9..9+32]);
         tmp
      };
      Ok((bytes, ret_depth, ret_index, ret_parent_fingerprint, ret_chain_code))
   }
}
