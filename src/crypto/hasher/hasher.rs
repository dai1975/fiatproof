use std;
use ::ToHex;

pub trait Hasher: Default {
   type Out;
   fn size_of() -> usize { std::mem::size_of::<Self::Out>() }
   
   fn reset(&mut self);
   fn input(&mut self, data: &[u8]);
   fn result(&mut self) -> Box<[u8]>;
   fn hexresult(&mut self) -> String {
      self.result().to_hex()
      //let r = box self.result();
      //r.to_hex()
   }

   fn hash(bytes: &[u8]) -> Box<[u8]> {
      let mut hasher = Self::default();
      hasher.input(bytes);
      hasher.result()
   }
   fn hexhash(bytes: &[u8]) -> String {
      let r = box Self::hash(bytes);
      (*r).to_hex()
   }
}
