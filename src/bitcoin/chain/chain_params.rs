use super::ConsensusParams;

pub struct ChainParams<'a> {
   pub name:  &'a str,
   pub magic: u32,
   pub consensus: ConsensusParams,
}

