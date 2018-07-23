use super::Medium;

const FLAG_UNSIZED:u32      = 1u32 << 0;

fn parse_opt(opt:&str) -> Result<(u32, Medium), ::ParseError> {
   let r = opt.split(',').fold(Ok((0, "".to_string())), |acc,s| {
      match (acc,s) {
         (Err(e), _)               => Err(e),
         (Ok((f,rest)), "unsized") => Ok((f|FLAG_UNSIZED, rest)),
         (Ok((f,rest)), _)         => Ok((f, rest + s)),
      }
   });
   match r {
      Err(e) => Err(e),
      Ok((flags,rest)) => match Medium::new(rest.as_str()) {
         Ok(m)  => Ok((flags,m)),
         Err(e) => Err(e),
      },
   }
}

use ::serialize::{OutofOctets, ToOctets, VecWriteStream, SliceReadStream};
use super::{Encoder, Encodee, Decoder, Decodee};

impl <E> ToOctets<Encodee> for E where E:Encodee {
   fn to_octets(&self, opt:&str) -> ::Result<Vec<u8>> {
      let (flags, medium) = try!(parse_opt(opt));
      let mut v = {
         let mut w = VecWriteStream::default();
         {
            let mut e = Encoder::new(&mut w, &medium);
            let _ = try!(self.encode(&mut e));
         }
         w.into_inner()
      };
      if flags & FLAG_UNSIZED != 0 {
         let skipsize = {
            let mut r = SliceReadStream::new(v.as_slice());
            let mut d = Decoder::new(&mut r, &medium);
            let mut size = 0u64;
            let x = try!(d.decode_var_int(&mut size));
            let size = size as usize;
            if x + size != v.len() {
               parse_error!(format!("size mismatch: {} but {}(with {})", size, v.len(), x));
            }
            x
         };
         Ok(v.split_off(skipsize))
      } else {
         Ok(v)
      }
   }
}

impl <T> OutofOctets<Decodee> for T where T:Decodee {
   fn outof_octets<S:AsRef<[u8]>>(&mut self, s:S, opt:&str) -> ::Result<usize> {
      let (flags, medium) = try!(parse_opt(opt));
      
      if flags & FLAG_UNSIZED != 0 {
         let mut w = VecWriteStream::default();
         {
            let mut e = Encoder::new(&mut w, &medium);
            let _ = try!(e.encode_var_octets(s.as_ref(), ::std::usize::MAX));
         }
         let v = w.into_inner();
         let mut r = SliceReadStream::new(v.as_slice());
         let mut d = Decoder::new(&mut r, &medium);
         self.decode(&mut d)
      } else {
         let mut r = SliceReadStream::new(s.as_ref());
         let mut d = Decoder::new(&mut r, &medium);
         self.decode(&mut d)
      }
   }
}

#[cfg(test)]
mod tests {
   use ::bitcoin::serialize::{Decoder, Decodee};
   #[derive(Default)] //FromOctets needs Default
   struct Foo { s: String }
   impl Decodee for Foo {
      fn decode(&mut self, d:&mut Decoder) -> ::Result<usize> {
         d.decode_var_string(&mut self.s, ::std::usize::MAX)
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
         assert_matches!(f.outof_octets(&buf, ""), Ok(13));
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
      {
         use ::serialize::FromOctets;
         let f = Foo::from_octets(&buf, "").unwrap();
         assert_eq!(f.s.as_str(), "Hatsune Miku");
      }
   }
   
   use ::bitcoin::serialize::{Encoder, Encodee};
   struct Bar<'a> { s: &'a str }
   impl <'a> Encodee for Bar<'a> {
      fn encode(&self, e:&mut Encoder) -> ::Result<usize> {
         e.encode_octets(self.s.as_bytes())
      }
   }
   
   #[test]
   fn test_to_bytes() {
      use ::serialize::ToOctets;
      let f = Bar { s:"Hatsune Miku" };
      let v = f.to_octets("hash").unwrap();
      assert_eq!(&[0x48, 0x61, 0x74, 0x73, 0x75, 0x6e, 0x65, 0x20, 0x4d, 0x69, 0x6b, 0x75], &v[..]);
   }

   #[test]
   fn test_to_digest() {
      use ::serialize::ToDigest;
      let f = Bar { s:"Hatsune Miku" };
      let v = f.to_dhash256_hex_string("").unwrap();
      assert_eq!("e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9", v.as_str());
      let v = f.to_hash160_hex_string("").unwrap();
      assert_eq!("b7233a798e6ea977644ded49241c2b153a6617b9", v.as_str());
      let v = f.to_hash160_hex_string_rev("").unwrap();
      assert_eq!("b917663a152b1c2449ed4d6477a96e8e793a23b7", v.as_str());
   }
}
