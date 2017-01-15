use ::std::borrow::Borrow;
use ::{Error, UInt256};
use super::BitcoinCodecParam;

pub trait BitcoinEncodee<E, P> where E:BitcoinEncoder {
   fn encode<BP>(&self, p:BP, e:&mut E) -> Result<usize, Error>
      where BP:Borrow<P>+Sized;
}

pub trait BitcoinEncoder: Sized {
   fn param(&self) -> &BitcoinCodecParam;

   fn encode_u8(&mut self, v:u8) -> Result<usize, Error>;
   fn encode_u16le(&mut self, v:u16) -> Result<usize, Error>;
   fn encode_u32le(&mut self, v:u32) -> Result<usize, Error>;
   fn encode_u64le(&mut self, v:u64) -> Result<usize, Error>;
   fn encode_u16be(&mut self, v:u16) -> Result<usize, Error>;
   fn encode_u32be(&mut self, v:u32) -> Result<usize, Error>;
   fn encode_u64be(&mut self, v:u64) -> Result<usize, Error>;

   fn encode_i8(&mut self, v:i8) -> Result<usize, Error>;
   fn encode_i16le(&mut self, v:i16) -> Result<usize, Error>;
   fn encode_i32le(&mut self, v:i32) -> Result<usize, Error>;
   fn encode_i64le(&mut self, v:i64) -> Result<usize, Error>;
   fn encode_i16be(&mut self, v:i16) -> Result<usize, Error>;
   fn encode_i32be(&mut self, v:i32) -> Result<usize, Error>;
   fn encode_i64be(&mut self, v:i64) -> Result<usize, Error>;
   
   fn encode_bool(&mut self, v:bool) -> Result<usize, Error>;
   fn encode_varint(&mut self, v:u64) -> Result<usize, Error>;
   fn encode_uint256(&mut self, v:&UInt256) -> Result<usize, Error>;
   fn encode_array_u8(&mut self, v:&[u8]) -> Result<usize, Error>;
   fn encode_sequence_u8(&mut self, v:&[u8]) -> Result<usize, Error>;

   /*
   fn encode<A,P,BA,BP>(&mut self, v:BA, p:BP) -> Result<usize, Error>
      where A:BitcoinEncodee<Self,P>, BA:Borrow<A>+Sized, BP:Borrow<P>+Sized,
   {
      v.borrow().encode(p, self)
   }
    */
}

#[derive(Default)]
pub struct BitcoinEncoderImpl { p:BitcoinCodecParam }

impl BitcoinEncoder for BitcoinEncoderImpl {
   fn param(&self) -> &BitcoinCodecParam { &self.p }

   fn encode_u8(&mut self, _v:u8) -> Result<usize, Error> { Ok(0) }
   fn encode_u16le(&mut self, _v:u16) -> Result<usize, Error> { Ok(0) }
   fn encode_u32le(&mut self, _v:u32) -> Result<usize, Error> { Ok(0) }
   fn encode_u64le(&mut self, _v:u64) -> Result<usize, Error> { Ok(0) }
   fn encode_u16be(&mut self, _v:u16) -> Result<usize, Error> { Ok(0) }
   fn encode_u32be(&mut self, _v:u32) -> Result<usize, Error> { Ok(0) }
   fn encode_u64be(&mut self, _v:u64) -> Result<usize, Error> { Ok(0) }

   fn encode_i8(&mut self, _v:i8) -> Result<usize, Error> { Ok(0) }
   fn encode_i16le(&mut self, _v:i16) -> Result<usize, Error> { Ok(0) }
   fn encode_i32le(&mut self, _v:i32) -> Result<usize, Error> { Ok(0) }
   fn encode_i64le(&mut self, _v:i64) -> Result<usize, Error> { Ok(0) }
   fn encode_i16be(&mut self, _v:i16) -> Result<usize, Error> { Ok(0) }
   fn encode_i32be(&mut self, _v:i32) -> Result<usize, Error> { Ok(0) }
   fn encode_i64be(&mut self, _v:i64) -> Result<usize, Error> { Ok(0) }
   
   fn encode_bool(&mut self, _v:bool) -> Result<usize, Error> { Ok(0) }
   fn encode_varint(&mut self, _v:u64) -> Result<usize, Error> { Ok(0) }
   fn encode_uint256(&mut self, _v:&UInt256) -> Result<usize, Error> { Ok(0) }
   fn encode_array_u8(&mut self, _v:&[u8]) -> Result<usize, Error> { Ok(0) }
   fn encode_sequence_u8(&mut self, _v:&[u8]) -> Result<usize, Error> { Ok(0) }
}   


#[cfg(test)]
mod tests {
   use ::Error;
   use ::std::borrow::Borrow;
   use super::{BitcoinEncoder, BitcoinEncodee, BitcoinEncoderImpl};
   struct FooParam { m:usize }
   struct Foo { n:usize }
   impl <E:BitcoinEncoder>BitcoinEncodee<E, FooParam> for Foo {
      fn encode<BP>(&self, p:BP, _e:&mut E) -> Result<usize, Error>
         where BP:Borrow<FooParam>+Sized
      {
         Ok(self.n * p.borrow().m)
      }
   }
   #[test]
   fn test() {
      let f = Foo{ n:2 };
      let p = FooParam{ m:3 };
      let mut e = BitcoinEncoderImpl::default();
      assert_matches!(f.encode(&p, &mut e), Ok(6));
   }
}

