use super::apriori::{sighash};
use super::flags::Flags;
use ::bitcoin::datatypes::{Tx, LockTime, TxIn};
use ::crypto::secp256k1;
use ::std::error::Error;

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
   
   let tmp = CustomTx::new(tx, txin_idx, &subscript, hash_type);
   let b = ::ui::bitcoin::serialize(&tmp, &())?;
   let b = ::ui::create_dhash256().u8_to_box(b.as_ref());
   Ok(b)
}

pub fn check_signature_encoding(vch:&[u8], flags:&Flags) -> ::Result<()> {
   if vch.len() == 0 {
      return Ok(());
   }
   
   if flags.script_verify.with(|f| f.is_der_sig() || f.is_low_s() || f.is_strict_enc()) {
      let dec = secp256k1::DerDecoder::new(true);
      let sig = dec.decode(vch).map_err(|e| {
         script_interpret_error!(SigDer, e.description())
      })?;
      if flags.script_verify.is_low_s() {
         if !sig.is_low_s() {
            raise_script_interpret_error!(SigHighS);
         };
      }
      if flags.script_verify.is_strict_enc() {
         is_defined_hashtype_signature(vch).map_err(|e| {
            script_interpret_error!(SigHashType, e.description())
         })?;
      }
   }
   Ok(())
}

pub fn check_pubkey_encoding(vch:&[u8], flags:&Flags) -> ::Result<()> {
   if flags.script_verify.is_strict_enc() {
      secp256k1::public_key::check_format(vch).map_err(|e| {
         script_interpret_error!(PubkeyType, e.description())
      })?;
   }
   if flags.script_verify.is_witness_pubkey_type() && flags.sig_version.is_witness_v0() {
      secp256k1::public_key::check_format(vch).map_err(|e| {
         script_interpret_error!(WitnessPubkeyType, e.description())
      })?;
   }
   Ok(())
}
pub fn parse_pubkey(vch:&[u8], flags:&Flags) -> ::Result<secp256k1::PublicKey> {
   let _ = check_pubkey_encoding(vch, flags)?;

   let pubkey = secp256k1::public_key::parse(vch).map_err(|e| {
      script_interpret_error!(SigDer, e.description())
   })?;
   Ok(pubkey)
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

pub fn chain_check_sign(
   tx:&Tx,
   txin_idx:usize,
   subscript:&[u8],
   pk_bytes:&[u8],
   sig_bytes:&[u8],
   flags:&Flags
) -> ::Result<bool>
{
   if pk_bytes.len() < 1 { return Ok(false); }
   if sig_bytes.len() < 1 { return Ok(false); }

   //println!("\nCheckSig");
   //println!("tx: {:?}", tx);
   //println!("txin_idx: {}", txin_idx);
   let hash = {
      let hash_type = sig_bytes[sig_bytes.len()-1];
      let hash = get_hash(tx, txin_idx, subscript, hash_type as i32)?;
      hash
   };

   let pubkey    = parse_pubkey(pk_bytes, flags)?;
   let signature = {
      let dec = secp256k1::DerDecoder::new(false);
      let mut sig = dec.decode(sig_bytes).map_err(|e| {
         use ::std::error::Error;
         script_interpret_error!(SigDer, e.description())
      })?;
      sig.normalize_low_s();
      sig
   };
   //println!("  hash: {}", ::ui::b2h(&hash[..]));
   //println!("  pub: {}", ::ui::b2h(pk_bytes));
   //println!("  sig: {}", ::ui::b2h(sig_bytes));
   let _ = secp256k1::verify(&pubkey, &hash[..], &signature)?; //失敗なら常にErrで返る
   Ok(true)
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

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
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

   fn serialize_tx_in(&self, e:&BitcoinSerializer, ws:&mut WriteStream, i:usize) -> ::Result<usize> {
      let mut r = 0usize;
      r += try!(self.tx.ins[i].prevout.serialize(&(), e, ws));

      if i == self.in_idx {
         r += try!(e.serialize_var_octets(ws, &self.subscript, ::std::usize::MAX));
      } else {
         r += try!(e.serialize_var_int(ws, 0)); // empty script
      }

      if (i == self.in_idx) || (!self.hash_single() && !self.hash_none()) {
         r += try!(e.serialize_u32le(ws, self.tx.ins[i].sequence));
      } else {
         r += try!(e.serialize_u32le(ws, 0));
      }

      Ok(r)
   }

   fn serialize_tx(&self, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;

      r += try!(e.serialize_i32le(ws, self.tx.version));

      { //txin
         if self.anyone_can_pay() {
            r += try!(e.serialize_var_int(ws, 1u64));
            r += try!(self.serialize_tx_in(e, ws, self.in_idx));
         } else {
            let len = self.tx.ins.len();
            r += try!(e.serialize_var_int(ws, len as u64));
            for i in 0..len {
               r += try!(self.serialize_tx_in(e, ws, i));
            }
         }
      }

      { //txout
         if self.hash_none() {
            r += try!(e.serialize_var_int(ws, 0u64));
         } else if self.hash_single() && self.in_idx < self.tx.outs.len() {
            let b = ::ui::bitcoin::serialize(&self.tx.outs[self.in_idx], &())?;
            let hash = ::ui::create_dhash256().u8_to_box(b.as_ref());
            r += try!(e.serialize_octets(ws, hash.as_ref()));
         } else {
            r += try!(e.serialize_var_array(&(), ws, self.tx.outs.as_slice(), ::std::usize::MAX));
         }

         /*
         let num_outs = match (self.hash_none(), self.hash_single()) {
            (true, _)     => 0usize,
            (false, true) => self.in_idx + 1,
            _             => self.tx.outs.len()
         };
         r += try!(e.serialize_varint(num_outs as u64));
         for i in 0..num_outs {
            r += try!(self.serialize_output(e, i));
         }
          */
      }
      
      r += try!(self.tx.locktime.serialize(&(), e, ws));
      Ok(r)
   }
}

impl <'a> BitcoinSerializee for CustomTx<'a> {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r = 0usize;
      r += try!(self.serialize_tx(e, ws));
      r += try!(e.serialize_i32le(ws, self.hash_type as i32));
      Ok(r)
   }
}

