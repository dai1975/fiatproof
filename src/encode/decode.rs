use ::std::borrow::Borrow;
use super::{ReadStream, Media};

pub trait Decoder {
   fn decode_skip(&mut self, r:&mut ReadStream, m:&Media, _v:usize) -> ::Result<usize>;
   
   fn decode_u8(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u8) -> ::Result<usize>;
   fn decode_u16le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u16) -> ::Result<usize>;
   fn decode_u32le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u32) -> ::Result<usize>;
   fn decode_u64le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u64) -> ::Result<usize>;
   fn decode_u16be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u16) -> ::Result<usize>;
   fn decode_u32be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u32) -> ::Result<usize>;
   fn decode_u64be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u64) -> ::Result<usize>;
   
   fn decode_i8(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i8) -> ::Result<usize>;
   fn decode_i16le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i16) -> ::Result<usize>;
   fn decode_i32le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i32) -> ::Result<usize>;
   fn decode_i64le(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i64) -> ::Result<usize>;
   fn decode_i16be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i16) -> ::Result<usize>;
   fn decode_i32be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i32) -> ::Result<usize>;
   fn decode_i64be(&mut self, r:&mut ReadStream, m:&Media, _v:&mut i64) -> ::Result<usize>;
   
   fn decode_bool(&mut self, r:&mut ReadStream, m:&Media, _v:&mut bool) -> ::Result<usize>;
   fn decode_varint(&mut self, r:&mut ReadStream, m:&Media, _v:&mut u64) -> ::Result<usize>;
   fn decode_array_u8(&mut self, r:&mut ReadStream, m:&Media, _v:&mut [u8]) -> ::Result<usize>;
   fn decode_sequence_u8(&mut self, r:&mut ReadStream, m:&Media, _v:&mut Vec<u8>) -> ::Result<usize>;
}

pub trait DecodeStream {
   type R: ReadStream;
   type D: Decoder;

   fn stream(&mut self)  -> &mut Self::R;
   fn decoder(&mut self) -> &mut Self::D;
   fn media(&self)       -> &Media;
   fn then<F>(&mut self, f:F) -> ::Result<usize> where F: FnMut(&mut Self::R, &mut Self::D, &Media) -> ::Result<usize>;

   fn decode_skip(&mut self, n:usize) -> ::Result<usize> { self.then(|r,d,m| { d.decode_skip(r,m,n) }) }

   fn decode_u8(&mut self, v:&mut u8) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u8(r,m,v) }) }
   fn decode_u16le(&mut self, v:&mut u16) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u16le(r,m,v) }) }
   fn decode_u32le(&mut self, v:&mut u32) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u32le(r,m,v) }) }
   fn decode_u64le(&mut self, v:&mut u64) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u64le(r,m,v) }) }
   fn decode_u16be(&mut self, v:&mut u16) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u16be(r,m,v) }) }
   fn decode_u32be(&mut self, v:&mut u32) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u32be(r,m,v) }) }
   fn decode_u64be(&mut self, v:&mut u64) -> ::Result<usize> { self.then(|r,d,m| { d.decode_u64be(r,m,v) }) }
   fn decode_i8(&mut self, v:&mut i8) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i8(r,m,v) }) }
   fn decode_i16le(&mut self, v:&mut i16) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i16le(r,m,v) }) }
   fn decode_i32le(&mut self, v:&mut i32) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i32le(r,m,v) }) }
   fn decode_i64le(&mut self, v:&mut i64) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i64le(r,m,v) }) }
   fn decode_i16be(&mut self, v:&mut i16) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i16be(r,m,v) }) }
   fn decode_i32be(&mut self, v:&mut i32) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i32be(r,m,v) }) }
   fn decode_i64be(&mut self, v:&mut i64) -> ::Result<usize> { self.then(|r,d,m| { d.decode_i64be(r,m,v) }) }
   fn decode_bool(&mut self, v:&mut bool) -> ::Result<usize> { self.then(|r,d,m| { d.decode_bool(r,m,v) }) }
   fn decode_varint(&mut self, v:&mut u64) -> ::Result<usize> { self.then(|r,d,m| { d.decode_varint(r,m,v) }) }
   fn decode_array_u8(&mut self, v:&mut [u8]) -> ::Result<usize> { self.then(|r,d,m| { d.decode_array_u8(r,m,v) }) }
   fn decode_sequence_u8(&mut self, v:&mut Vec<u8>) -> ::Result<usize> { self.then(|r,d,m| { d.decode_sequence_u8(r,m,v) }) }
}

pub trait Decodee {
   type P;
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, p:BP) -> ::Result<usize>;
}
