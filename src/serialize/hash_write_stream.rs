use std;
use super::WriteStream;
use ::crypto::Hasher;

pub struct HashWriteStream<T:Hasher> {
   hasher: T,
}
impl <T:Hasher> HashWriteStream<T> {
   pub fn new(inner:T) -> Self {
      HashWriteStream { hasher: inner }
   }
   pub fn rewind(&mut self) {
      self.hasher.reset();
   }
   pub fn result(&mut self) -> Box<[u8]> {
      self.hasher.result()
   }
   pub fn hexresult(&mut self) -> String {
      self.hasher.hexresult()
   }
}

impl <T:Hasher> std::io::Write for HashWriteStream<T> {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      self.hasher.input(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl <T:Hasher> WriteStream for HashWriteStream<T> { }


#[test]
fn test_hash_write_stream() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   use std::io::Write;
   let mut ws = HashWriteStream::new(::crypto::DHash256::default());
   assert_matches!(ws.write(input), Ok(5));
   assert_eq!(ws.hexresult(), expect);
}
