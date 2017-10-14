pub struct ScriptNum(pub i64);

impl ScriptNum {
   pub fn s_encode(v:i64, buf: &mut [u8;9]) -> usize {
      if v == 0 {
         return 0usize;
      }
      let (neg, mut abs) = if v < 0 { (true, -v) } else { (false, v) };

      let mut i:usize = 0;
      while 0 < abs {
         buf[i] = (abs & 0xFF) as u8;
         abs >>= 8;
         i += 1;
      }

      if (buf[i-1] & 0x80) != 0 {
         buf[i] = if neg { 0x80 } else { 0 };
         i += 1;
      } else if neg {
         buf[i-1] |= 0x80;
      }
      i
   }
   pub fn s_decode(buf: &[u8]) -> i64 {
      let mut acc:i64 = 0;
      let len = buf.len();
      for i in 0..len {
         if (i == len-1) && (buf[i] & 0x80 != 0) {
            acc |= ((buf[i] & 0x7F) as i64) << (i*8);
            acc = -acc;
         } else {
            acc |= (buf[i] as i64) << (i*8);
         }
      }
      acc
   }
   
   pub fn encode(&self, buf: &mut [u8;9]) -> usize {
      ScriptNum::s_encode(self.0, buf)
   }
   pub fn decode(&mut self, buf: &[u8]) -> i64 {
      self.0 = ScriptNum::s_decode(buf);
      self.0
   }
}
   
#[test]
fn test_encode_0() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(0, ScriptNum::s_encode(0, &mut buf));
   assert_eq!(buf, [11, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_encode_0x48() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(1, ScriptNum::s_encode(0x48, &mut buf));
   assert_eq!(buf, [0x48, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_neg1() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(1, ScriptNum::s_encode(-1, &mut buf));
   assert_eq!(buf, [0x81, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_0x1234() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::s_encode(0x1234, &mut buf));
   assert_eq!(buf, [0x34, 0x12, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_0x80() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::s_encode(0x80, &mut buf));
   assert_eq!(buf, [0x80, 0x00, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_neg0x1234() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::s_encode(-0x1234, &mut buf));
   assert_eq!(buf, [0x34, 0x92, 33, 44, 55, 66, 77, 88, 99]);
}


#[test]
fn test_decode_0() {
   use super::ScriptNum;

   assert_eq!(0, ScriptNum::s_decode(&[]));
   assert_eq!(0, ScriptNum::s_decode(&[0x80]));
   assert_eq!(0, ScriptNum::s_decode(&[0x00, 0x80]));
}


