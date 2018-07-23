use std;
use std::convert::AsRef;
use ::ui::b2h;

pub trait Hasher: Default {
   type Out;
   fn size_of() -> usize { std::mem::size_of::<Self::Out>() }
   
   fn reset(&mut self);
   fn input<T:AsRef<[u8]>>(&mut self, data:T);
   fn result(&mut self) -> Box<[u8]>;
   fn hexresult(&mut self) -> String {
      b2h(self.result().as_ref())
   }

   fn hash<T:AsRef<[u8]>>(data:T) -> Box<[u8]> {
      let mut hasher = Self::default();
      hasher.input(data);
      hasher.result()
   }
   fn hexhash<T:AsRef<[u8]>>(data:T) -> String {
      b2h(Self::hash(data).as_ref())
   }
}
