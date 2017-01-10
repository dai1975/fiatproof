use std;

def_error! { ParseUInt256Error }

#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct UInt256 {
   pub data: [u8;32],
}

pub const ZERO:UInt256 = UInt256 { data: [0u8;32] };

impl std::hash::Hash for UInt256 {
   fn hash<H:std::hash::Hasher>(&self, state:&mut H) {
      state.write(&self.data[..]);
   }
}

impl UInt256 {
   pub fn new(d: &[u8;32]) -> UInt256 {
      let mut v = UInt256 { data: [0u8;32] };
      v.data.clone_from_slice(d);
      v
   }
   pub fn as_slice(&self) -> &[u8] {
      &self.data[..]
   }
}

use ::{ToHex,FromHex};
impl ToHex for UInt256 {
   fn to_hex(&self) -> ::Result<String> {
      let mut rev = [0u8;32];
      for i in 0..32 {
         rev[i] = self.data[31-i];
      }
      rev.to_hex()
   }
}
impl FromHex for UInt256 {
   fn from_hex<S:AsRef<str>>(&mut self, s:S) -> ::Result<()> {
      let s:&str = s.as_ref();
      if s.len() != 64 { try!(Err(ParseUInt256Error::new(format!("string is too short: {}", self)))); }
      let mut tmp = UInt256::default();
      let _ = try!(tmp.data.from_hex(s));
      for i in 0..32 {
         self.data[i] = tmp.data[31-i];
      }
      Ok(())
   }
}

impl std::ops::Index<usize> for UInt256 {
   type Output = u8;
   fn index(&self, i:usize) -> &u8 {
      &self.data[i]
   }
}
impl std::ops::IndexMut<usize> for UInt256 {
   fn index_mut(&mut self, i:usize) -> &mut u8 {
      &mut self.data[i]
   }
}
impl std::fmt::Display for UInt256 {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self.to_hex() {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(e) => f.write_fmt(format_args!("{:?}", e)),
      }
   }
}

#[test]
fn test_str() {
   use ::WithHex;
   let s = "00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81";
   let uint256 = UInt256::with_hex(s).unwrap();

   let expect:[u8;32] = [
      0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
      0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
   ];
   assert_eq!(expect, uint256.data);

   let t = format!("{}", uint256);
   assert_eq!(s, t);
}
