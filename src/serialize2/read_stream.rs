pub trait ReadStream: ::std::io::Read {
   fn read_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error>;
   
   #[inline(always)]
   fn read_u8(&mut self, v:&mut u8) -> Result<usize, ::std::io::Error> {
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(v) };
      try!(self.read_exact(buf));
      Ok(1)
   }
   #[inline(always)]
   fn read_i8(&mut self, v:&mut i8) -> Result<usize, ::std::io::Error> {
      let mut tmp:i8 = 0;
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = tmp;
      Ok(1)
   }
   
   #[inline(always)]
   fn read_u16le(&mut self, v:&mut u16) -> Result<usize, ::std::io::Error> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_u32le(&mut self, v:&mut u32) -> Result<usize, ::std::io::Error> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_u64le(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   fn read_u16be(&mut self, v:&mut u16) -> Result<usize, ::std::io::Error> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_u32be(&mut self, v:&mut u32) -> Result<usize, ::std::io::Error> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_u64be(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u64::from_be(tmp);
      Ok(8)
   }

   #[inline(always)]
   fn read_i16le(&mut self, v:&mut i16) -> Result<usize, ::std::io::Error> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_i32le(&mut self, v:&mut i32) -> Result<usize, ::std::io::Error> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_i64le(&mut self, v:&mut i64) -> Result<usize, ::std::io::Error> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   fn read_i16be(&mut self, v:&mut i16) -> Result<usize, ::std::io::Error> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_i32be(&mut self, v:&mut i32) -> Result<usize, ::std::io::Error> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_i64be(&mut self, v:&mut i64) -> Result<usize, ::std::io::Error> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i64::from_be(tmp);
      Ok(8)
   }
}

impl <'a> ReadStream for ::std::io::Cursor<&'a [u8]> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}
impl ReadStream for ::std::io::Cursor<Vec<u8>> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}
impl ReadStream for ::std::io::Cursor<Box<[u8]>> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}

pub struct SliceReadStream<T: ::std::borrow::Borrow<[u8]>> {
   inner_:  T,
   cursor_: usize,
}
impl <T: ::std::borrow::Borrow<[u8]>> SliceReadStream<T> {
   pub fn new(inner:T) -> Self {
      SliceReadStream { inner_:inner, cursor_:0 }
   }
   pub fn get_ref(&self) -> &T { &self.inner_ }
   pub fn get_mut(&mut self) -> &mut T { &mut self.inner_ }
   pub fn into_inner(self) -> T { self.inner_ }
   pub fn cursor(&self) -> usize { self.cursor_ }
   pub fn rewind(&mut self) {
      self.cursor_ = 0;
   }
}
impl <T: ::std::borrow::Borrow<[u8]>> ::std::io::Read for SliceReadStream<T> {
   fn read(&mut self, buf: &mut [u8]) -> Result<usize, ::std::io::Error> {
      let (_,o) = self.inner_.borrow().split_at(self.cursor_);
      let rsize = ::std::cmp::min(o.len(), buf.len());
      (&mut buf[..rsize]).copy_from_slice(&o[..rsize]);
      self.cursor_ += rsize;
      Ok(rsize)
   }
}
impl <T: ::std::borrow::Borrow<[u8]>> ReadStream for SliceReadStream<T> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      self.cursor_ += s;
      Ok(s)
   }
}

pub struct SizeReadStream {
   pub size:usize,
}
impl SizeReadStream {
   pub fn new() -> Self { SizeReadStream { size: 0 } }
   pub fn rewind(&mut self) { self.size = 0; }
   pub fn size(&self) -> usize { self.size }
}
impl ::std::io::Read for SizeReadStream {
   #[inline(always)]
   fn read(&mut self, buf:&mut [u8]) -> ::std::io::Result<usize> {
      self.size += buf.len();
      Ok(buf.len())
   }
}
impl ReadStream for SizeReadStream {
   #[inline(always)]
   fn read_skip(&mut self, n:usize) -> ::std::io::Result<usize> {
      self.size += n;
      Ok(n)
   }
}


/*
pub trait ReadStream<'r>: ::std::io::Read {
   fn read_skip(&mut self, n:usize) -> Result<usize, ::std::io::Error>;
   
   #[inline(always)]
   fn read_u8(&mut self, v:&mut u8) -> Result<usize, ::std::io::Error> {
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(v) };
      try!(self.read_exact(buf));
      Ok(1)
   }
   #[inline(always)]
   fn read_i8(&mut self, v:&mut i8) -> Result<usize, ::std::io::Error> {
      let mut tmp:i8 = 0;
      let buf: &mut [u8;1] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = tmp;
      Ok(1)
   }
   
   #[inline(always)]
   fn read_u16le(&mut self, v:&mut u16) -> Result<usize, ::std::io::Error> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_u32le(&mut self, v:&mut u32) -> Result<usize, ::std::io::Error> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_u64le(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   fn read_u16be(&mut self, v:&mut u16) -> Result<usize, ::std::io::Error> {
      let mut tmp:u16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_u32be(&mut self, v:&mut u32) -> Result<usize, ::std::io::Error> {
      let mut tmp:u32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_u64be(&mut self, v:&mut u64) -> Result<usize, ::std::io::Error> {
      let mut tmp:u64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = u64::from_be(tmp);
      Ok(8)
   }

   #[inline(always)]
   fn read_i16le(&mut self, v:&mut i16) -> Result<usize, ::std::io::Error> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i16::from_le(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_i32le(&mut self, v:&mut i32) -> Result<usize, ::std::io::Error> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i32::from_le(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_i64le(&mut self, v:&mut i64) -> Result<usize, ::std::io::Error> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i64::from_le(tmp);
      Ok(8)
   }
   #[inline(always)]
   fn read_i16be(&mut self, v:&mut i16) -> Result<usize, ::std::io::Error> {
      let mut tmp:i16 = 0;
      let buf: &mut [u8;2] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i16::from_be(tmp);
      Ok(2)
   }
   #[inline(always)]
   fn read_i32be(&mut self, v:&mut i32) -> Result<usize, ::std::io::Error> {
      let mut tmp:i32 = 0;
      let buf: &mut [u8;4] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i32::from_be(tmp);
      Ok(4)
   }
   #[inline(always)]
   fn read_i64be(&mut self, v:&mut i64) -> Result<usize, ::std::io::Error> {
      let mut tmp:i64 = 0;
      let buf: &mut [u8;8] = unsafe { ::std::mem::transmute(&mut tmp) };
      try!(self.read_exact(buf));
      *v = i64::from_be(tmp);
      Ok(8)
   }
}

impl <'a, 'r> ReadStream<'r> for ::std::io::Cursor<&'a [u8]> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}
impl <'r> ReadStream<'r> for ::std::io::Cursor<Vec<u8>> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}
impl <'r> ReadStream<'r> for ::std::io::Cursor<Box<[u8]>> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      let pos = self.position();
      self.set_position(pos + (s as u64));
      Ok(s)
   }
}

pub struct SliceReadStream<T: ::std::borrow::Borrow<[u8]>> {
   inner_:  T,
   cursor_: usize,
}
impl <T: ::std::borrow::Borrow<[u8]>> SliceReadStream<T> {
   pub fn new(inner:T) -> Self {
      SliceReadStream { inner_:inner, cursor_:0 }
   }
   pub fn get_ref(&self) -> &T { &self.inner_ }
   pub fn get_mut(&mut self) -> &mut T { &mut self.inner_ }
   pub fn into_inner(self) -> T { self.inner_ }
   pub fn cursor(&self) -> usize { self.cursor_ }
   pub fn rewind(&mut self) {
      self.cursor_ = 0;
   }
}
impl <T: ::std::borrow::Borrow<[u8]>> ::std::io::Read for SliceReadStream<T> {
   fn read(&mut self, buf: &mut [u8]) -> Result<usize, ::std::io::Error> {
      let (_,o) = self.inner_.borrow().split_at(self.cursor_);
      let rsize = ::std::cmp::min(o.len(), buf.len());
      (&mut buf[..rsize]).copy_from_slice(&o[..rsize]);
      self.cursor_ += rsize;
      Ok(rsize)
   }
}
impl <'r, T: ::std::borrow::Borrow<[u8]>> ReadStream<'r> for SliceReadStream<T> {
   fn read_skip(&mut self, s:usize) -> Result<usize, ::std::io::Error> {
      self.cursor_ += s;
      Ok(s)
   }
}

pub struct SizeReadStream {
   pub size:usize,
}
impl SizeReadStream {
   pub fn new() -> Self { SizeReadStream { size: 0 } }
   pub fn rewind(&mut self) { self.size = 0; }
   pub fn size(&self) -> usize { self.size }
}
impl ::std::io::Read for SizeReadStream {
   #[inline(always)]
   fn read(&mut self, buf:&mut [u8]) -> ::std::io::Result<usize> {
      self.size += buf.len();
      Ok(buf.len())
   }
}
impl <'r> ReadStream<'r> for SizeReadStream {
   #[inline(always)]
   fn read_skip(&mut self, n:usize) -> ::std::io::Result<usize> {
      self.size += n;
      Ok(n)
   }
}
*/

#[test]
fn test_cursor_vec() {
   let mut r = ::std::io::Cursor::new(vec![1u8, 0u8]);

   let mut v = 0u8;
   assert_matches!(r.read_u8(&mut v),  Ok(1));
   assert_eq!(v, 1);
   assert_matches!(r.read_u8(&mut v), Ok(1));
   assert_eq!(v, 0);
}

#[test]
fn test_slice() {
   {
      let mut r = SliceReadStream::new([1u8, 0u8]);
      let mut v = 0u8;
      assert_matches!(r.read_u8(&mut v),  Ok(1));
      assert_eq!(v, 1);
      assert_matches!(r.read_u8(&mut v), Ok(1));
      assert_eq!(v, 0);
   }
   {
      let mut r = SliceReadStream::new(vec![1u8, 0u8]);
      let mut v = 0u8;
      assert_matches!(r.read_u8(&mut v),  Ok(1));
      assert_eq!(v, 1);
      assert_matches!(r.read_u8(&mut v), Ok(1));
      assert_eq!(v, 0);
   }
}

#[test]
fn test_size() {
   let mut r = SizeReadStream::new();

   let mut v = 0u8;
   assert_eq!(r.size(), 0);
   assert_matches!(r.read_u8(&mut v),  Ok(1));
   assert_matches!(r.read_u8(&mut v), Ok(1));
   assert_eq!(r.size(), 2);
}

#[test]
fn test_u() {
   let a = [0x01,
            0x01, 0x00,
            0x00, 0x01,
            0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
   ];

   let mut r = SliceReadStream::new(a);
   let mut u8  = 0u8;
   let mut u16 = 0u16;
   let mut u32 = 0u32;
   let mut u64 = 0u64;
   assert_matches!(r.read_u8(&mut u8), Ok(1));
   assert_eq!(u8, 1);
   assert_matches!(r.read_u16le(&mut u16), Ok(2));
   assert_eq!(u16, 1);
   assert_matches!(r.read_u16be(&mut u16), Ok(2));
   assert_eq!(u16, 1);
   assert_matches!(r.read_u32le(&mut u32), Ok(4));
   assert_eq!(u32, 1);
   assert_matches!(r.read_u32be(&mut u32), Ok(4));
   assert_eq!(u32, 1);
   assert_matches!(r.read_u64le(&mut u64), Ok(8));
   assert_eq!(u64, 1);
   assert_matches!(r.read_u64be(&mut u64), Ok(8));
   assert_eq!(u64, 1);
}

#[test]
fn test_i() {
   let a = [0xFF,
            0xFE, 0xFF,
            0xFF, 0xFE,
            0xFE, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFE,
            0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
   ];

   let mut r = SliceReadStream::new(a);
   let mut i8  = 0i8;
   let mut i16 = 0i16;
   let mut i32 = 0i32;
   let mut i64 = 0i64;
   assert_matches!(r.read_i8(&mut i8), Ok(1));
   assert_eq!(i8, -1);
   assert_matches!(r.read_i16le(&mut i16), Ok(2));
   assert_eq!(i16, -2);
   assert_matches!(r.read_i16be(&mut i16), Ok(2));
   assert_eq!(i16, -2);
   assert_matches!(r.read_i32le(&mut i32), Ok(4));
   assert_eq!(i32, -2);
   assert_matches!(r.read_i32be(&mut i32), Ok(4));
   assert_eq!(i32, -2);
   assert_matches!(r.read_i64le(&mut i64), Ok(8));
   assert_eq!(i64, -2);
   assert_matches!(r.read_i64be(&mut i64), Ok(8));
   assert_eq!(i64, -2);
}
