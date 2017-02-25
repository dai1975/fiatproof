use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee, BitcoinCodec};

pub struct ScriptNum(pub i64);

stack 上のバイト列の展開、符号化に使うだけで、一般的な bitcoin のシリアライズには使われない。
   なので、codec 系を実装する必要はないな。
   単純に bytes との変換関数だけあればよい。

   さらに、ここに独立させるよりも、
   interpreter.rs の中に private で押し来んじゃった方がいいんじゃないか。
   
impl ScriptNum {
   pub fn as_bool(&self) -> bool {
      self.0 != 0
   }
   pub fn as_bytes(&self) -> ::Result<Vec<u8>> {
      BitcoinCodec::encode(self, (), "net")
   }
   pub fn bytes_as_bool(bytes:&[u8]) -> bool {
      for (i,&b) in bytes.iter().enumerate() {
         if b != 0x00 {
            if b == 0x80 && i == bytes.len()-1 {
               return false;
            } else {
               return true;
            }
         }
      }
      return false;
   }
   pub fn bytes_as_num(&self, bytes:&[u8], len:usize) -> ::Result<i64> {
      let mut n = ScriptNum(0);
      try!(BitcoinCodec::decode(&mut n, bytes, len, "net"));
      Ok(n.0)
   }
}

impl Encodee for ScriptNum {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      if self.0 == 0 {
         return Ok(0usize)
      }

      let (neg, mut abs) = if self.0 < 0 { (true, -self.0) } else { (false, self.0) };

      let mut out = [0u8; 9];
      let mut i:usize = 0;
      while 0 < abs {
         out[i] = (abs & 0xFF) as u8;
         abs >>= 8;
         i += 1;
      }

      if (out[i-1] & 0x80) != 0 {
         out[i] = if neg { 0x80 } else { 0 };
         i += 1;
      } else if neg {
         out[i-1] |= 0x80;
      }
      e.encode_array_u8(&out[..i])
   }
}

impl Decodee for ScriptNum {
   type P = usize;
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, len:BP) -> ::Result<usize> {
      let mut acc:i64 = 0;
      let mut v:u8 = 0;
      let len = *len.borrow();
      for i in 0..len {
         try!(d.decode_u8(&mut v));
         if (i == len-1) && (v & 0x80 != 0) {
            acc |= ((v & 0x7F) as i64) << (i*8);
            acc = -acc;
         } else {
            acc |= (v as i64) << (i*8);
         }
      }
      self.0 = acc;
      Ok(len)
   }
}
   
#[cfg(test)]
mod tests {
   fn test(val:i64, bytes:&[u8]) {
      use super::ScriptNum;
      {
         use ::codec::{BitcoinEncodeStream, Encodee, VecWriteStream, Media};
         let mut e = BitcoinEncodeStream::new(VecWriteStream::default(), Media::default().set_net());
         let v = ScriptNum(val);
         assert_eq!(v.encode(&mut e, ()).unwrap(), bytes.len());
         assert_eq!(&e.w.get_ref()[..bytes.len()], bytes);
      }
      {
         use ::codec::{BitcoinDecodeStream, SliceReadStream, Decodee, Media};
         let mut d = BitcoinDecodeStream::new(SliceReadStream::new(bytes), Media::default().set_net());
         let mut v = ScriptNum(0);
         assert_eq!(v.decode(&mut d, bytes.len()).unwrap(), bytes.len());
         assert_eq!(v.0, val);
      }
   }

   #[test]
   fn test_0() {
      test(0, &[]);
   }
   #[test]
   fn test_0x48() {
      test(0x48, &[0x48]);
   }
   #[test]
   fn test_neg1() {
      test(-1, &[0x81]);
   }
   #[test]
   fn test_0x1234() {
      test(0x1234, &[0x34, 0x12]);
   }
   #[test]
   fn test_0x80() {
      test(0x80, &[0x80, 0x00]);
   }

}
