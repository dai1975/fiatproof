use std;
extern crate byteorder;
use self::byteorder::{LittleEndian, BigEndian, ReadBytesExt};

pub trait ReadStream: ReadBytesExt {
   fn read_u8_to(&mut self, v:&mut u8) -> Result<(), std::io::Error> { self.read_u8().map(|r| { *v = r; }) }
   fn read_i8_to(&mut self, v:&mut i8) -> Result<(), std::io::Error> { self.read_i8().map(|r| { *v = r; }) }
   
   fn read_u16le_to(&mut self, v:&mut u16) -> Result<(), std::io::Error> { self.read_u16::<LittleEndian>().map(|r| { *v = r; }) }
   fn read_u32le_to(&mut self, v:&mut u32) -> Result<(), std::io::Error> { self.read_u32::<LittleEndian>().map(|r| { *v = r; }) }
   fn read_u64le_to(&mut self, v:&mut u64) -> Result<(), std::io::Error> { self.read_u64::<LittleEndian>().map(|r| { *v = r; }) }
   fn read_i16le_to(&mut self, v:&mut i16) -> Result<(), std::io::Error> { self.read_i16::<LittleEndian>().map(|r| { *v = r; }) }
   fn read_i32le_to(&mut self, v:&mut i32) -> Result<(), std::io::Error> { self.read_i32::<LittleEndian>().map(|r| { *v = r; }) }
   fn read_i64le_to(&mut self, v:&mut i64) -> Result<(), std::io::Error> { self.read_i64::<LittleEndian>().map(|r| { *v = r; }) }

   fn read_u16be_to(&mut self, v:&mut u16) -> Result<(), std::io::Error> { self.read_u16::<BigEndian>().map(|r| { *v = r; }) }
   fn read_u32be_to(&mut self, v:&mut u32) -> Result<(), std::io::Error> { self.read_u32::<BigEndian>().map(|r| { *v = r; }) }
   fn read_u64be_to(&mut self, v:&mut u64) -> Result<(), std::io::Error> { self.read_u64::<BigEndian>().map(|r| { *v = r; }) }
   fn read_i16be_to(&mut self, v:&mut i16) -> Result<(), std::io::Error> { self.read_i16::<BigEndian>().map(|r| { *v = r; }) }
   fn read_i32be_to(&mut self, v:&mut i32) -> Result<(), std::io::Error> { self.read_i32::<BigEndian>().map(|r| { *v = r; }) }
   fn read_i64be_to(&mut self, v:&mut i64) -> Result<(), std::io::Error> { self.read_i64::<BigEndian>().map(|r| { *v = r; }) }
}

impl <'a> ReadStream for std::io::Cursor<&'a mut [u8]> { }
impl      ReadStream for std::io::Cursor<Vec<u8>> { }
impl      ReadStream for std::io::Cursor<Box<[u8]>> { }

pub struct SliceReadStream<T: std::borrow::Borrow<[u8]>> {
   inner_:  T,
   cursor_: usize,
}
impl <T: std::borrow::Borrow<[u8]>> SliceReadStream<T> {
   pub fn new(inner:T) -> Self {
      SliceReadStream { inner_:inner, cursor_:0 }
   }
   pub fn as_slice(&self) -> &[u8] {
      self.inner_.borrow()
   }
   pub fn inner(self) -> T {
      self.inner_
   }
   pub fn rewind(&mut self) {
      self.cursor_ = 0;
   }
}
impl <T: std::borrow::Borrow<[u8]>> std::io::Read for SliceReadStream<T> {
   fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
      let (_,o) = self.inner_.borrow().split_at(self.cursor_);
      let rsize = std::cmp::min(o.len(), buf.len());
      (&mut buf[..rsize]).copy_from_slice(&o[..rsize]);
      self.cursor_ += rsize;
      Ok(rsize)
   }
}
impl <T: std::borrow::Borrow<[u8]>> ReadStream for SliceReadStream<T> { }

pub struct FixedReadStream {
   inner_: SliceReadStream<Box<[u8]>>,
}
impl FixedReadStream {
   pub fn new(size:usize) -> Self {
      FixedReadStream { inner_: SliceReadStream::new(vec![0u8; size].into_boxed_slice()) }
   }
   pub fn as_slice(&self) -> &[u8] { self.inner_.as_slice() }
   pub fn rewind(&mut self) { self.inner_.rewind() }
   pub fn inner(self) -> Box<[u8]> { self.inner_.inner() }
}
impl std::io::Read for FixedReadStream {
   fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { self.inner_.read(buf) }
}
impl ReadStream for FixedReadStream { }
