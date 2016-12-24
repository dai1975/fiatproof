pub const MESSAGE_START_SIZE:usize  =  4;
pub const COMMAND_SIZE:usize        = 12;

#[derive(Debug,Default)]
pub struct MessageHeader {
   pub start    : [u8; MESSAGE_START_SIZE],
   pub command  : [u8; COMMAND_SIZE],
   pub size     : u32,
   pub checksum : u32,
}

