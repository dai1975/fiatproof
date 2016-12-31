use std;

pub const SIZE:usize        = 12;

#[derive(Debug)]
pub struct MessageCommand {
   pub data: &'static [u8; SIZE],
}

const DEFAULT:MessageCommand = MessageCommand { data: &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
impl Default for MessageCommand {
   fn default() -> Self {
      DEFAULT
   }
}

impl MessageCommand {
   pub fn as_str(&self) -> &'static str {
      let data:&[u8] = self.data;
      let s =
         match data.iter().position(|&x| x == 0) {
            Some(pos) => { &data[0..pos] }
            None      => { &data[..] }
         };
      std::str::from_utf8(s).unwrap()
   }
}

impl PartialEq for MessageCommand {
   fn eq(&self, that:&Self) -> bool {
      let lp = self.data as *const u8;
      let rp = that.data as *const u8;
      std::ptr::eq(lp, rp)
   }
}

impl Eq for MessageCommand { }

impl Clone for MessageCommand {
   fn clone(&self) -> Self {
      MessageCommand { data: self.data }
   }
}
impl Copy for MessageCommand { }

#[test]
fn test_ptr() {
   const DATA:&'static[u8;SIZE] = &[0u8;SIZE];
   let a = MessageCommand { data: DATA };
   let b = a; //clone
   assert_eq!(a.data as *const u8,      b.data as *const u8);
   assert_ne!(&a     as *const MessageCommand, &b     as *const MessageCommand);
   //println!(" a=@{:?} [0]@{:?}", &a as *const MessageCommand, a.data as *const u8);
   //println!(" b=@{:?} [0]@{:?}", &b as *const MessageCommand, b.data as *const u8);
}
