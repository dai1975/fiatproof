use std;

//pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;

const REJECT_MALFORMED:u8        = 0x01;
const REJECT_INVALID:u8          = 0x10;
const REJECT_OBSOLETE:u8         = 0x11;
const REJECT_DUPLICATE:u8        = 0x12;
const REJECT_NON_STANDARD:u8     = 0x40;
const REJECT_DUST:u8             = 0x41;
const REJECT_INSUFFICIENT_FEE:u8 = 0x42;
const REJECT_CHECKPOINT:u8       = 0x43;

#[derive(Debug,Clone)]
pub struct RejectMessage {
   pub command : String, //not [u8;12] but var_str. check https://en.bitcoin.it/wiki/Protocol_documentation#reject
   pub code    : u8,
   pub reason  : String,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for RejectMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl RejectMessage {
   pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;
   pub fn is_malformed(&self)        -> bool { self.code == REJECT_MALFORMED }
   pub fn is_invalid(&self)          -> bool { self.code == REJECT_INVALID }
   pub fn is_obsolete(&self)         -> bool { self.code == REJECT_OBSOLETE }
   pub fn is_duplicate(&self)        -> bool { self.code == REJECT_DUPLICATE }
   pub fn is_non_standard(&self)     -> bool { self.code == REJECT_NON_STANDARD }
   pub fn is_dust(&self)             -> bool { self.code == REJECT_DUST }
   pub fn is_insufficient_fee(&self) -> bool { self.code == REJECT_INSUFFICIENT_FEE }
   pub fn is_checkpoint(&self)       -> bool { self.code == REJECT_CHECKPOINT }
}

impl RejectMessage {
   pub fn new<T:Message>(_: &T, code_:u8, reason_:&String) -> Self {
      RejectMessage {
         command: unsafe{::std::str::from_utf8_unchecked(&T::COMMAND[..]).to_string()},
         code:    code_,
         reason:  reason_.clone(),
      }
   }
}

impl std::fmt::Display for RejectMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Reject(cmd={}, code={}, reason={})",
             self.command, self.code, self.reason)
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for RejectMessage {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += e.serialize_var_string(ws, self.command.as_str(), ::std::usize::MAX)?;
      r += e.serialize_u8(ws, self.code)?;
      r += e.serialize_var_string(ws, self.reason.as_str(), RejectMessage::MAX_REJECT_MESSAGE_LENGTH)?;
      Ok(r)
   }
}
impl BitcoinDeserializee for RejectMessage {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += d.deserialize_var_string(rs, &mut self.command, ::std::usize::MAX)?;
      r += d.deserialize_u8(rs, &mut self.code)?;
      r += d.deserialize_var_string(rs, &mut self.reason, RejectMessage::MAX_REJECT_MESSAGE_LENGTH)?;
      // この後に拡張データがあるが、メッセージヘッダのサイズを見ないと分からない。
      Ok(r)
   }
}
