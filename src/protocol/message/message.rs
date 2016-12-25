pub use super::super::message_command::Command;

pub trait Message {
   const COMMAND: Command;
   fn get_command(&self) -> Command {
      Self::COMMAND
   }
}
