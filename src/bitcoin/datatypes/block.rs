use super::{BlockHeader, Tx};

#[derive(Debug,Default,Clone)]
pub struct Block {
   pub header: BlockHeader,
   pub txs: Vec<Tx>,
   pub checked: bool,
}

impl ::std::fmt::Display for Block {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "Block(header={}, tx={})", self.header, self.txs.len())
   }
}

use ::iostream::{ WriteStream, ReadStream };
use ::bitcoin::serialize::{
   Serializer as BitcoinSerializer,
   Serializee as BitcoinSerializee,
   Deserializer as BitcoinDeserializer,
   Deserializee as BitcoinDeserializee,
};
impl BitcoinSerializee for Block {
   type P = ();
   fn serialize(&self, _p:&Self::P, e:&BitcoinSerializer, ws:&mut WriteStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.serialize(&(), e, ws));
      r += try!(e.serialize_var_array(&(), ws, &self.txs, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDeserializee for Block {
   type P = ();
   fn deserialize(&mut self, _p:&Self::P, d:&BitcoinDeserializer, rs:&mut ReadStream) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.header.deserialize(&(), d, rs));
      r += try!(d.deserialize_var_array(&(), rs, &mut self.txs, ::std::usize::MAX));
      Ok(r)
   }
}
