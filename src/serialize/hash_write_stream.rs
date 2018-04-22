extern crate crypto;
use ::crypto::Digest;
use std;
use super::WriteStream;

pub struct HashWriteStream<T:Digest> {
   hasher: T,
}
impl <T:Digest> HashWriteStream<T> {
   pub fn new() -> Self {
      HashWriteStream { hasher: <T>::default() }
   }
   pub fn rewind(&mut self) {
      self.hasher.reset();
   }
   pub fn result(&mut self) -> Box<[u8]> {
      self.hasher.result_box()
   }
   pub fn hexresult(&mut self) -> String {
      self.hasher.result_str()
   }
}

impl <T:Digest> std::io::Write for HashWriteStream<T> {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      self.hasher.input(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl <T:Digest> WriteStream for HashWriteStream<T> {
   fn write_skip(&mut self, _n:usize) -> Result<usize, ::std::io::Error> {
      Err(::std::io::Error::new(::std::io::ErrorKind::Other, "cannot skip"))
   }
}


#[test]
fn test_hash_write_stream() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   use std::io::Write;
   let mut ws = HashWriteStream::<::crypto::DHash256>::new();
   assert_matches!(ws.write(input), Ok(5));
   assert_eq!(ws.hexresult(), expect);
}
