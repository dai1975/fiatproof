use ::std::borrow::Borrow;
use super::{Encodee, EncodeStream, Decodee, DecodeStream};

// I can impl for AsRef<[A]> but it is a bit irritative to import AsRef trait.
impl <'a,A:Encodee> Encodee for &'a [A] {
   type P = (usize, A::P);
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      let len = self.len();
      {
         let lim = p.borrow().0;
         if lim < len {
            encode_error!(format!("sequence exceeds limit: {} but {}", lim, len));
         }
      }
      r += try!(e.encode_varint(len as u64));
      for elm in self.iter() {
         r += try!(elm.encode(e, &p.borrow().1));
      }
      Ok(r)
   }
}
impl <A:Encodee> Encodee for Vec<A> {
   type P = (usize, A::P);
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, p:BP) -> ::Result<usize> {
      self.as_slice().encode(e, p)
   }
}

impl <A:Decodee+Default> Decodee for Vec<A> {
   type P = (usize, A::P);
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      {
         let lim = p.borrow().0;
         let mut len:u64 = 0;
         r += try!(d.decode_varint(&mut len));
         if lim < (len as usize) {
            encode_error!(format!("sequence exceeds limit: {} but {}", lim, len));
         }
         let len = len as usize;
         if self.len() < len {
            let ext = len - self.len();
            self.reserve(ext);
            for _ in 0..ext {
               self.push(A::default());
            }
         } else if len < self.len() {
            self.truncate(len);
         }
      }
      for elm in self.iter_mut() {
         r += try!(elm.decode(d, &p.borrow().1));
      }
      Ok(r)
   }
}


#[cfg(test)]
mod tests {
   use ::std::borrow::Borrow;
   use super::super::{EncodeStream, Encodee, DecodeStream, Decodee};
   #[derive(Default)]
   struct Foo { n:usize }
   struct FooParam { m:usize }
   impl Encodee for Foo {
      type P = FooParam;
      fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, _e:&mut ES, p:BP) -> ::Result<usize> {
         Ok(self.n * p.borrow().m)
      }
   }
   impl Decodee for Foo {
      type P = FooParam;
      fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, _d:&mut DS, p:BP) -> ::Result<usize> {
         Ok(self.n * p.borrow().m)
      }
   }

   #[test]
   fn test_encode() {
      use ::serialize::{BitcoinEncodeStream, VecWriteStream, Media};
      let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
      let v = vec![ Foo{n:1}, Foo{n:2}];
      {
         let p = (100usize, FooParam{ m:3 });
         assert_matches!(v.as_slice().encode(&mut e, &p), Ok(10)); // 1(=varint) + 1*3 + 2*3
      }
      {
         let p = (1usize, FooParam{ m:3 });
         assert_matches!(v.as_slice().encode(&mut e, &p), Err(_));
      }
   }
   #[test]
   fn test_decode() {
      use ::serialize::{BitcoinDecodeStream, SliceReadStream, Media};
      let mut d = BitcoinDecodeStream::new(SliceReadStream::new([0u8]), Media::default().set_net());
      let mut v = vec![ Foo{n:1}, Foo{n:2}];
      {
         let p = (100usize, FooParam{ m:3 });
         assert_matches!(v.decode(&mut d, &p), Ok(1));
      }
   }
}
