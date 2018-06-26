use super::ConsensusParams;

pub struct ChainParams<'a> {
   pub coin:  &'a str,
   pub network:  &'a str,
   pub magic: u32,
   pub consensus: ConsensusParams,
   pub base58check: (&'a str, &'a [u8]),
}

