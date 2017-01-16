use ::std::borrow::Borrow;
use ::{Error, UInt256};
use super::CodecParam;

pub trait Decodee<D, P> where D:Decoder {
   fn decode<BP>(&mut self, p:BP, d:&mut D) -> Result<usize, Error>
      where BP:Borrow<P>+Sized;
}

pub trait Decoder: Sized {
   fn param(&self) -> &CodecParam;

   fn decode_skip(&mut self, n:usize) -> Result<usize, Error>;
   
   fn decode_u8(&mut self, v:&mut u8) -> Result<usize, Error>;
   fn decode_u16le(&mut self, v:&mut u16) -> Result<usize, Error>;
   fn decode_u32le(&mut self, v:&mut u32) -> Result<usize, Error>;
   fn decode_u64le(&mut self, v:&mut u64) -> Result<usize, Error>;
   fn decode_u16be(&mut self, v:&mut u16) -> Result<usize, Error>;
   fn decode_u32be(&mut self, v:&mut u32) -> Result<usize, Error>;
   fn decode_u64be(&mut self, v:&mut u64) -> Result<usize, Error>;

   fn decode_i8(&mut self, v:&mut i8) -> Result<usize, Error>;
   fn decode_i16le(&mut self, v:&mut i16) -> Result<usize, Error>;
   fn decode_i32le(&mut self, v:&mut i32) -> Result<usize, Error>;
   fn decode_i64le(&mut self, v:&mut i64) -> Result<usize, Error>;
   fn decode_i16be(&mut self, v:&mut i16) -> Result<usize, Error>;
   fn decode_i32be(&mut self, v:&mut i32) -> Result<usize, Error>;
   fn decode_i64be(&mut self, v:&mut i64) -> Result<usize, Error>;
   
   fn decode_bool(&mut self, v:&mut bool) -> Result<usize, Error>;
   fn decode_varint(&mut self, v:&mut u64) -> Result<usize, Error>;
   fn decode_uint256(&mut self, v:&mut UInt256) -> Result<usize, Error>;
   fn decode_array_u8(&mut self, v:&mut [u8]) -> Result<usize, Error>;
   fn decode_sequence_u8(&mut self, v:&mut Vec<u8>) -> Result<usize, Error>;
}

#[derive(Default)]
pub struct DecoderImpl { p:CodecParam }

impl Decoder for DecoderImpl {
   fn param(&self) -> &CodecParam { &self.p }

   fn decode_skip(&mut self, _v:usize) -> Result<usize, Error> { Ok(0) }
   
   fn decode_u8(&mut self, _v:&mut u8) -> Result<usize, Error> { Ok(0) }
   fn decode_u16le(&mut self, _v:&mut u16) -> Result<usize, Error> { Ok(0) }
   fn decode_u32le(&mut self, _v:&mut u32) -> Result<usize, Error> { Ok(0) }
   fn decode_u64le(&mut self, _v:&mut u64) -> Result<usize, Error> { Ok(0) }
   fn decode_u16be(&mut self, _v:&mut u16) -> Result<usize, Error> { Ok(0) }
   fn decode_u32be(&mut self, _v:&mut u32) -> Result<usize, Error> { Ok(0) }
   fn decode_u64be(&mut self, _v:&mut u64) -> Result<usize, Error> { Ok(0) }

   fn decode_i8(&mut self, _v:&mut i8) -> Result<usize, Error> { Ok(0) }
   fn decode_i16le(&mut self, _v:&mut i16) -> Result<usize, Error> { Ok(0) }
   fn decode_i32le(&mut self, _v:&mut i32) -> Result<usize, Error> { Ok(0) }
   fn decode_i64le(&mut self, _v:&mut i64) -> Result<usize, Error> { Ok(0) }
   fn decode_i16be(&mut self, _v:&mut i16) -> Result<usize, Error> { Ok(0) }
   fn decode_i32be(&mut self, _v:&mut i32) -> Result<usize, Error> { Ok(0) }
   fn decode_i64be(&mut self, _v:&mut i64) -> Result<usize, Error> { Ok(0) }
   
   fn decode_bool(&mut self, _v:&mut bool) -> Result<usize, Error> { Ok(0) }
   fn decode_varint(&mut self, _v:&mut u64) -> Result<usize, Error> { Ok(0) }
   fn decode_uint256(&mut self, _v:&mut UInt256) -> Result<usize, Error> { Ok(0) }
   fn decode_array_u8(&mut self, _v:&mut [u8]) -> Result<usize, Error> { Ok(0) }
   fn decode_sequence_u8(&mut self, _v:&mut Vec<u8>) -> Result<usize, Error> { Ok(0) }
}   


#[cfg(test)]
mod tests {
   use ::Error;
   use ::std::borrow::Borrow;
   use super::{Decoder, Decodee, DecoderImpl};
   struct FooParam { m:usize }
   struct Foo { n:usize }
   impl <D:Decoder>Decodee<D, FooParam> for Foo {
      fn decode<BP>(&mut self, p:BP, _d:&mut D) -> Result<usize, Error>
         where BP:Borrow<FooParam>+Sized
      {
         Ok(self.n * p.borrow().m)
      }
   }
   #[test]
   fn test() {
      let mut f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut e = DecoderImpl::default();
      assert_matches!(f.decode(&p, &mut e), Ok(6));
   }
}

