use super::UInt256;

#[derive(Debug,Default,Clone)]
pub struct BlockLocator {
   pub haves: Vec<UInt256>,
}

impl ::std::fmt::Display for BlockLocator {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "BlockLocator(len={})", self.haves.len())
   }
}

use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for BlockLocator {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      if !e.medium().is_hash() {
         let v:i32 = e.medium().version();
         r += try!(e.encode_i32le(v));
      }
      r += try!(e.encode_var_array(&self.haves, ::std::usize::MAX));
      Ok(r)
   }
}
impl BitcoinDecodee for BlockLocator {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      if !d.medium().is_hash() {
         let mut v:i32 = 0;
         r += try!(d.decode_i32le(&mut v));
      }
      r += try!(d.decode_var_array(&mut self.haves, ::std::usize::MAX));
      Ok(r)
   }
}

