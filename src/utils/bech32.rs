use crypto::digest::Digest;
use crate::crypto::digest::DHash256;
use crate::utils::b2h;

def_error! { Bech32Error }
macro_rules! raise_bech32error {
   ($m:expr) => {
      Err(crate::utils::Bech32Error::new($m, 0))?
   }
}

pub static BYTE2CHAR:&[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
pub static CHAR2BYTE:&[u8] = &[
//  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
   99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, //0
   99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, //1
   99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, //2 // space, ...
   15,  0, 10, 17, 21, 20, 26, 30,  7,  5, 99, 99, 99, 99, 99, 99, //3 // digit, ...
//  @   A   B   C   D   E   F   G   H   I   J   K   L   M   N   O
   99, 29, 99, 24, 13, 25,  9,  8, 23, 99, 18, 22, 31, 27, 19, 99, //4
//  P   Q   R   S   T   U   V   W   X   Y   Z
    1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, 99, 99, 99, 99, 99, //5
//  @   a   b   c   d   e   f   g   h   i   j   k   l   m   n   o
   99, 29, 99, 24, 13, 25,  9,  8, 23, 99, 18, 22, 31, 27, 19, 99, //6
//  p   q   r   s   t   u   v   w   x   y   z
    1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, 99, 99, 99, 99, 99, //7
];

#[inline] pub fn is_hrp_char(c:u8) -> bool { 33 <= c && c <= 126 }
#[inline] pub fn is_dp_char(c:u8) -> bool { c < 0x80 && CHAR2BYTE[c as usize] != 99 }
fn char2byte(c:u8, i:usize) -> crate::Result<u8> {
   if CHAR2BYTE.len() <= (c as usize) {
      raise_bech32error!(format!("not a bech32 char at {}: 0x{:x}", i, c));
   }
   match CHAR2BYTE[c as usize] {
      x if x <= 32 => Ok(x),
      _ => raise_bech32error!(format!("not a bech32 char at {}: 0x{:x}", i, c)),
   }
}

fn u5_to_u8<T: AsRef<[u8]>>(u5s_:T) -> Vec<u8> {
   let u5s:&[u8] = u5s_.as_ref();
   let mut ret:Vec<u8> = Vec::with_capacity(u5s.len());
   u5s.iter().fold((0u8, 0usize), |(carry, bits), &u5| {
      match bits {
         0..=2 => {
            (carry | u5 << (3-bits), bits + 5)
         },
         3 => {
            ret.push(carry | u5);
            ( 0, 0 )
         },
         4..=7 => { // u8 << 8 raise panic
            ret.push(carry | (u5 >> (bits-3)) );
            ( u5 << (11-bits), bits-3 )
         },
         _ => {
            panic!(format!("invalid bits: {}", bits));
         },
      }
   });
   ret
}

fn u8_to_u5<T:AsRef<[u8]>>(u8s_: T) -> Vec<u8> {
   let u8s = u8s_.as_ref();
   let mut ret:Vec::<u8> = Vec::with_capacity(u8s.len()*2);
   let (c,b) = u8s.iter().fold((0u8, 0usize), |(carry,bits), &oct| {
      ret.push(carry | (oct >> (3+bits)));
      match bits {
         0..=1 => {
            ( ((oct << (2-bits))) & 0x1F, 3+bits )
         },
         2..=4 => {
            ret.push((oct >> (bits-2)) & 0x1F);
            ( ((oct << (7-bits))) & 0x1F, bits-2 )
         }
         _ => {
            panic!(format!("invalid bits: {}", bits));
         }
      }
   });
   if b != 0 {
      ret.push(c);
   }
   ret
}
     

#[derive(Debug)]
pub struct Bech32 {
   pub format: String,
   pub hrp:  String,
   pub data: Vec<u8>,
   pub checksum: [u8;6]
}

fn check_hrp(hrp:&[u8]) -> crate::Result<()> {
   if hrp.len() < 1 {
      raise_bech32error!(format!("hrp is too short: {}", hrp.len()));
   }
   if 83 < hrp.len() {
      raise_bech32error!(format!("hrp is too long: {}", hrp.len()));
   }
   if let Some((i, &c)) = hrp.iter().enumerate().find(|(_i,&c)| { !is_hrp_char(c) }) {
      raise_bech32error!(format!("not a hrp char: {} at {}", c, i));
   }
   Ok(())
}
fn check_dp(dp:&[u8]) -> crate::Result<()> {
   if dp.len() < 6 {
      raise_bech32error!(format!("dp is too short: {}", dp.len()));
   }
   if 88 < dp.len() {
      raise_bech32error!(format!("dp is too long: {}", dp.len()));
   }
   if let Some((i, &c)) = dp.iter().enumerate().find(|(_i,&c)| { !is_dp_char(c) }) {
      raise_bech32error!(format!("not a dp char: {} at {}", c, i));
   }
   Ok(())
}
fn check_data(data:&[u8]) -> crate::Result<()> {
   if 52 < data.len() {
      raise_bech32error!(format!("raw data is too long: {}bytes > 410bits", data.len()));
   }
   if 52 == data.len() && (data[51] & 0x3F) != 0 {
      raise_bech32error!(format!("raw data is too long: 51byte + 0x{:x} > 410bits", data[51]));
   }
   Ok(())
}

impl Bech32 {
   pub fn from_data<T:AsRef<[u8]>>(hrp: &str, u8s_: T) -> crate::Result<Self> {
      let u8s = u8s_.as_ref();
      let hrp_bytes = hrp.as_bytes();
      let _ = check_hrp(hrp_bytes)?;
      let _ = check_data(u8s)?;
      
      println!("{}", format!("from_data: {:?}", u8s));
      let u5s = u8_to_u5(u8s);
      println!("{}", format!("  u5s={:?}", u5s));
      let checksum = create_checksum(hrp_bytes, u5s.as_slice());
      println!("{}", format!("  sum={:?}", checksum));
      let data_format = u5s.iter().chain(checksum.iter()).map(|&b| {
         BYTE2CHAR[b as usize]
      }).collect();

      let mut format = String::from(hrp);
      format.push('1');
      format.push_str(String::from_utf8(data_format).unwrap().as_str());
      
      Ok(Bech32 {
         format: format,
         hrp: String::from(hrp),
         data: Vec::from(u8s),
         checksum: checksum,
      })
   }

   pub fn from_str<T:AsRef<str>>(format_: T) -> crate::Result<Self> {
      let format = format_.as_ref();
      let format_bytes = format.as_bytes();
      let sep_index = match format_bytes.iter().rposition(|&b| b == 0x31) { // '1'
         Some(i) => i,
         None => { raise_bech32error!("separator not found"); 0 },
      };
      let hrp = &format_bytes[0..sep_index];
      let _ = check_hrp(hrp)?;

      let dp = &format_bytes[sep_index+1..];
      let _ = check_dp(dp)?;
   
      let u5s_with_checksum = {
         let mut ret = Vec::<u8>::with_capacity(dp.len());
         let mut lower = false;
         let mut upper = false;
         for (i,b) in dp.iter().enumerate() {
            ret.push(char2byte(*b, i)?);
            match *b {
               0x61..=0x7a => { // 'a'..='z'
                  if upper { raise_bech32error!(format!("lower char is found at {}", i)); }
                  lower = true;
               }
               0x41..=0x5a => { //'A'..='Z'
                  if lower { raise_bech32error!(format!("upper char is found at {}", i)); }
                  upper = true;
               }
               _ => (),
            }
         }
         ret
      };
      if !verify_checksum(hrp, u5s_with_checksum.as_slice()) {
         raise_bech32error!("checksum error");
      }
      let (u5s, checksum) = u5s_with_checksum.split_at(u5s_with_checksum.len()-6);
      let u8s = u5_to_u8(u5s);
      Ok(Bech32 {
         format: String::from(format),
         hrp: String::from_utf8(hrp.to_vec()).unwrap(),
         data: u8s,
         checksum: [checksum[0], checksum[1], checksum[2], checksum[3], checksum[4], checksum[5]],
      })
   }
}

pub static GEN:[u32;5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
fn polymod(values:&[u8]) -> u32 {
   //println!("polymod(0x{})", b2h(values));
   values.iter().fold(1u32, |acc, v| {
      //println!("values: v=0x{:x}, acc=0x{:x}", v, acc);
      let b = acc >> 25;
      let mut chk = (acc & 0x1ffffff) << 5 ^ (*v as u32);
      //println!("  b=0x{:x}", b);
      //println!("  c=0x{:x}", chk);
      GEN.iter().enumerate().for_each(|(i,g)| {
         if ((b >> i) & 1) != 0 {
            chk ^= g;
         }
         //println!("    gens: i={}, g=0x{:x} -> chk=0x{:x}", i, g, chk);
      });
      chk
   })
}
macro_rules! tolower {
   ($x:expr) => { if 0x41 <= $x && $x <= 0x5A { $x + 0x20 } else { $x } }
}
fn concat_for_checksum(hrp:&[u8], u5s:&[u8]) -> Vec<u8> {
   let mut ret = Vec::<u8>::with_capacity(hrp.len()*2 + 1 + u5s.len());
   hrp.iter().for_each(|b| { ret.push(tolower!(*b) >> 5) });
   ret.push(0u8);
   hrp.iter().for_each(|b| { ret.push(tolower!(*b) & 0x1F) });
   ret.extend_from_slice(u5s);
   ret
}
   
fn create_checksum(hrp:&[u8], u5s:&[u8]) -> [u8;6] {
   let mut tmp = concat_for_checksum(hrp, u5s);
   tmp.extend_from_slice(&[0u8; 6]);
   let sum = polymod(tmp.as_slice()) ^ 1;
   let mut ret = [0u8; 6];
   for i in 0..6 {
      ret[5-i] = ((sum >> (i*5)) & 0x1F) as u8;
   }
   ret
}
fn verify_checksum(hrp:&[u8], u5s:&[u8]) -> bool {
   let tmp = concat_for_checksum(hrp, u5s);
   polymod(tmp.as_slice()) == 1u32
}

#[test]
fn check_table() {
   for (i,c) in BYTE2CHAR.iter().enumerate() {
      assert_eq!(i, CHAR2BYTE[*c as usize] as usize);
   }
}

mod tests {
   use super::{Bech32, u8_to_u5, u5_to_u8};
   use hex_literal::hex;

   // 00010001 00100010 00110011 01000100 01010101 01100110
   // 00010 00100 10001 00011 00110 10001 00010 10101 01100 110..
   // 2     4     17    3     6     17    2     21    12    24
   #[allow(dead_code)]
   const TESTSET1:&[(&[u8], &[u8])] = &[
      ( &[2u8, 4],                              &[0x11u8] ),
      ( &[2u8, 4, 17, 0],                       &[0x11u8, 0x22] ),
      ( &[2u8, 4, 17, 3, 6],                    &[0x11u8, 0x22, 0x33] ),
      ( &[2u8, 4, 17, 3, 6, 17, 0],             &[0x11u8, 0x22, 0x33, 0x44] ),
      ( &[2u8, 4, 17, 3, 6, 17, 2, 21],         &[0x11u8, 0x22, 0x33, 0x44, 0x55] ),
      ( &[2u8, 4, 17, 3, 6, 17, 2, 21, 12, 24], &[0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66] ),
   ];

   #[test]
   fn test_u8_to_u5() {
      assert_eq!(TESTSET1[0].0, u8_to_u5(TESTSET1[0].1).as_slice());
      assert_eq!(TESTSET1[1].0, u8_to_u5(TESTSET1[1].1).as_slice());
      assert_eq!(TESTSET1[2].0, u8_to_u5(TESTSET1[2].1).as_slice());
      assert_eq!(TESTSET1[3].0, u8_to_u5(TESTSET1[3].1).as_slice());
      assert_eq!(TESTSET1[4].0, u8_to_u5(TESTSET1[4].1).as_slice());
      assert_eq!(TESTSET1[5].0, u8_to_u5(TESTSET1[5].1).as_slice());
   }

   #[test]
   fn test_u5_to_u8() {
      assert_eq!(TESTSET1[0].1, u5_to_u8(TESTSET1[0].0).as_slice());
      assert_eq!(TESTSET1[1].1, u5_to_u8(TESTSET1[1].0).as_slice());
      assert_eq!(TESTSET1[2].1, u5_to_u8(TESTSET1[2].0).as_slice());
      assert_eq!(TESTSET1[3].1, u5_to_u8(TESTSET1[3].0).as_slice());
      assert_eq!(TESTSET1[4].1, u5_to_u8(TESTSET1[4].0).as_slice());
      assert_eq!(TESTSET1[5].1, u5_to_u8(TESTSET1[5].0).as_slice());
   }

   #[test]
   fn test_decode_bech32() {
      assert_matches!(Bech32::from_str("A12UEL5L"), Ok(_)); // data=null, checksum=2UEL5L
      assert_matches!(Bech32::from_str("a12uel5l"), Ok(_));
      assert_matches!(Bech32::from_str("A12UEL5l"), Err(_));
      assert_matches!(Bech32::from_str("a12uel5L"), Err(_));
      let b = Bech32::from_str("A12UEL5L").unwrap();
      assert_eq!("A", b.hrp);
      assert_eq!(0, b.data.len());
      assert_eq!([10,28,25,31,20,31], b.checksum);

      
      assert_matches!(Bech32::from_str("an83characterlonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1tt5tgs"), Ok(_));
      assert_matches!(Bech32::from_str("abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw"), Ok(_));
      assert_matches!(Bech32::from_str("11qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqc8247j"), Ok(_));
      assert_matches!(Bech32::from_str("split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w"), Ok(_));
      let b = Bech32::from_str("split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w").unwrap();
      assert_eq!("split", b.hrp);
      assert_eq!(b.data, [197u8, 243, 139, 112, 48, 95, 81, 155, 246, 109, 133, 251, 108, 240, 48, 88, 243, 221, 228, 99, 236, 215, 145, 143, 45, 199, 67, 145, 143, 45]);
      assert_eq!(b.checksum, [10,4,5,25,17,14]);
      
      assert_matches!(Bech32::from_str("\x201nwldjs"), Err(_)); // hrp char out of range
      assert_matches!(Bech32::from_str("\x71axkwrx"), Err(_)); // hrp char out of range
      assert_matches!(Bech32::from_str("\u{80}1eym55h"), Err(_)); // hrp char out of range
      assert_matches!(Bech32::from_str("an84characterslonghumanreadablepartthatcontainsthenumber1andtheexcludedcharactersbio1569pvx"), Err(_)); // too long
      assert_matches!(Bech32::from_str("pzry9x0s0muk"), Err(_)); //no separator
      assert_matches!(Bech32::from_str("1pzry9x0s0muk"), Err(_)); //empty hrp
      assert_matches!(Bech32::from_str("x1b4n0q5v"), Err(_)); //invalid data char
      assert_matches!(Bech32::from_str("li1dgmt3"), Err(_)); //too short checksum
      assert_matches!(Bech32::from_str("de1lg7wt\u{FF}"), Err(_)); //invalid char in checksum
      assert_matches!(Bech32::from_str("A1G75GD8"), Err(_)); //checksum calculated with uppercase from HRP
      assert_matches!(Bech32::from_str("10a06t8"), Err(_)); //empty hrp
      assert_matches!(Bech32::from_str("1qzzfhee"), Err(_)); //empty hrp
   }
   
   #[test]
   fn test_encode_bech32() {
      use hex_literal::hex;

      let b = Bech32::from_data("split", [197u8, 243, 139, 112, 48, 95, 81, 155, 246, 109, 133, 251, 108, 240, 48, 88, 243, 221, 228, 99, 236, 215, 145, 143, 45, 199, 67, 145, 143, 45]).unwrap();
      assert_eq!("split", b.hrp);
      assert_eq!(b.format, "split1checkupstagehandshakeupstreamerranterredcaperred2y9e3w");
      assert_eq!(b.checksum, [10,4,5,25,17,14]);
   }
}
