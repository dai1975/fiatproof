use std;
extern crate byteorder;
use self::byteorder::{LittleEndian, BigEndian, WriteBytesExt};

pub trait WriteStream: WriteBytesExt {
   fn write_u16le(&mut self, v:u16) -> Result<(), std::io::Error> { self.write_u16::<LittleEndian>(v) }
   fn write_u32le(&mut self, v:u32) -> Result<(), std::io::Error> { self.write_u32::<LittleEndian>(v) }
   fn write_u64le(&mut self, v:u64) -> Result<(), std::io::Error> { self.write_u64::<LittleEndian>(v) }
   fn write_i16le(&mut self, v:i16) -> Result<(), std::io::Error> { self.write_i16::<LittleEndian>(v) }
   fn write_i32le(&mut self, v:i32) -> Result<(), std::io::Error> { self.write_i32::<LittleEndian>(v) }
   fn write_i64le(&mut self, v:i64) -> Result<(), std::io::Error> { self.write_i64::<LittleEndian>(v) }

   fn write_u16be(&mut self, v:u16) -> Result<(), std::io::Error> { self.write_u16::<BigEndian>(v) }
   fn write_u32be(&mut self, v:u32) -> Result<(), std::io::Error> { self.write_u32::<BigEndian>(v) }
   fn write_u64be(&mut self, v:u64) -> Result<(), std::io::Error> { self.write_u64::<BigEndian>(v) }
   fn write_i16be(&mut self, v:i16) -> Result<(), std::io::Error> { self.write_i16::<BigEndian>(v) }
   fn write_i32be(&mut self, v:i32) -> Result<(), std::io::Error> { self.write_i32::<BigEndian>(v) }
   fn write_i64be(&mut self, v:i64) -> Result<(), std::io::Error> { self.write_i64::<BigEndian>(v) }
}

impl <'a> WriteStream for std::io::Cursor<&'a mut [u8]> { }
impl      WriteStream for std::io::Cursor<Vec<u8>> { }
impl      WriteStream for std::io::Cursor<Box<[u8]>> { }

pub struct SizeSink {
   size_: usize,
}
impl SizeSink {
   pub fn new() -> Self { SizeSink { size_: 0 } }
   pub fn reset_size(&mut self) { self.size_ = 0; }
   pub fn size(&self) -> usize { self.size_ }
}
impl std::io::Write for SizeSink {
   fn write(&mut self, buf:&[u8]) -> std::io::Result<usize> {
      self.size_ += buf.len();
      Ok(buf.len())
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl WriteStream for SizeSink { }


pub struct SliceWriteStream<T: std::borrow::BorrowMut<[u8]>> {
   inner:  T,
   cursor: usize,
}
impl <T: std::borrow::BorrowMut<[u8]>> SliceWriteStream<T> {
   pub fn new(inner:T) -> Self {
      SliceWriteStream { inner:inner, cursor:0 }
   }
   pub fn get_ref(&self) -> &[u8] {
      self.inner.borrow()
   }
   pub fn reset(&mut self) {
      self.cursor = 0;
   }
}
impl <T: std::borrow::BorrowMut<[u8]>> std::io::Write for SliceWriteStream<T> {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      let size = buf.len();
      let (_,o) = self.inner.borrow_mut().split_at_mut(self.cursor);
      if o.len() < size {
         Err(std::io::Error::new(std::io::ErrorKind::WouldBlock, "no capacity to write"))
      } else {
         o[..size].copy_from_slice(buf);
         self.cursor += size;
         Ok(size)
      }
   }
   fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
impl <T: std::borrow::BorrowMut<[u8]>> WriteStream for SliceWriteStream<T> { }

pub struct FixedWriteStream {
   inner: SliceWriteStream<Box<[u8]>>,
}
impl FixedWriteStream {
   pub fn new(size:usize) -> Self {
      FixedWriteStream { inner: SliceWriteStream::new(vec![0u8; size].into_boxed_slice()) }
   }
   pub fn get_ref(&self) -> &[u8] { self.inner.get_ref() }
   pub fn reset(&mut self) { self.inner.reset() }
}
impl std::io::Write for FixedWriteStream {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { self.inner.write(buf) }
   fn flush(&mut self) -> std::io::Result<()> { self.inner.flush() }
}
impl WriteStream for FixedWriteStream { }
