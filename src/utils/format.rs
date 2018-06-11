pub struct FmtVec<T: ::std::fmt::Display>(pub Vec<T>);

impl <T: ::std::fmt::Display> ::std::fmt::Display for FmtVec<T> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      let o = self.0.iter().fold(None, |acc,item| {
         match (acc, item) {
            (None, _)         => Some(item.fmt(f)),
            (Some(Err(e)), _) => Some(Err(e)),
            (Some(Ok(_)), _)  => Some(f.write_str(" ").and_then(|_| item.fmt(f))),
         }
      });
      match o {
         None    => Ok(()),
         Some(r) => r,
      }
   }
}

