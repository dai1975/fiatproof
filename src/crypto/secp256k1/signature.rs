use secp256k1::{Secp256k1, Signature, All};
use num::bigint::BigUint;

lazy_static! {
   static ref SECP256K1_N:BigUint = BigUint::from_bytes_be(&[
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
      0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
      0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
   ]);
   static ref SECP256K1_N_H:BigUint = BigUint::from_bytes_be(&[
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D,
      0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B, 0x20, 0xA0
   ]);
}

pub fn get_raw(sig: &Signature) -> (BigUint, BigUint) {
   let bytes = sig.serialize_compact();
   let r = BigUint::from_bytes_be(&bytes[0..32]);
   let s = BigUint::from_bytes_be(&bytes[32..64]);
   (r,s)
}

pub fn is_low_s(sig: &Signature) -> bool {
   let bytes = sig.serialize_compact();
   if (bytes[32] & 0x80) != 0 {
      false
   } else {
      let s = BigUint::from_bytes_be(&bytes[32..64]);
      s < *SECP256K1_N_H
   }
}
pub fn normalize_s(sig: &mut Signature) -> bool {
   if is_low_s(sig) {
      false
   } else {
      sig.normalize_s();
      true
   }
}

pub struct DerEncoder {
}

impl DerEncoder {
   pub fn new() -> Self {
      Self { }
   }
   
   pub fn s_encode(sig:&Signature) -> Box<[u8]> {
      let sersig = sig.serialize_der();
      Vec::from(&sersig[..]).into_boxed_slice()
   }
   pub fn encode(&self, sig:&Signature) -> Box<[u8]> {
      Self::s_encode(sig)
   }
}

pub struct DerDecoder {
   is_strict: bool,
}
impl DerDecoder {
   pub fn new(is_strict: bool) -> Self {
      Self { is_strict:is_strict }
   }
   
   pub fn decode(&self, vch: &[u8]) -> crate::Result<Signature> {
      Self::s_decode(self.is_strict, vch)
   }
   
   pub fn decode_lax(&self, vch: &[u8]) -> crate::Result<Signature> {
      Self::s_decode_lax(vch)
   }
   
   pub fn s_decode(is_strict:bool, vch: &[u8]) -> crate::Result<Signature> {
      if is_strict {
         Self::s_check_strict(vch)?;
         // because of the check_strict is not a secp256k1 function, it is not returns secp256 data.
      }
      Self::s_decode_lax(vch)
   }
   pub fn s_check_strict(vch: &[u8]) -> crate::Result<()> {
      let len = vch.len();
      if len < 9 { raise_secp256k1_error!(format!("der: too short: {}", len)); }
      if len >73 { raise_secp256k1_error!(format!("der: too long: {}", len)); }
      
      if vch[0] != 0x30 { raise_secp256k1_error!(format!("der: [0] != 0x30: {:x}", vch[0])); }
      if (vch[1]+3) as usize != len { raise_secp256k1_error!(format!("der: [1]={}, len={}", vch[1], len)); }
   
      let len_r = vch[3] as usize;
      if 5 + len_r >= len { raise_secp256k1_error!(format!("der: len_r={}, len={}", len_r, len)); }

      let len_s = vch[5 + len_r] as usize;
      if len_r + len_s + 7 != len { raise_secp256k1_error!(format!("der: len_r={}, len_s={}, len={}", len_r, len_s, len)); }

      if vch[2] != 0x02 { raise_secp256k1_error!(format!("der: [2] != 0x02: {:x}", vch[2])); }
      if len_r == 0 { raise_secp256k1_error!(format!("der: len_r == 0: {}", len_r)); }
      if (vch[4] & 0x80) != 0 { raise_secp256k1_error!(format!("der: [4]&0x80 != 0: {:x}", vch[4])); }
      // R is minimal format so that heading zero is not allowed
      if (len_r > 1) && (vch[4] == 0x00) && ((vch[5] & 0x80) == 0) { raise_secp256k1_error!(format!("der: len_r={}, [4]={:x}, [5]={:x}", len_r, vch[4], vch[5])); }
      
      if vch[len_r+4] != 0x02 { raise_secp256k1_error!(format!("der: [{}+4] != 0x02: {:x}", len_r, vch[len_r+4])); }
      if len_s == 0 { raise_secp256k1_error!(format!("der: len_s == 0: {}", len_s)); }
      if (vch[len_r+6] & 0x80) != 0  { raise_secp256k1_error!(format!("der: [{}+6]&0x80 != 0: {:x}", len_r, vch[len_r+6])); }
      // S is minimal format so that heading zero is not allowed   
      if (len_s > 1) && (vch[len_r+6] == 0x00) && ((vch[len_r+7] & 0x80) == 0) { raise_secp256k1_error!(format!("der: len_s={}, [{}+6]={:x}, [{}+7]={:x}", len_s, len_r, vch[len_r+6], len_r, vch[len_r+7])); }
      Ok(())
   }
   pub fn s_decode_lax(vch: &[u8]) -> crate::Result<Signature> {
      let sig = Signature::from_der_lax(vch).map_err(|e| {
         use std::error::Error;
         secp256k1_error!(e.description())
      })?;
      Ok(sig)
   }
}
/*
pub fn check_format_low_der(vch:&[u8]) -> crate::Result<()> {
   let r = unsafe {
      // call ffi directly because rust-secp256k1 drops return value of normalize_s...
      extern crate libc;
      use self::secp256k1::ffi;
      let mut result = Ok(());
      let ctx = ffi::secp256k1_context_create(ffi::SECP256K1_START_VERIFY);
      let mut sig = ffi::Signature::blank();

      // bitcoin/src/pubkey.cpp/CheckLowS
      //   if (!ecdsa_signature_parse_der_lax(...) { return false; }
      //   return (!secp256k1_ecdsa_signature_normalize(...);
      let r = ffi::ecdsa_signature_parse_der_lax(ctx, &mut sig, vch.as_ptr(), vch.len() as libc::size_t);
      if r != 1 {
         result = Err(secp256k1_error!(format!("parse fail({})", r)));
      } else {
         let r = ffi::secp256k1_ecdsa_signature_normalize(ctx, &mut sig as *mut ffi::Signature, &sig as *const ffi::Signature);
         if r != 0 {
            result = Err(secp256k1_error!(format!("normalize fail({})", r)))
         }
      }
      ffi::secp256k1_context_destroy(ctx);
      result
   };
   let _ = r?;
   Ok(())
}
*/

