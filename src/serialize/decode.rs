use super::{WriteStream, ReadStream, Media};

pub trait Encoder<'a> {
}
pub trait Encodee {
   pub type E: Encoder<'a>;
   fn encode(&self, enc: &mut E) -> ::Result<usize>;
}


pub trait Decoder<'a> {
}
pub trait Decodee {
   type D: Decoder<'a>;
   fn decode(&mut self, dec: &mut D) -> ::Result<usize>;
}

use super::Medium;
pub struct StreamCodecInner<'a, S> {
   stream: &'a mut S,
   medium: Medium,
}
impl <'a, S> StreamCodecInner<'a,S> { // macro か trait にしたい
   pub fn new(s: &'a mut S) -> Self {
      Self { stream:s, medium:Medium::default() }
   }
   pub fn medium(&self) -> &'a Medium {
      &self.medium
   }
   pub fn update_medium<F>(&mut self, f:F) -> Medium
      where F: Fn(Media)->Medium
   {
      let r = self.medium;
      self.medium = f(self.medium.clone());
      r
   }
}

