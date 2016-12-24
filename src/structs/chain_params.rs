use super::ConsensusParams;

pub struct ChainParams<'a> {
   pub id   : i32,
   pub name : &'a str,
   pub message_start : [u8;4],
   pub consensus: ConsensusParams,
}

