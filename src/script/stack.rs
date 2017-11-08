use super::pushee::Pushee;

#[derive(Debug,Clone)]
pub struct Stack<'a> {
   stack: Vec<Pushee<'a>>,
}

impl <'a> Stack<'a> {
   pub fn new() -> Self { Self { stack:  Vec::new() } }

   pub fn clear(&mut self) { self.stack.clear(); }
   pub fn len(&self) -> usize { self.stack.len() }

   pub fn push(&mut self, p:Pushee<'a>) {
      self.stack.push(p)
   }
   pub fn push_data(&mut self, data:&'a [u8])  {
      self.stack.push(Pushee::new_data(data))
   }
   pub fn push_value(&mut self, v:i64) {
      self.stack.push(Pushee::new_value(v))
   }
   pub fn push_bool(&mut self, b:bool) {
      self.stack.push(Pushee::new_value( if b { 1i64 } else { 0i64 } ))
   }

   fn at(&self, idx_: isize) -> ::Result<&Pushee<'a>> {
      let idx = if 0 <= idx_ {
         idx_ as usize
      } else {
         ((self.stack.len() as isize) + idx_) as usize
      };
      if self.stack.len() <= idx { script_error!("few stacks"); }
      Ok(&self.stack[idx as usize])
   }
   
   pub fn pop(&mut self) -> ::Result<Pushee<'a>> {
      let _ = self.at(-1);
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

