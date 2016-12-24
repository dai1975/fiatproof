use std;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct FilterClearMessage;

impl Message for FilterClearMessage {
   fn get_command(&self) -> Command { command::FILTER_CLEAR }
}

impl std::fmt::Display for FilterClearMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "FilterClear()")
   }
}

