pub trait WriteStream: ::std::io::Write {
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error>;

   #[inline(always)]
   fn write_u8(&mut self, v:u8) -> Result<usize, ::std::io::Error> {
      let buf: &[u8;1] = unsafe { ::std::mem::transmute(&v) };
      self.write_all(buf)?;
      Ok(1)
   }
   #[inline(always)]
   fn write_i8(&mut self, v:i8) -> Result<usize, ::std::io::Error> {
      let buf: &[u8;1] = unsafe { ::std::mem::transmute(&v) };
      self.write_all(buf)?;
      Ok(1)
   }

   
   #[inline(always)]
   fn write_u16le(&mut self, v:u16) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;2] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   fn write_u32le(&mut self, v:u32) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;4] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   fn write_u64le(&mut self, v:u64) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;8] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(8)
   }
   #[inline(always)]
   fn write_i16le(&mut self, v:i16) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;2] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   fn write_i32le(&mut self, v:i32) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;4] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   fn write_i64le(&mut self, v:i64) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_le();
      let buf: &[u8;8] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(8)
   }

   #[inline(always)]
   fn write_u16be(&mut self, v:u16) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;2] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   fn write_u32be(&mut self, v:u32) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;4] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   fn write_u64be(&mut self, v:u64) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;8] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(8)
   }
   #[inline(always)]
   fn write_i16be(&mut self, v:i16) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;2] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(2)
   }
   #[inline(always)]
   fn write_i32be(&mut self, v:i32) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;4] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(4)
   }
   #[inline(always)]
   fn write_i64be(&mut self, v:i64) -> Result<usize, ::std::io::Error> {
      let tmp = v.to_be();
      let buf: &[u8;8] = unsafe { ::std::mem::transmute(&tmp) };
      self.write_all(buf)?;
      Ok(8)
   }
}

impl <'a> WriteStream for ::std::io::Cursor<&'a mut [u8]> {
   #[inline(always)]
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      let pos:u64 = self.position() + (n as u64);
      self.set_position(pos);
      Ok(n)
   }
}
impl WriteStream for ::std::io::Cursor<Vec<u8>> {
   #[inline(always)]
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      let pos:u64 = self.position() + (n as u64);
      self.set_position(pos);
      Ok(n)
   }
}   
impl WriteStream for ::std::io::Cursor<Box<[u8]>> {
   #[inline(always)]
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      let pos:u64 = self.position() + (n as u64);
      self.set_position(pos);
      Ok(n)
   }
}   

pub struct SliceWriteStream<T: ::std::borrow::BorrowMut<[u8]>> {
   inner_:  T,
   cursor_: usize,
}
impl <T: ::std::borrow::BorrowMut<[u8]>> SliceWriteStream<T> {
   pub fn new(inner:T) -> Self {
      SliceWriteStream { inner_:inner, cursor_:0 }
   }
   pub fn get_ref(&self) -> &T { &self.inner_ }
   pub fn get_mut(&mut self) -> &mut T { &mut self.inner_ }
   pub fn into_inner(self) -> T { self.inner_   }
   pub fn cursor(&self) -> usize { self.cursor_ }
   pub fn rewind(&mut self) {
      self.cursor_ = 0;
   }
}
impl <T: ::std::borrow::BorrowMut<[u8]>> ::std::io::Write for SliceWriteStream<T> {
   fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
      let size = buf.len();
      let (_,o) = self.inner_.borrow_mut().split_at_mut(self.cursor_);
      if o.len() < size {
         Err(::std::io::Error::new(::std::io::ErrorKind::WouldBlock, "no capacity to write"))
      } else {
         o[..size].copy_from_slice(buf);
         self.cursor_ += size;
         Ok(size)
      }
   }
   fn flush(&mut self) -> ::std::io::Result<()> { Ok(()) }
}
impl <T: ::std::borrow::BorrowMut<[u8]>> WriteStream for SliceWriteStream<T> {
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      self.cursor_ += n;
      Ok(n)
   }
}


#[derive(Default,Clone)]
pub struct VecWriteStream {
   inner_:  Vec<u8>,
}
impl VecWriteStream {
   pub fn new() -> Self {
      VecWriteStream { inner_:Vec::new() }
   }
   pub fn new_with_vec(inner:Vec<u8>) -> Self {
      VecWriteStream { inner_:inner }
   }
   pub fn get_ref(&self) -> &Vec<u8> { &self.inner_ }
   pub fn get_mut(&mut self) -> &mut Vec<u8> { &mut self.inner_ }
   pub fn into_inner(self) -> Vec<u8> { self.inner_ }
   pub fn rewind(&mut self) { self.inner_.clear(); }
}
impl ::std::io::Write for VecWriteStream {
   fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
      let len0 = self.inner_.len();
      self.inner_.reserve(buf.len());
      unsafe { self.inner_.set_len(len0 + buf.len()); }
      self.inner_.as_mut_slice()[len0..].clone_from_slice(buf);
      Ok(buf.len())
   }
   fn flush(&mut self) -> ::std::io::Result<()> { Ok(()) }
}
impl WriteStream for VecWriteStream {
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      let len0 = self.inner_.len();
      self.inner_.reserve(n);
      unsafe { self.inner_.set_len(len0 + n); }
      Ok(n)
   }
}

pub struct SizeWriteStream {
   size: usize,
}
impl SizeWriteStream {
   pub fn new() -> Self { SizeWriteStream { size: 0 } }
   pub fn rewind(&mut self) { self.size = 0; }
   pub fn size(&self) -> usize { self.size }
}
impl ::std::io::Write for SizeWriteStream {
   #[inline(always)]
   fn write(&mut self, buf:&[u8]) -> ::std::io::Result<usize> {
      self.size += buf.len();
      Ok(buf.len())
   }
   #[inline(always)]
   fn flush(&mut self) -> ::std::io::Result<()> { Ok(()) }
}
impl WriteStream for SizeWriteStream {
   #[inline(always)]
   fn write_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error> {
      self.size += n;
      Ok(n)
   }
}   

#[test]
fn test_cursor_vec() {
   let mut w = ::std::io::Cursor::new(Vec::<u8>::with_capacity(100));
   
   assert_eq!(w.get_ref().len(), 0);
   assert_matches!(w.write_u8(1),  Ok(1));
   assert_matches!(w.write_u8(0), Ok(1));
   assert_eq!(w.get_ref().len(), 2);
   assert_eq!(w.get_ref().as_slice(), &[0x01, 0x00]);
}

#[test]
fn test_slice() {
   {
      let mut w = SliceWriteStream::new([0u8; 32]);
      assert_eq!(w.get_ref().len(), 32);
      assert_matches!(w.write_u8(1),  Ok(1));
      assert_matches!(w.write_u8(0), Ok(1));
      assert_eq!(w.get_ref().len(), 32);
      assert_eq!(&w.get_ref()[..2], &[0x01, 0x00]);
   }
   {
      let mut w = SliceWriteStream::new(vec![0u8; 100]);
      assert_eq!(w.get_ref().len(), 100);
      assert_matches!(w.write_u8(1),  Ok(1));
      assert_matches!(w.write_u8(0), Ok(1));
      assert_eq!(w.get_ref().len(), 100);
      assert_eq!(&w.get_ref()[..2], &[0x01, 0x00]);
   }
}

#[test]
fn test_size() {
   let mut w = SizeWriteStream::new();
   
   assert_eq!(w.size(), 0);
   assert_matches!(w.write_u8(1),  Ok(1));
   assert_matches!(w.write_u8(0), Ok(1));
   assert_eq!(w.size(), 2);
}

#[test]
fn test_u() {
   let mut w = VecWriteStream::default();
   assert_matches!(w.write_u8(1u8), Ok(1));
   assert_matches!(w.write_u16le(1u16), Ok(2));
   assert_matches!(w.write_u16be(1u16), Ok(2));
   assert_matches!(w.write_u32le(1u32), Ok(4));
   assert_matches!(w.write_u32be(1u32), Ok(4));
   assert_matches!(w.write_u64le(1u64), Ok(8));
   assert_matches!(w.write_u64be(1u64), Ok(8));
   assert_eq!(w.get_ref().len(), 29);
   assert_eq!(w.get_ref().as_slice(),
              &[0x01,
                0x01, 0x00,
                0x00, 0x01,
                0x01, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x01,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
              ]);
}

#[test]
fn test_i() {
   let mut w = VecWriteStream::default();
   assert_matches!(w.write_i8(-1i8), Ok(1));
   assert_matches!(w.write_i16le(-2i16), Ok(2));
   assert_matches!(w.write_i16be(-2i16), Ok(2));
   assert_matches!(w.write_i32le(-2i32), Ok(4));
   assert_matches!(w.write_i32be(-2i32), Ok(4));
   assert_matches!(w.write_i64le(-2i64), Ok(8));
   assert_matches!(w.write_i64be(-2i64), Ok(8));
   assert_eq!(w.get_ref().len(), 29);
   assert_eq!(w.get_ref().as_slice(),
              &[0xFF,
                0xFE, 0xFF,
                0xFF, 0xFE,
                0xFE, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFE,
                0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
              ]);
}
