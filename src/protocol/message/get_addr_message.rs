use std;
use super::{Message};
use super::super::{ command, Command };

#[derive(Debug,Default,Clone)]
pub struct GetAddrMessage;

impl Message for GetAddrMessage {
   fn get_command(&self) -> Command { command::GET_ADDR }
}

impl std::fmt::Display for GetAddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "GetAddr()")
   }
}

