use super::MessageCommand;

#[derive(Debug,Default,Clone)]
pub struct MessageHeader {
   pub magic:    u32,
   pub command:  MessageCommand,
   pub length:   u32,
   pub checksum: u32,
}

