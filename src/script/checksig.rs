use super::apriori::{sighash};
use super::flags;
use ::{Tx};

pub struct CheckSig<'a> {
   ctx:    ::secp256k1::Secp256k1,
   tx:     &'a Tx,
   in_idx: usize,
}

impl <'a> CheckSig<'a> {
   pub fn new<'x>(tx:&'x Tx, in_idx:usize) -> CheckSig<'x> {
      CheckSig {
         ctx: ::secp256k1::Secp256k1::new(),
         tx:  tx,
         in_idx: in_idx,
      }
   }
   pub fn verify(&self, subscript:&[u8], pk:&[u8], sig:&[u8], flags:flags::ScriptVerify) -> ::Result<()> {
      if sig.len() < 1 { script_error!("short sig"); }
      try!(PubKeyChecker::check(&self.ctx, pk, flags));
      try!(SignatureChecker::check(&self.ctx, sig, flags));

      let pubkey    = try!(::secp256k1::key::PublicKey::from_slice(&self.ctx, pk));
      let mut signature = try!(::secp256k1::Signature::from_der_lax(&self.ctx, sig));
      signature.normalize_s(&self.ctx);

      let message = {
         let hash_type = sig[sig.len()-1];
         let hash = try!(self.get_hash(subscript, hash_type as i32));
         try!(::secp256k1::Message::from_slice(&hash[..]))
      };

      let _ = try!(self.ctx.verify(&message, &signature, &pubkey));
      Ok(())
   }
   pub fn get_hash(&self, subscript:&[u8], hash_type:i32) -> ::Result<Box<[u8]>> {
      const ONE:[u8;32] = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
      if self.in_idx >= self.tx.ins.len() {
         return Ok(Box::new(ONE));
      }

      // Check for invalid use of SIGHASH_SINGLE
      if (hash_type & 0x1f) as u8 == sighash::SINGLE {
         if self.in_idx >= self.tx.outs.len() { //??
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
      let tmp = CustomTx::new(self.tx, self.in_idx, &subscript, hash_type);
      tmp.to_dhash256("hash")
   }
}

struct SignatureChecker;
impl SignatureChecker {
   pub fn check(ctx: &::secp256k1::Secp256k1, vch:&[u8], flags:flags::ScriptVerify) -> ::Result<bool> {
      if vch.len() == 0 { return Ok(true); }

      if flags.map(|f| f.is_der_sig() || f.is_low_s() || f.is_strict_enc()) {
         if !SignatureChecker::is_valid_encoding(vch) {
            script_error!("signature encoding");
         }
      }

      if flags.is_low_s() {
         if !SignatureChecker::is_low_der(ctx, vch) {
            script_error!("not a low der signature");
         }
      }

      if flags.is_strict_enc() {
         if !SignatureChecker::is_defined_hashtype(vch) {
            script_error!("not a defined sig hashtype");
         }
      }
      Ok(true)
   }

   fn is_valid_encoding(vch:&[u8]) -> bool {
      let len = vch.len();
      if len < 9 { return false; }
      if len >73 { return false; }

      if vch[0] != 0x30 { return false; }
      if (vch[1]+3) as usize != len { return false; }

      let len_r = vch[3] as usize;
      if 5 + len_r >= len { return false; }

      let len_s = vch[5 + len_r] as usize;
      if len_r + len_s + 7 != len { return false; }

      if vch[2] != 0x02 { return false; }
      if len_r == 0 { return false; }
      if (vch[4] & 0x80) != 0 { return false; }
      if (len_r > 1) && (vch[4] == 0x00) && ((vch[5] & 0x80) != 0) { return false; }
   
      if vch[len_r+4] != 0x02 { return false; }
      if len_s == 0 { return false; }
      if (vch[len_r+6] & 0x80) != 0 { return false; }
      if (len_s > 1) && (vch[len_r+6] == 0x00) && ((vch[len_r+7] & 0x80) != 0) { return false; }

      true
   }
   fn is_low_der(_ctx: &::secp256k1::Secp256k1, vch:&[u8]) -> bool {
      if !SignatureChecker::is_valid_encoding(vch) {
         return false;
      }
      // call ffi directly because rust-secp256k1 drops return value of normalize_s...
      unsafe {
         extern crate libc;
         use ::secp256k1::ffi;
         let mut ret:bool = false;
         let ctx = ffi::secp256k1_context_create(ffi::SECP256K1_START_VERIFY);
         let mut sig = ffi::Signature::blank();
         if ffi::ecdsa_signature_parse_der_lax(ctx, &mut sig, vch.as_ptr(), vch.len() as libc::size_t) == 1 {
            let r = ffi::secp256k1_ecdsa_signature_normalize(ctx, &mut sig as *mut ffi::Signature, &sig as *const ffi::Signature);
            ret = r == 0
         }
         ffi::secp256k1_context_destroy(ctx);
         ret
      }         
   }
   pub fn is_defined_hashtype(vch:&[u8]) -> bool {
      if vch.len() == 0 { return false; }

      let hash_type = vch[vch.len()-1] & !sighash::ANYONECANPAY;
      if hash_type < sighash::ALL || hash_type > sighash::SINGLE {
         return false;
      }
      true
   }
}

struct PubKeyChecker;
impl PubKeyChecker {
   pub fn check(_ctx: &::secp256k1::Secp256k1, vch:&[u8], flags:flags::ScriptVerify) -> ::Result<bool> {
      if flags.is_strict_enc() {
         if !PubKeyChecker::is_compressed_or_uncompressed(vch) {
            script_error!("pubkey encoding");
         }
      }
      Ok(true)
   }

   fn is_compressed_or_uncompressed(vch:&[u8]) -> bool {
      let len = vch.len();

      if len < 33 { return false; }
      match vch[0] {
         0x02 => len == 33,
         0x03 => len == 33,
         0x04 => len == 65,
         _ => false
      }
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
