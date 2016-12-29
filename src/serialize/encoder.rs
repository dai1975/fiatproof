use ::Error;

pub trait Encoder {
   fn encode_bool(&mut self, v:bool) -> Result<usize, Error>;
   fn encode_u8  (&mut self, v:u8)   -> Result<usize, Error>;
   fn encode_u16 (&mut self, v:u16)  -> Result<usize, Error>;
   fn encode_u32 (&mut self, v:u32)  -> Result<usize, Error>;
   fn encode_u64 (&mut self, v:u64)  -> Result<usize, Error>;
   fn encode_i8  (&mut self, v:i8)   -> Result<usize, Error>;
   fn encode_i16 (&mut self, v:i16)  -> Result<usize, Error>;
   fn encode_i32 (&mut self, v:i32)  -> Result<usize, Error>;
   fn encode_i64 (&mut self, v:i64)  -> Result<usize, Error>;
}

