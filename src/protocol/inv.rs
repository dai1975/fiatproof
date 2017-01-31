use std;
use ::UInt256;

#[derive(Debug,Clone,PartialEq)]
pub enum InvType {
   Unknown       = 0,
   Tx            = 1,
   Block         = 2,
   FilteredBlock = 3,
}

impl Default for InvType {
   fn default() -> Self { InvType::Unknown }
}

impl std::fmt::Display for InvType {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match *self {
         InvType::Tx            => write!(f, "tx"),
         InvType::Block         => write!(f, "block"),
         InvType::FilteredBlock => write!(f, "filtered"),
         _ => write!(f, "{}", *self),
      }
   }
}
impl InvType {
   pub fn is_tx(&self)             -> bool { *self == InvType::Tx }
   pub fn is_block(&self)          -> bool { *self == InvType::Block }
   pub fn is_filtered_block(&self) -> bool { *self == InvType::FilteredBlock }
}


#[derive(Debug,Clone,Default)]
pub struct Inv {
   pub invtype: InvType,
   pub hash:    UInt256,
}
impl std::fmt::Display for Inv {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}:{}", self.invtype, self.hash)
   }
}
impl Inv {
   #[allow(dead_code)]
   pub fn new(invtype_:InvType, hash_: UInt256) -> Self {
      Inv {
         invtype: invtype_,
         hash:    hash_,
      }
   }
   pub fn new_tx(hash: UInt256)             -> Self { Self::new(InvType::Tx, hash) }
   pub fn new_block(hash: UInt256)          -> Self { Self::new(InvType::Block, hash) }
   pub fn new_filtered_block(hash: UInt256) -> Self { Self::new(InvType::FilteredBlock, hash) }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for InvType {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let tmp:u32 = match *self {
         InvType::Tx => 1,
         InvType::Block => 2,
         InvType::FilteredBlock => 3,
         _ => encode_error!("malformed inv type"),
      };
      e.encode_u32le(tmp)
   }
}
impl Decodee for InvType {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let mut tmp:u32 = 0;
      r += try!(d.decode_u32le(&mut tmp));
      *self = match tmp {
         1 => InvType::Tx,
         2 => InvType::Block,
         3 => InvType::FilteredBlock,
         _ => encode_error!("unexpected inv value"),
      };
      Ok(r)
   }
}

impl Encodee for Inv {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invtype.encode(e, ()));
      r += try!(self.hash.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for Inv {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.invtype.decode(d, ()));
      r += try!(self.hash.decode(d, ()));
      Ok(r)
   }
}
