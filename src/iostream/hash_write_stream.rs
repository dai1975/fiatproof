use std;
use super::WriteStream;
use ::crypto::digest;

pub struct HashWriteStream<D: digest::Digest> {
   digest: D,
}
impl <D: digest::Digest> HashWriteStream<D> {
   pub fn new(digest:D) -> Self {
      HashWriteStream { digest:digest }
   }
   pub fn rewind(&mut self) {
      self.digest.reset();
   }
   pub fn result(&mut self) -> Box<[u8]> {
      digest::helper::result_u8(&mut self.digest)
   }
   pub fn hexresult(&mut self) -> String {
      digest::helper::result_hex(&mut self.digest)
   }
}

impl <D: digest::Digest> std::io::Write for HashWriteStream<D> {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      self.digest.input(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl <D: digest::Digest> WriteStream for HashWriteStream<D> {
   fn write_skip(&mut self, _n:usize) -> Result<usize, ::std::io::Error> {
      Err(::std::io::Error::new(::std::io::ErrorKind::Other, "cannot skip"))
   }
}


#[test]
fn test_hash_write_stream() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   use std::io::Write;
   let mut ws = HashWriteStream::new(::crypto::digest::DHash256::new());
   assert_matches!(ws.write(input), Ok(5));
   assert_eq!(ws.hexresult(), expect);
}
