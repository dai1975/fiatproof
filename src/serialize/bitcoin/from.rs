use ::serialize::{OutofOctets};
use super::{Decoder, Decodee, Medium};

impl <T> OutofOctets<Decodee> for T where T:Decodee {
   fn outof_octets<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<usize> {
      use ::serialize::SliceReadStream;
      let mut r = SliceReadStream::new(s.as_ref());
      let mut d = Decoder::new(&mut r, &Medium::default());
      self.decode(&mut d)
   }
}

#[cfg(test)]
mod tests {
   use ::serialize::bitcoin::{Decoder, Decodee};
   #[derive(Default)] //WithByte needs Default
   struct Foo { s: String }
   impl Decodee for Foo {
      fn decode(&mut self, d:&mut Decoder) -> ::Result<usize> {
         d.decode_string(&mut self.s, None)
      }
   }
   
   #[test]
   fn test_from_octets() {
      let buf:&[u8] = &[
         12,
         0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75,
      ];
      {
         use ::serialize::OutofOctets;
         let mut f = Foo::default();
         assert_matches!(f.outof_octets(&buf), Ok(13));
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
      {
         use ::serialize::FromOctets;
         let f = Foo::from_octets(&buf).unwrap();
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
   }
}
