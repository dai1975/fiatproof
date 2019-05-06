use crypto::digest::Digest;
use crate::crypto::digest::helper;

pub struct DigestWrite<D: Digest> {
   digest: D,
}
impl <D: Digest> DigestWrite<D> {
   pub fn new(digest:D) -> Self {
      Self { digest:digest }
   }
}
impl <D: Digest> Digest for DigestWrite<D> {
   #[inline] fn input(&mut self, input: &[u8]) { self.digest.input(input) }
   #[inline] fn input_str(&mut self, input: &str) { self.digest.input_str(input) }
   #[inline] fn result(&mut self, out: &mut [u8]) { self.digest.result(out) }
   #[inline] fn result_str(&mut self) -> String { self.digest.result_str() }
   #[inline] fn reset(&mut self) { self.digest.reset() }
   #[inline] fn output_bits(&self) -> usize { self.digest.output_bits() }
   #[inline] fn output_bytes(&self) -> usize { self.digest.output_bytes() }
   #[inline] fn block_size(&self) -> usize { self.digest.block_size() }
}

impl <D: Digest> std::io::Write for DigestWrite<D> {
   fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
      self.input(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> Result<(), std::io::Error> {
      Ok(())
   }
}


#[test]
fn test_digest_write() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   use std::io::Write;
   use crate::crypto::digest::{ DigestWrite, DHash256 };
   let mut ws = DigestWrite::new(DHash256::new());
   assert_matches!(ws.write(input), Ok(5));
   assert_eq!(ws.result_str(), expect);
}
