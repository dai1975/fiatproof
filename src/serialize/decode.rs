use ::{Error, UInt256};
use super::BitcoinCodecParam;

pub trait BitcoinDecoder: Sized {
   fn param(&self) -> &BitcoinCodecParam;

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
   
   fn decode<A:BitcoinDecodee<Self>>(&mut self, v:&mut A) -> Result<usize, Error> {
      v.decode(self)
   }
   fn decode_sequence<A:BitcoinDecodee<Self>>(&mut self, v:&mut Vec<A>) -> Result<usize, Error> {
      let mut r:usize = 0;
      {
         let mut len:u64 = 0;
         r += try!(self.decode_varint(&mut len));
         v.resize(len as usize, A::default());
      }
      for elm in v.iter() {
         r += try!(elm.decode(self));
      }
      Ok(r)
   }
}

pub trait BitcoinDecodee<E:BitcoinDecoder>: Default + Clone {
   fn decode(&self, e:&mut E) -> Result<usize, Error>;
}   
