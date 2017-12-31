use super::num::ScriptNum;
use std::borrow::{Cow};

#[derive(Clone)]
pub enum Pushee<'a> {
   Data(Cow<'a, [u8]>),
   Value(i64, [u8;9], usize),
}

impl <'a> Pushee<'a> {
   pub fn new_data(data: &'a[u8]) -> Self {
      Pushee::Data(Cow::from(data))
   }
   pub fn new_data_copy(data: &[u8]) -> Self {
      let v:Vec<u8> = data.into_iter().cloned().collect();
      Pushee::Data(Cow::from(v))
   }
   pub fn new_value<T:Into<i64>>(v: T) -> Self {
      let v:i64 = v.into();
      let mut data = [0u8; 9];
      let len = ScriptNum::encode(v, &mut data);
      Pushee::Value(v, data, len)
   }

   pub fn data(&self) -> &[u8] {
      match self {
         &Pushee::Data(Cow::Borrowed(x)) => x,
         &Pushee::Data(Cow::Owned(ref v)) => v.as_slice(),
         &Pushee::Value(_, ref a, len) => &a[0..len],
      }
   }
   pub fn value(&self) -> ::Result<i64> {
      match self {
         &Pushee::Data(ref a) => {
            ScriptNum::decode_i64(a) // Cow<T>::deref -> &T
         }
         &Pushee::Value(v, _, _) => Ok(v),
      }
   }
}

impl <'a> ::std::fmt::Display for Pushee<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Pushee::Data(ref data) => write!(f, "[{}]", data.len()),
         &Pushee::Value(ref n, _, _) => write!(f, "{}(0x{:x})", n, n),
      }
   }
}
impl <'a> ::std::fmt::Debug for Pushee<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Pushee::Data(ref data) => write!(f, "Data[{}]", data.len()),
         &Pushee::Value(ref n, _, _) => write!(f, "Value({})", n),
      }
   }
}


impl <'a, 'x> ::std::cmp::PartialEq<Pushee<'x>> for Pushee<'a> {
   fn eq(&self, other:&Pushee<'x>) -> bool {
      match (self, other) {
         (&Pushee::Data(ref lhs), &Pushee::Data(ref rhs)) => {
            lhs == rhs
         },
         (&Pushee::Value(lhs,_,_), &Pushee::Value(rhs,_,_)) => {
            lhs == rhs
         },
         (lhs, rhs) => {
            lhs.data() == rhs.data()
         },
      }
   }
}
