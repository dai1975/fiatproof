use ::std::convert::AsRef;
use ::utils::h2b;

pub trait OutofOctets<T> where T:?Sized {
   fn outof_octets<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize>;

   fn outof_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<usize> {
      h2b(s).and_then(|b| self.outof_octets(&b))
   }
   fn outof_octets_rev<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      self.outof_octets(rev.as_slice())
   }
   fn outof_hex_string_rev<S:AsRef<str>>(&mut self, s:S) -> ::Result<usize> {
      h2b(s).and_then(|b| self.outof_octets_rev(&b))
   }
}

pub trait FromOctets<T>: Sized where T:?Sized {
   fn from_octets<S:AsRef<[u8]>>(s:S) -> ::Result<Self>;
   fn from_hex_string<S:AsRef<str>>(s:S) -> ::Result<Self> {
      h2b(s).and_then(|b| Self::from_octets(&b))
   }
   fn from_octets_rev<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<Self> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      Self::from_octets(rev.as_slice())
   }
   fn from_hex_string_rev<S:AsRef<str>>(s:S) -> ::Result<Self> {
      h2b(s).and_then(|mut b| {
         b.reverse();
         Self::from_octets(&b)
      })
   }
}

impl <T,X> FromOctets<T> for X where T:?Sized, X:OutofOctets<T>+Default {
   fn from_octets<S:AsRef<[u8]>>(s:S) -> ::Result<Self> {
      let mut r = Self::default();
      r.outof_octets(s).map(|_| r)
   }
}


