use super::num::ScriptNum;

#[derive(Debug, Clone)]
pub enum Entry {
   Data(Vec<u8>),
   Value(i64, [u8;9], usize),
}

impl Entry {
   pub fn new_data(data: &[u8]) -> Self {
      Entry::Data(data.iter().cloned().collect())
   }
   pub fn new_value<T:Into<i64>>(v: T) -> Self {
      let v:i64 = v.into();
      let mut data = [0u8; 9];
      let len = ScriptNum::encode(v, &mut data);
      Entry::Value(v, data, len)
   }

   pub fn data(&self) -> &[u8] {
      match self {
         &Entry::Data(ref v) => v.as_slice(),
         &Entry::Value(_, ref a, len) => &a[0..len],
      }
   }
   pub fn value(&self, require_minimal:bool, max_len:usize) -> ::Result<i64> {
      match self {
         &Entry::Data(ref v) => {
            ScriptNum::decode_i64(v.as_slice(), require_minimal, max_len)
         },
         &Entry::Value(v, _, _) => Ok(v),
      }
   }
   pub fn as_bool(&self) -> bool {
      let data = self.data();
      for (i,d) in data.iter().enumerate() {
         if *d != 0 {
            if i == data.len()-1 && *d == 0x80 {
               return false;
            } else {
               return true;
            }
         }
      }
      return false;
   }
}

impl ::std::cmp::PartialEq<Entry> for Entry {
   fn eq(&self, other:&Entry) -> bool {
      match (self, other) {
         (&Entry::Data(ref lhs), &Entry::Data(ref rhs)) => {
            lhs == rhs
         },
         (&Entry::Value(lhs,_,_), &Entry::Value(rhs,_,_)) => {
            lhs == rhs
         },
         (lhs, rhs) => {
            lhs.data() == rhs.data()
         },
      }
   }
}


#[derive(Debug,Clone)]
pub struct Stack {
   stack: Vec<Entry>,
}

impl Stack {
   pub fn new() -> Self { Self { stack:  Vec::new() } }

   pub fn clear(&mut self) { self.stack.clear(); }
   pub fn len(&self) -> usize { self.stack.len() }

   pub fn push(&mut self, e:Entry) {
      self.stack.push(e)
   }
   pub fn push_data(&mut self, data:&[u8])  {
      self.stack.push(Entry::new_data(data))
   }
   pub fn push_value(&mut self, v:i64) {
      self.stack.push(Entry::new_value(v))
   }
   pub fn push_bool(&mut self, b:bool) {
      self.stack.push(Entry::new_value( if b { 1i64 } else { 0i64 } ))
   }

   pub fn at(&self, idx_: isize) -> ::Result<&Entry> {
      let idx = if 0 <= idx_ {
         idx_ as usize
      } else {
         let i = (self.stack.len() as isize) + idx_;
         if i < 0 { script_error!("few stacks"); }
         i as usize
      };
      if self.stack.len() <= idx { script_error!("few stacks"); }
      Ok(&self.stack[idx as usize])
   }
   
   pub fn pop(&mut self) -> ::Result<Entry> {
      if self.stack.len() < 1 { script_error!("few stacks"); }
      Ok(self.stack.pop().unwrap())
   }
   pub fn dup_at(&mut self, idx:isize) -> ::Result<()> {
      let e = {
         let e = try!(self.at(idx));
         e.clone()
      };
      self.stack.push(e);
      Ok(())
   }
}

