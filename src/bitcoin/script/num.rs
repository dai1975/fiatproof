pub struct ScriptNum;

/*
 i8: [-128, 127].
 127  -> 0x7F
 -127 -> 0x7F | 0x80 = 0xFF
 -128 -> 0x80 | 0x80<<1 = 0x8080
*/
impl ScriptNum {
   pub fn from_bool(v:bool) -> i64 { if v { 1 } else { 0 } }
      
   pub fn serialize(v:i64, buf: &mut [u8;9]) -> usize {
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
   
   pub fn deserialize(buf: &[u8]) -> i64 { //todo: return bignum
      let len = buf.len();
      if len == 0 {
         0
      } else {
         let mut acc:i64 = 0;
         for i in 0..(len-1) {
            acc |= (buf[i] as i64) << (i*8);
         }
         let i = len-1;
         if (buf[i] & 0x80) == 0 {
            acc |= (buf[i] as i64) << (i*8);
         } else {
            acc |= ((buf[i] & 0x7F) as i64) << (i*8);
            acc = -acc;
         }
         acc
      }
   }
   pub fn check_minimal(buf: &[u8]) -> crate::Result<()> {
      let len = buf.len();
      if 0 < len {
         if buf[len-1] & 0x7f == 0 {
            if (len <= 1) || ((buf[len-2] & 0x80) == 0) {
               raise_script_error!("non-minimal serialized script number");
            }
         }
      }
      Ok(())
   }
   pub fn deserialize_i64(buf: &[u8], require_minimal:bool, max_len:usize) -> crate::Result<i64> {
      let len = buf.len();
      if max_len < len {
         raise_script_error!("script number overflow");
      }
      if require_minimal && 0 < len {
         ScriptNum::check_minimal(buf)?;
      }
      
      if len == 0 {
         Ok(0)
      } else if 9 < len {
         raise_script_error!("data is too long")
      } else if len == 9 {
         if buf == [0x80u8, 0x80u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8] {
            Ok(std::i64::MIN)
         } else {
            raise_script_error!("data is too long")
         }
      } else {
         let mut acc:i64 = 0;
         for i in 0..(len-1) {
            acc |= (buf[i] as i64) << (i*8);
         }
         let i = len-1;
         if (buf[i] & 0x80) == 0 {
            acc |= (buf[i] as i64) << (i*8);
         } else {
            acc |= ((buf[i] & 0x7F) as i64) << (i*8);
            acc = -acc;
         }
         Ok(acc)
      }
   }
   /*
   bitcoin/src/script.h/CScriptNum だと、
   - 二項演算はオーバーフロー無視、代入演算はオーバーフローチェックあり。
   - 代入演算はオーバーフローチェックあり
   コメントに書かれている暗黙の仕様的には、
   - 演算時は 4byte 値のみ受けつけ([2^32+1 ... 2^32-1])
   - 演算結果はバイト列で格納され、4byteオーバーフローは気にしない
   - オーバーフローしたl演算結果を 4byte制限の数値展開する時にエラーになる
    */
}
   
#[test]
fn test_serialize_0() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(0, ScriptNum::serialize(0, &mut buf));
   assert_eq!(buf, [11, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_serialize_0x48() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(1, ScriptNum::serialize(0x48, &mut buf));
   assert_eq!(buf, [0x48, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_neg1() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(1, ScriptNum::serialize(-1, &mut buf));
   assert_eq!(buf, [0x81, 22, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_0x1234() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::serialize(0x1234, &mut buf));
   assert_eq!(buf, [0x34, 0x12, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_0x80() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::serialize(0x80, &mut buf));
   assert_eq!(buf, [0x80, 0x00, 33, 44, 55, 66, 77, 88, 99]);
}

#[test]
fn test_neg0x1234() {
   use super::ScriptNum;
   let mut buf:[u8;9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
   assert_eq!(2, ScriptNum::serialize(-0x1234, &mut buf));
   assert_eq!(buf, [0x34, 0x92, 33, 44, 55, 66, 77, 88, 99]);
}


#[test]
fn test_deserialize_0() {
   use super::ScriptNum;

   assert_eq!(0, ScriptNum::deserialize(&[]));
   assert_eq!(0, ScriptNum::deserialize(&[0x80]));
   assert_eq!(0, ScriptNum::deserialize(&[0x00, 0x80]));
}


