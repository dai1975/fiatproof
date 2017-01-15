use std;

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

impl ::ToBytes for UInt256 {
   fn to_bytes(&self) -> ::Result<Vec<u8>> {
      self.data.to_bytes()
   }
}
impl ::FromBytes for UInt256 {
   fn from_bytes<S:AsRef<[u8]>>(&mut self, s:S) -> ::Result<()> {
      let s = s.as_ref();
      if s.len() != self.data.len() { frombytes_error!(format!("length mismatch: {} but {}", self.data.len(), s.len())); }
      self.data.clone_from_slice(s);
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
      use ::ToBytes;
      match self.to_rhex() {
         Ok(s)  => f.write_fmt(format_args!("{}", s)),
         Err(e) => f.write_fmt(format_args!("{:?}", e)),
      }
   }
}

#[test]
fn test_str() {
   use ::WithBytes;
   let s = "00000000000008a3a41b85b8b29ad444def299fee21793cd8b9e567eab02cd81";
   let uint256 = UInt256::with_rhex(s).unwrap();

   let expect:[u8;32] = [
      0x81, 0xcd, 0x02, 0xab, 0x7e, 0x56, 0x9e, 0x8b, 0xcd, 0x93, 0x17, 0xe2, 0xfe, 0x99, 0xf2, 0xde,
      0x44, 0xd4, 0x9a, 0xb2, 0xb8, 0x85, 0x1b, 0xa4, 0xa3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
   ];
   assert_eq!(expect, uint256.data);

   let t = format!("{}", uint256);
   assert_eq!(s, t);
}
