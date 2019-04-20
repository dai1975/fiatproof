use std;

pub struct ByteSlice <'a>(pub &'a [u8]);

impl<'a> std::fmt::LowerHex for ByteSlice<'a> {
   fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      for byte in self.0 {
         fmtr.write_fmt(format_args!("{:02x}", byte))?;
      }
      Ok(())
   }
}
