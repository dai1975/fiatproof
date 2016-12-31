use std;

pub const SIZE:usize        = 12;

#[derive(Debug)]
pub struct Command {
   pub data: &'static [u8; SIZE],
}

impl Command {
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

impl PartialEq for Command {
   fn eq(&self, that:&Self) -> bool {
      let lp = self.data as *const u8;
      let rp = that.data as *const u8;
      std::ptr::eq(lp, rp)
   }
}

impl Eq for Command { }

impl Clone for Command {
   fn clone(&self) -> Self {
      Command { data: self.data }
   }
}
impl Copy for Command { }

#[test]
fn test_ptr() {
   const DATA:&'static[u8;SIZE] = &[0u8;SIZE];
   let a = Command { data: DATA };
   let b = a; //clone
   assert_eq!(a.data as *const u8,      b.data as *const u8);
   assert_ne!(&a     as *const Command, &b     as *const Command);
   //println!(" a=@{:?} [0]@{:?}", &a as *const Command, a.data as *const u8);
   //println!(" b=@{:?} [0]@{:?}", &b as *const Command, b.data as *const u8);
}
