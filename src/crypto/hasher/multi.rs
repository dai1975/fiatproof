use super::{Hasher};

#[derive(Default)]
pub struct Multi2< T1, T2 > where T1:Hasher, T2:Hasher {
   h1: T1,
   h2: T2,
}

impl <T1,T2> Hasher for Multi2<T1, T2> where T1:Hasher, T2:Hasher {
   type Out = T2::Out;
   
   fn reset(&mut self) {
      self.h1.reset();
   }
   fn input<T: ::std::convert::AsRef<[u8]>>(&mut self, data:T) {
      self.h1.input(data);
   }
   fn result(&mut self) -> Box<[u8]> {
      let r1 = self.h1.result();
      self.h2.reset();
      self.h2.input(&*r1);
      let r2 = self.h2.result();
      r2
   }
}

