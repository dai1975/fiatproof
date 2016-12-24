use std;

pub struct ByteArray <'a>(pub &'a [u8]);

impl<'a> std::fmt::LowerHex for ByteArray<'a> {
   fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      for byte in self.0 {
         try!( fmtr.write_fmt(format_args!("{:02x}", byte)));
      }
      Ok(())
   }
}
