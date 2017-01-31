use ::std::borrow::Borrow;
use super::{WriteStream, Media};

pub trait Encoder {
   fn encode_skip(&mut self, w:&mut WriteStream, m:&Media, n:usize) -> ::Result<usize>;
   
   fn encode_u8(&mut self, w:&mut WriteStream, m:&Media, v:u8) -> ::Result<usize>;
   fn encode_u16le(&mut self, w:&mut WriteStream, m:&Media, v:u16) -> ::Result<usize>;
   fn encode_u32le(&mut self, w:&mut WriteStream, m:&Media, v:u32) -> ::Result<usize>;
   fn encode_u64le(&mut self, w:&mut WriteStream, m:&Media, v:u64) -> ::Result<usize>;
   fn encode_u16be(&mut self, w:&mut WriteStream, m:&Media, v:u16) -> ::Result<usize>;
   fn encode_u32be(&mut self, w:&mut WriteStream, m:&Media, v:u32) -> ::Result<usize>;
   fn encode_u64be(&mut self, w:&mut WriteStream, m:&Media, v:u64) -> ::Result<usize>;

   fn encode_i8(&mut self, w:&mut WriteStream, m:&Media, v:i8) -> ::Result<usize>;
   fn encode_i16le(&mut self, w:&mut WriteStream, m:&Media, v:i16) -> ::Result<usize>;
   fn encode_i32le(&mut self, w:&mut WriteStream, m:&Media, v:i32) -> ::Result<usize>;
   fn encode_i64le(&mut self, w:&mut WriteStream, m:&Media, v:i64) -> ::Result<usize>;
   fn encode_i16be(&mut self, w:&mut WriteStream, m:&Media, v:i16) -> ::Result<usize>;
   fn encode_i32be(&mut self, w:&mut WriteStream, m:&Media, v:i32) -> ::Result<usize>;
   fn encode_i64be(&mut self, w:&mut WriteStream, m:&Media, v:i64) -> ::Result<usize>;
   
   fn encode_bool(&mut self, w:&mut WriteStream, m:&Media, v:bool) -> ::Result<usize>;
   fn encode_varint(&mut self, w:&mut WriteStream, m:&Media, v:u64) -> ::Result<usize>;
   fn encode_array_u8(&mut self, w:&mut WriteStream, m:&Media, v:&[u8]) -> ::Result<usize>;
   fn encode_sequence_u8(&mut self, w:&mut WriteStream, m:&Media, v:&[u8]) -> ::Result<usize>;
}

pub trait EncodeStream {
   type W: WriteStream;
   type E: Encoder;
   fn stream(&mut self)    -> &mut Self::W;
   fn encoder(&mut self)   -> &mut Self::E;
   fn media(&self)         -> &Media;
   fn set_media(&mut self, m:Media) -> Media;
   fn update_media<F>(&mut self, f:F) -> Media where F: Fn(Media) -> Media {
      let m1 = f(self.media().clone());
      self.set_media(m1)
   }
   fn then<F>(&mut self, f:F) -> ::Result<usize> where F: Fn(&mut Self::W, &mut Self::E, &Media) -> ::Result<usize>;

   fn encode_skip(&mut self, n:usize) -> ::Result<usize> { self.then(|w,e,m| { e.encode_skip(w,m,n) }) }

   fn encode_u8(&mut self, v:u8) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u8(w,m,v) }) }
   fn encode_u16le(&mut self, v:u16) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u16le(w,m,v) }) }
   fn encode_u32le(&mut self, v:u32) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u32le(w,m,v) }) }
   fn encode_u64le(&mut self, v:u64) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u64le(w,m,v) }) }
   fn encode_u16be(&mut self, v:u16) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u16be(w,m,v) }) }
   fn encode_u32be(&mut self, v:u32) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u32be(w,m,v) }) }
   fn encode_u64be(&mut self, v:u64) -> ::Result<usize> { self.then(|w,e,m| { e.encode_u64be(w,m,v) }) }
   fn encode_i8(&mut self, v:i8) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i8(w,m,v) }) }
   fn encode_i16le(&mut self, v:i16) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i16le(w,m,v) }) }
   fn encode_i32le(&mut self, v:i32) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i32le(w,m,v) }) }
   fn encode_i64le(&mut self, v:i64) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i64le(w,m,v) }) }
   fn encode_i16be(&mut self, v:i16) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i16be(w,m,v) }) }
   fn encode_i32be(&mut self, v:i32) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i32be(w,m,v) }) }
   fn encode_i64be(&mut self, v:i64) -> ::Result<usize> { self.then(|w,e,m| { e.encode_i64be(w,m,v) }) }
   fn encode_bool(&mut self, v:bool) -> ::Result<usize> { self.then(|w,e,m| { e.encode_bool(w,m,v) }) }
   fn encode_varint(&mut self, v:u64) -> ::Result<usize> { self.then(|w,e,m| { e.encode_varint(w,m,v) }) }
   fn encode_array_u8(&mut self, v:&[u8]) -> ::Result<usize> { self.then(|w,e,m| { e.encode_array_u8(w,m,v) }) }
   fn encode_sequence_u8(&mut self, v:&[u8]) -> ::Result<usize> { self.then(|w,e,m| { e.encode_sequence_u8(w,m,v) }) }
}

pub trait Encodee {
   type P;
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize>;
}



