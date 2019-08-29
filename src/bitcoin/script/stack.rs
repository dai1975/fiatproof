use super::num::ScriptNum;

#[derive(Clone)]
pub enum Entry {
   Data(Vec<u8>),
   Value(i64, [u8;9], usize),
}
impl std::fmt::Debug for Entry {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         &Entry::Data(ref v) => {
            write!(f, "Data(")?;
            for x in v { write!(f, "{:X} ", x)?; }
            write!(f, ")")?;
         },
         &Entry::Value(v, ref a, size) => {
            write!(f, "Value({}", v)?;
            for x in &a[0..size] { write!(f, "{:X} ", x)?; }
            write!(f, ")")?;
         },
      }
      Ok(())
   }
}

impl Entry {
   pub fn new_data(data: &[u8]) -> Self {
      Entry::Data(data.iter().cloned().collect())
   }
   pub fn new_value<T:Into<i64>>(v: T) -> Self {
      let v:i64 = v.into();
      let mut data = [0u8; 9];
      let len = ScriptNum::serialize(v, &mut data);
      Entry::Value(v, data, len)
   }

   pub fn data(&self) -> &[u8] {
      match self {
         &Entry::Data(ref v) => v.as_slice(),
         &Entry::Value(_, ref a, len) => &a[0..len],
      }
   }
   pub fn value(&self, require_minimal:bool, max_len:usize) -> crate::Result<i64> {
      match self {
         &Entry::Data(ref v) => {
            ScriptNum::deserialize_i64(v.as_slice(), require_minimal, max_len)
         },
         &Entry::Value(v, data, size) => {
            if max_len < size {
               raise_script_error!(format!("data is longer: max={} but {}", max_len, size));
            }
            if require_minimal && 0 < size {
               ScriptNum::check_minimal(&data[0..size])?;
            }
            Ok(v)
         },
      }
   }
   pub fn as_i32(&self, require_minimal:bool, max_len:usize) -> crate::Result<i32> {
      let v = self.value(require_minimal, max_len)?;
      if (std::i32::MAX as i64) < v {
         Ok(std::i32::MAX)
      } else if v < (std::i32::MIN as i64) {
         Ok(std::i32::MIN)
      } else {
         Ok(v as i32)
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
   pub fn is_minimal_if(&self) -> bool {
      let d = self.data();
      d.len() == 0 || (d.len() == 1 && d[0] == 0)
   }
}

impl std::cmp::PartialEq<Entry> for Entry {
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

#[inline] fn opt_regulate_index(size:usize, idx_:isize) -> Option<usize> {
   if 0 <= idx_ {
      let i = idx_ as usize;
      if i < size { return Some(i as usize); }
   } else {
      let i = (-idx_) as usize;
      if i <= size { return Some((size - i) as usize); }
   }
   None
}
#[inline] fn regulate_index(size:usize, idx_:isize) -> usize {
   let o = opt_regulate_index(size, idx_);
   if o.is_none() { panic!("index out of range"); }
   o.unwrap()
}
 
impl std::ops::Index<isize> for Stack {
   type Output = Entry;
   fn index<'a>(&'a self, index: isize) -> &'a Entry {
      let i = regulate_index(self.stack.len(), index);
      &self.stack[i]
   }
}
/* unstable
impl std::slice::AsSlice<Entry> for Stack {
   fn as_slice<'a>(&'a self) -> &'a [Entry] {
      self.stack.as_slice()
   }
}
*/

impl Stack {
   pub fn new() -> Self { Self { stack:  Vec::new() } }
   pub fn from_vecs(vecs: &[Vec<u8>]) -> Self {
      Self { stack: vecs.iter().map(|v| Entry::new_data(v.as_slice())).collect() }
   }

   pub fn clear(&mut self) { self.stack.clear(); }
   pub fn truncate(&mut self, len: usize) { self.stack.truncate(len); }
   pub fn len(&self) -> usize { self.stack.len() }

   pub fn as_slice(&self) -> &[Entry] {
      self.stack.as_slice()
   }
   
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

   pub fn top(&self) -> crate::Result<&Entry> {
      let len = self.stack.len();
      if len < 1 { script_error!("few stacks"); }
      Ok(&self.stack[len-1])
   }
   
   pub fn at(&self, idx_: isize) -> crate::Result<&Entry> {
      let o = opt_regulate_index(self.stack.len(), idx_);
      if o.is_none() { script_error!("few stacks"); }
      Ok(&self.stack[o.unwrap()])
   }
   
   pub fn pop(&mut self) -> crate::Result<Entry> {
      if self.stack.len() < 1 { script_error!("few stacks"); }
      Ok(self.stack.pop().unwrap())
   }
   pub fn remove_at(&mut self, idx_:isize) -> crate::Result<Entry> {
      let o = opt_regulate_index(self.stack.len(), idx_);
      if o.is_none() { script_error!("few stacks"); }
      let idx = o.unwrap();
      if idx+2 == self.len() {
         Ok(self.stack.swap_remove(idx))
      } else {
         Ok(self.stack.remove(idx))
      }
   }
   pub fn swap(&mut self, a_:isize, b_:isize) -> crate::Result<()> {
      let a = opt_regulate_index(self.stack.len(), a_);
      let b = opt_regulate_index(self.stack.len(), b_);
      if a.is_none() || b.is_none() {
         script_error!("few stacks");
      }
      self.stack.swap(a.unwrap(), b.unwrap());
      Ok(())
   }
   pub fn dup_at(&mut self, idx:isize) -> crate::Result<()> {
      let e = {
         let e = self.at(idx)?;
         e.clone()
      };
      self.stack.push(e);
      Ok(())
   }
   pub fn insert_at(&mut self, idx_:isize, e:Entry) -> crate::Result<()> {
      if 0 < idx_ && (idx_ as usize) == self.stack.len() {
         self.stack.insert(idx_ as usize, e);
      } else {
         let o = opt_regulate_index(self.stack.len(), idx_);
         if o.is_none() { script_error!("few stacks"); }
         self.stack.insert(o.unwrap(), e);
      }
      Ok(())
   }
}

