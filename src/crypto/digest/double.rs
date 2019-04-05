use super::Digest;

pub struct Double<T1,T2> where T1:Digest, T2:Digest {
   d1:T1,
   d2:T2,
}

impl <T1,T2> Digest for Double<T1, T2> where T1:Digest, T2:Digest {
   fn input(&mut self, input: &[u8]) {
      self.d1.input(input)
   }
   fn input_str(&mut self, input: &str) {
      self.d1.input_str(input)
   }
   
   fn result(&mut self, out: &mut [u8]) {
      let mut tmp = Vec::<u8>::with_capacity(self.d1.output_bytes());
      tmp.resize(self.d1.output_bytes(), 0);
      self.d1.result(tmp.as_mut_slice());
      self.d2.input(tmp.as_slice());
      self.d2.result(out)
   }
   fn result_str(&mut self) -> String {
      let mut tmp = Vec::<u8>::with_capacity(self.d1.output_bytes());
      tmp.resize(self.d1.output_bytes(), 0);
      self.d1.result(tmp.as_mut_slice());
      self.d2.input(tmp.as_slice());
      self.d2.result_str()
   }
   
   fn reset(&mut self) {
      self.d1.reset();
      self.d2.reset();
   }
   
   fn output_bits(&self) -> usize {
      self.d2.output_bits()
   }
   fn output_bytes(&self) -> usize {
      self.d2.output_bytes()
   }
   fn block_size(&self) -> usize {
      self.d1.block_size()
   }
}

macro_rules! def_double {
   ($h:ident, $d1:path, $d2:path) => {
      pub type $h = Double<$d1, $d2>;
      impl $h {
         pub fn new() -> Self {
            Self { d1: <$d1>::new(), d2: <$d2>::new() }
         }
      }
   };
}
def_double!(DHash256, super::Sha256, super::Sha256);
def_double!(Hash160,  super::Sha256, super::Ripemd160);

#[test]
fn test_dhash256() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9";
      
   let mut d = super::DHash256::new();
   assert_eq!(32, d.output_bytes());
   assert_eq!(expect, crate::crypto::digest::u8_to_hex(&mut d, input));
}

#[test]
fn test_hash160() {
   let input:&[u8]  = b"Hatsune Miku";
   let expect = "b7233a798e6ea977644ded49241c2b153a6617b9";

   let mut d = super::Hash160::new();
   assert_eq!(20, d.output_bytes());
   assert_eq!(expect, crate::crypto::digest::u8_to_hex(&mut d, input));
}


