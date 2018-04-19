pub const COMMAND_LENGTH: usize = super::super::apriori::COMMAND_LENGTH;

pub trait Message {
   const COMMAND: [u8; COMMAND_LENGTH];
}
