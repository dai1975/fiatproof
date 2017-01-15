use ::std::borrow::Borrow;
use ::Error;
use super::{BitcoinEncoder, BitcoinEncodee, BitcoinDecoder, BitcoinDecodee};

// I can impl for AsRef<[A]> but it is a bit irritative to import AsRef trait.
impl <'a,E,P,A> BitcoinEncodee<E,(usize,P)> for &'a [A]
   where E:BitcoinEncoder, A:BitcoinEncodee<E,P>+Sized
{
   fn encode<BP>(&self, p:BP, e:&mut E) -> Result<usize, Error>
      where BP:Borrow<(usize,P)>+Sized
   {
      let mut r:usize = 0;
      let len = self.len();
      {
         let lim = p.borrow().0;
         if lim < len {
            serialize_error!(format!("sequence exceeds limit: {} but {}", lim, len));
         }
      }
      r += try!(e.encode_varint(len as u64));
      for elm in self.iter() {
         r += try!(elm.encode(&p.borrow().1, e));
      }
      Ok(r)
   }
}
impl <E,P,A> BitcoinEncodee<E,(usize,P)> for Vec<A>
   where E:BitcoinEncoder, A:BitcoinEncodee<E,P>+Sized
{
   fn encode<BP>(&self, p:BP, e:&mut E) -> Result<usize, Error>
      where BP:Borrow<(usize,P)>+Sized
   {
      self.as_slice().encode(p, e)
   }
}

impl <D,P,A> BitcoinDecodee<D,(usize,P)> for Vec<A>
   where D:BitcoinDecoder, A:BitcoinDecodee<D,P>+Sized+Clone+Default
{
   fn decode<BP>(&mut self, p:BP, d:&mut D) -> Result<usize, Error>
      where BP:Borrow<(usize,P)>+Sized
   {
      let mut r:usize = 0;
      {
         let lim = p.borrow().0;
         let mut len:u64 = 0;
         r += try!(d.decode_varint(&mut len));
         if lim < (len as usize) {
            serialize_error!(format!("sequence exceeds limit: {} but {}", lim, len));
         }
         self.resize(len as usize, A::default());
      }
      for elm in self.iter_mut() {
         r += try!(elm.decode(&p.borrow().1, d));
      }
      Ok(r)
   }
}

/*
impl <D,A> BitcoinDecodee<D> for Vec<A>
   where D:BitcoinDecoder, A:BitcoinDecodee<D>+Default+Clone
{
   type P = (usize, Borrow<A::P>);
   fn decode<BP:Borrow<Self::P>+Sized>(&mut self, p:BP, d:&mut D) -> Result<usize, Error> {
   }
}
*/


#[cfg(test)]
mod tests {
   use ::Error;
   use ::std::borrow::Borrow;
   use super::super::encode::{BitcoinEncoder, BitcoinEncodee, BitcoinEncoderImpl};
   use super::super::decode::{BitcoinDecoder, BitcoinDecodee, BitcoinDecoderImpl};
   struct FooParam { m:usize }
   #[derive(Clone,Default)]
   struct Foo { n:usize }
   impl <E:BitcoinEncoder>BitcoinEncodee<E, FooParam> for Foo {
      fn encode<BP>(&self, p:BP, _e:&mut E) -> Result<usize, Error>
         where BP:Borrow<FooParam>+Sized
      {
         Ok(self.n * p.borrow().m)
      }
   }
   impl <D:BitcoinDecoder>BitcoinDecodee<D, FooParam> for Foo {
      fn decode<BP>(&mut self, p:BP, _d:&mut D) -> Result<usize, Error>
         where BP:Borrow<FooParam>+Sized
      {
         Ok(self.n * p.borrow().m)
      }
   }

   #[test]
   fn test_encode() {
      let v = vec![ Foo{n:1}, Foo{n:2}];
      let mut e = BitcoinEncoderImpl::default();
      {
         let p = (100usize, FooParam{ m:3 });
         assert_matches!(v.as_slice().encode(&p, &mut e), Ok(9));
      }
      {
         let p = (1usize, FooParam{ m:3 });
         assert_matches!(v.as_slice().encode(&p, &mut e), Err(_));
      }
   }
   #[test]
   fn test_decode() {
      let mut v = vec![ Foo{n:1}, Foo{n:2}];
      let mut d = BitcoinDecoderImpl::default();
      {
         let p = (100usize, FooParam{ m:3 });
         assert_matches!(v.decode(&p, &mut d), Ok(0));
      }
   }
}
