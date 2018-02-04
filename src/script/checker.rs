use super::apriori::{sighash};
use super::flags::Flags;
use ::{Tx, LockTime, TxIn};

pub fn get_hash(tx:&Tx, txin_idx:usize, subscript:&[u8], hash_type:i32) -> ::Result<Box<[u8]>> {
   const ONE:[u8;32] = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
   if txin_idx >= tx.ins.len() {
      return Ok(Box::new(ONE));
   }

   // Check for invalid use of SIGHASH_SINGLE
   if (hash_type & 0x1f) as u8 == sighash::SINGLE {
      if txin_idx >= tx.outs.len() { //??
         return Ok(Box::new(ONE));
      }
   }

   let subscript = { // from begin of script to last codeseparator except codeseparator itself
      use super::opcode::OP_CODESEPARATOR;
      let mut tmp:Vec<u8> = Vec::with_capacity(subscript.len());
      use super::parser::Parser;
      let mut beg = 0usize;
      for iter in Parser::iter(subscript) {
         let parsed = try!(iter);
         if parsed.opcode == OP_CODESEPARATOR {
            tmp.extend(&subscript[beg .. parsed.offset]);
            beg = parsed.offset + 1;
         }
      }
      if beg < subscript.len() {
         tmp.extend(&subscript[beg ..]);
      }
      tmp
   };
   
   use ::serialize::ToDigest;
   let tmp = CustomTx::new(tx, txin_idx, &subscript, hash_type);
   tmp.to_dhash256("hash")
}

pub fn check_signature_encoding(vch:&[u8], flags:&Flags) -> ::Result<()> {
   if vch.len() == 0 {
      return Ok(());
   }
   
   if flags.script_verify.map(|f| f.is_der_sig() || f.is_low_s() || f.is_strict_enc()) {
      is_valid_signature_encoding(vch).map_err(|e| {
         use ::std::error::Error;
         script_interpret_error!(SigDer, e.description())
      })?;
   }
   if flags.script_verify.is_low_s() {
      check_low_der(vch)?;
   }
   if flags.script_verify.is_strict_enc() {
      is_defined_hashtype_signature(vch).map_err(|e| {
         use ::std::error::Error;
         script_interpret_error!(SigHashType, e.description())
      })?;
   }
   Ok(())
}
pub fn parse_signature(vch:&[u8], flags:&Flags) -> ::Result<::secp256k1::Signature> {
   let _ = check_signature_encoding(vch, flags)?;

   let secp256k1 = ::secp256k1::Secp256k1::new();
   let sig = ::secp256k1::Signature::from_der_lax(
      &secp256k1, vch
   ).map(|mut sig| {
      sig.normalize_s(&secp256k1);
      sig
   }).map_err(|e| {
      use ::std::error::Error;
      script_interpret_error!(SigDer, e.description())
   })?;
   Ok(sig)
}

pub fn check_pubkey_encoding(vch:&[u8], flags:&Flags) -> ::Result<()> {
   if flags.script_verify.is_strict_enc() {
      if !is_compressed_or_uncompressed_pubkey(vch) {
         raise_script_interpret_error!(PubkeyType);
      }
   }
   if flags.script_verify.is_witness_pubkey_type() && flags.sig_version.is_witness_v0() {
      if !is_compressed_or_uncompressed_pubkey(vch) {
         raise_script_interpret_error!(WitnessPubkeyType);
      }
   }
   Ok(())
}
pub fn parse_pubkey(vch:&[u8], flags:&Flags) -> ::Result<::secp256k1::key::PublicKey> {
   let _ = check_pubkey_encoding(vch, flags)?;

   let secp256k1 = ::secp256k1::Secp256k1::new();
   let pubkey = ::secp256k1::key::PublicKey::from_slice(&secp256k1, vch).map_err(|e| {
      use ::std::error::Error;
      script_interpret_error!(SigDer, e.description())
   })?;
   Ok(pubkey)
}

pub fn is_valid_signature_encoding(vch:&[u8]) -> ::Result<()> {
   let len = vch.len();
   if len < 9 { raise_script_error!(format!("sigenc: too short: {}", len)); }
   if len >73 { raise_script_error!(format!("sigenc: too long: {}", len)); }
   
   if vch[0] != 0x30 { raise_script_error!(format!("sigenc: [0] != 0x30: {:x}", vch[0])); }
   if (vch[1]+3) as usize != len { raise_script_error!(format!("sigenc: [1]={}, len={}", vch[1], len)); }
   
   let len_r = vch[3] as usize;
   if 5 + len_r >= len { raise_script_error!(format!("sigenc: len_r={}, len={}", len_r, len)); }

   let len_s = vch[5 + len_r] as usize;
   if len_r + len_s + 7 != len { raise_script_error!(format!("sigenc: len_r={}, len_s={}, len={}", len_r, len_s, len)); }

   if vch[2] != 0x02 { raise_script_error!(format!("sigenc: [2] != 0x02: {:x}", vch[2])); }
   if len_r == 0 { raise_script_error!(format!("sigenc: len_r == 0: {}", len_r)); }
   if (vch[4] & 0x80) != 0 { raise_script_error!(format!("sigenc: [4]&0x80 != 0: {:x}", vch[4])); }
   if (len_r > 1) && (vch[4] == 0x00) && ((vch[5] & 0x80) != 0) { raise_script_error!(format!("sigenc: len_r={}, [4]={:x}, [5]={:x}", len_r, vch[4], vch[5])); }
   
   if vch[len_r+4] != 0x02 { raise_script_error!(format!("sigenc: [{}+4] != 0x02: {:x}", len_r, vch[len_r+4])); }
   if len_s == 0 { raise_script_error!(format!("sigenc: len_s == 0: {}", len_s)); }
   if (vch[len_r+6] & 0x80) != 0  { raise_script_error!(format!("sigenc: [{}+6]&0x80 != 0: {:x}", len_r, vch[len_r+6])); }
   if (len_s > 1) && (vch[len_r+6] == 0x00) && ((vch[len_r+7] & 0x80) != 0) { raise_script_error!(format!("sigenc: len_s={}, [{}+6]={:x}, [{}+7]={:x}", len_s, len_r, vch[len_r+6], len_r, vch[len_r+7])); }

   Ok(())
}

fn is_low_der(vch:&[u8]) -> ::Result<()> {
   let r = unsafe {
      // call ffi directly because rust-secp256k1 drops return value of normalize_s...
      extern crate libc;
      use ::secp256k1::ffi;
      let mut result = Ok(());
      let ctx = ffi::secp256k1_context_create(ffi::SECP256K1_START_VERIFY);
      let mut sig = ffi::Signature::blank();

      // bitcoin/src/pubkey.cpp/CheckLowS
      //   if (!ecdsa_signature_parse_der_lax(...) { return false; }
      //   return (!secp256k1_ecdsa_signature_normalize(...);
      let r = ffi::ecdsa_signature_parse_der_lax(ctx, &mut sig, vch.as_ptr(), vch.len() as libc::size_t);
      if r != 1 {
         result = Err(script_error!(format!("parse fail({})", r)));
      } else {
         let r = ffi::secp256k1_ecdsa_signature_normalize(ctx, &mut sig as *mut ffi::Signature, &sig as *const ffi::Signature);
         if r != 0 {
            result = Err(script_error!(format!("normalize fail({})", r)))
         }
      }
      ffi::secp256k1_context_destroy(ctx);
      result
   };
   r.map_err(|e| ::Error::from(e))
}

pub fn check_low_der(vch:&[u8]) -> ::Result<()> {
   is_valid_signature_encoding(vch).map_err(|e| {
      use ::std::error::Error;
      script_interpret_error!(SigDer, e.description())
   })?;
   is_low_der(vch).map_err(|e| {
      use ::std::error::Error;
      script_interpret_error!(SigHighS, e.description())
   })?;
   Ok(())
}

pub fn is_defined_hashtype_signature(vch:&[u8]) -> ::Result<()> {
   if vch.len() == 0 {
      raise_script_error!("empty");
   }
   
   let hash_type = vch[vch.len()-1] & !sighash::ANYONECANPAY;
   if hash_type < sighash::ALL || hash_type > sighash::SINGLE {
      raise_script_error!(format!("unknown hash type: {}", hash_type));
   }
   Ok(())
}

fn is_compressed_or_uncompressed_pubkey(vch:&[u8]) -> bool {
   let len = vch.len();

   if len < 33 { return false; }
   match vch[0] {
      0x02 => len == 33,
      0x03 => len == 33,
      0x04 => len == 65,
      _ => false
   }
}

pub fn chain_check_sign(
   tx:&Tx,
   txin_idx:usize,
   subscript:&[u8],
   pk:&[u8],
   sig:&[u8],
   flags:&Flags
) -> ::Result<bool>
{
   if pk.len() < 1 { return Ok(false); }
   if sig.len() < 1 { return Ok(false); }
   
   let message = {
      let hash_type = sig[sig.len()-1];
      let hash = get_hash(tx, txin_idx, subscript, hash_type as i32)?;
      ::secp256k1::Message::from_slice(&hash[..])?
   };

   let pubkey    = parse_pubkey(pk, flags)?;
   let signature = parse_signature(sig, flags)?;
   let secp256k1 = ::secp256k1::Secp256k1::new();
   let r = secp256k1.verify(&message, &signature, &pubkey);
   Ok(r.is_ok())
}

pub fn chain_check_locktime(
   tx: &Tx,
   txin_idx:usize,
   locktime: u64,
) -> ::Result<bool>
{
   let locktime = LockTime::new_by_u64(locktime);
   match (&tx.locktime, locktime) {
      (&LockTime::Block(l), LockTime::Block(r)) => {
         if l < r { return Ok(false); };
      },
      (&LockTime::Time(l), LockTime::Time(r)) => {
         if l < r { return Ok(false); };
      },
      (_, _) => { return Ok(false); }
   }
   if tx.ins[txin_idx].is_sequence_final() {
      return Ok(false);
   }
   Ok(true)
}

pub fn chain_check_sequence(
   tx: &Tx,
   txin_idx:usize,
   sequence: u32,
) -> ::Result<bool>
{
   if tx.version < 2 {
      //raise_script_error!(format!("BIP68 low version: {}", tx.version));
      return Ok(false);
   }
   if !tx.ins[txin_idx].is_locktime_enable() {
      //raise_script_error!(format!("txin[{}] is to be locktime disabled", txin_idx));
      return Ok(false);
   }

   match TxIn::compare_sequence_locktime(tx.ins[txin_idx].sequence, sequence) {
      None => Ok(false),
      Some(b) => Ok(b),
   }
}

use ::serialize::bitcoin::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
};
struct CustomTx<'a> {
   tx: &'a Tx,
   in_idx: usize,
   subscript: &'a [u8],
   hash_type: u8,
}
impl <'a> CustomTx<'a> {
   pub fn new<'x>(tx:&'x Tx, in_idx:usize, subscript:&'x [u8], hash_type:i32) -> CustomTx<'x> {
      CustomTx { tx:tx, in_idx:in_idx, subscript:subscript, hash_type:hash_type as u8 }
   }
   pub fn anyone_can_pay(&self) -> bool { (self.hash_type & sighash::ANYONECANPAY) != 0 }
   pub fn hash_single(&self) -> bool    { (self.hash_type & 0x1f) == sighash::SINGLE }
   pub fn hash_none(&self) -> bool      { (self.hash_type & 0x1f) == sighash::NONE }

   fn encode_tx_in(&self, e:&mut BitcoinEncoder, i:usize) -> ::Result<usize> {
      let mut r = 0usize;
      r += try!(self.tx.ins[i].prevout.encode(e));

      if i == self.in_idx {
         r += try!(e.encode_var_octets(&self.subscript, ::std::usize::MAX));
      } else {
         r += try!(e.encode_var_int(0)); // empty script
      }

      if (i == self.in_idx) || (!self.hash_single() && !self.hash_none()) {
         r += try!(e.encode_u32le(self.tx.ins[i].sequence));
      } else {
         r += try!(e.encode_u32le(0));
      }

      Ok(r)
   }

   fn encode_tx(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;

      r += try!(e.encode_i32le(self.tx.version));

      { //txin
         if self.anyone_can_pay() {
            r += try!(e.encode_var_int(1u64));
            r += try!(self.encode_tx_in(e, self.in_idx));
         } else {
            let len = self.tx.ins.len();
            r += try!(e.encode_var_int(len as u64));
            for i in 0..len {
               r += try!(self.encode_tx_in(e, i));
            }
         }
      }

      { //txout
         if self.hash_none() {
            r += try!(e.encode_var_int(0u64));
         } else if self.hash_single() && self.in_idx < self.tx.outs.len() {
            use ::serialize::ToDigest;
            let hash = try!(self.tx.outs[self.in_idx].to_dhash256("hash"));
            r += try!(e.encode_octets(hash.as_ref()));
         } else {
            r += try!(e.encode_var_array(self.tx.outs.as_slice(), ::std::usize::MAX));
         }

         /*
         let num_outs = match (self.hash_none(), self.hash_single()) {
            (true, _)     => 0usize,
            (false, true) => self.in_idx + 1,
            _             => self.tx.outs.len()
         };
         r += try!(e.encode_varint(num_outs as u64));
         for i in 0..num_outs {
            r += try!(self.encode_output(e, i));
         }
          */
      }
      
      r += try!(self.tx.locktime.encode(e));
      Ok(r)
   }
}

impl <'a> BitcoinEncodee for CustomTx<'a> {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r = 0usize;
      r += try!(self.encode_tx(e));
      r += try!(e.encode_i32le(self.hash_type as i32));
      Ok(r)
   }
}

