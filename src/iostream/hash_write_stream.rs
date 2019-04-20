use crypto::digest::Digest;
use super::WriteStream;
use crate::crypto::digest::helper;

pub struct HashWriteStream<D: Digest> {
   digest: D,
}
impl <D: Digest> HashWriteStream<D> {
   pub fn new(digest:D) -> Self {
      HashWriteStream { digest:digest }
   }
   pub fn rewind(&mut self) {
      self.digest.reset();
   }
   pub fn result(&mut self) -> Box<[u8]> {
      helper::result_u8(&mut self.digest)
   }
   pub fn hexresult(&mut self) -> String {
      helper::result_hex(&mut self.digest)
   }
}

impl <D: Digest> std::io::Write for HashWriteStream<D> {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      self.digest.input(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl <D: Digest> WriteStream for HashWriteStream<D> {
   fn write_skip(&mut self, _n:usize) -> Result<usize, std::io::Error> {
      Err(std::io::Error::new(std::io::ErrorKind::Other, "cannot skip"))
   }
}


#[test]
fn test_hash_write_stream() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   use std::io::Write;
   let mut ws = HashWriteStream::new(crate::crypto::digest::DHash256::new());
   assert_matches!(ws.write(input), Ok(5));
   assert_eq!(ws.hexresult(), expect);
}
