use ::std::convert::AsRef;
use ::ui::h2b;

pub trait OutofOctets<T> where T:?Sized {
   fn outof_octets<S:AsRef<[u8]>>(&mut self, s:S, opt:&str) -> ::Result<usize>;

   fn outof_hex<S:AsRef<str>>(&mut self, s:S, opt:&str) -> ::Result<usize> {
      h2b(s).and_then(|b| self.outof_octets(&b, opt))
   }
   fn outof_octets_rev<S:AsRef<[u8]>>(&mut self, s:S, opt:&str) -> ::Result<usize> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      self.outof_octets(rev.as_slice(), opt)
   }
   fn outof_hex_string_rev<S:AsRef<str>>(&mut self, s:S, opt:&str) -> ::Result<usize> {
      h2b(s).and_then(|b| self.outof_octets_rev(&b, opt))
   }
}

pub trait FromOctets<T>: Sized where T:?Sized {
   fn from_octets<S:AsRef<[u8]>>(s:S, opt:&str) -> ::Result<Self>;
   fn from_hex_string<S:AsRef<str>>(s:S, opt:&str) -> ::Result<Self> {
      h2b(s).and_then(|b| Self::from_octets(&b, opt))
   }
   fn from_octets_rev<S:AsRef<[u8]>>(&mut self, s:S, opt:&str) -> ::Result<Self> {
      let mut rev = Vec::<u8>::from(s.as_ref());
      rev.reverse();
      Self::from_octets(rev.as_slice(), opt)
   }
   fn from_hex_string_rev<S:AsRef<str>>(s:S, opt:&str) -> ::Result<Self> {
      h2b(s).and_then(|mut b| {
         b.reverse();
         Self::from_octets(&b, opt)
      })
   }
}

impl <T,X> FromOctets<T> for X where T:?Sized, X:OutofOctets<T>+Default {
   fn from_octets<S:AsRef<[u8]>>(s:S, opt:&str) -> ::Result<Self> {
      let mut r = Self::default();
      r.outof_octets(s, opt).map(|_| r)
   }
}


