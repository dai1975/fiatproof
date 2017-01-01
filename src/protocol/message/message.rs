pub use super::super::MessageCommand;

pub trait Message {
   const COMMAND: MessageCommand;
   fn get_command(&self) -> MessageCommand {
      Self::COMMAND
   }
}
