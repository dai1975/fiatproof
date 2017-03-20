use super::consensus_params::ConsensusParams;

pub struct ChainParams<'a> {
   pub id:    i32,
   pub name:  &'a str,
   pub magic: u32,
   pub consensus: ConsensusParams,
}

