use super::super::Command;

pub trait Message {
   fn get_command(&self) -> Command;
}
