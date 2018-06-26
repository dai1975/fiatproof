pub struct Chain<'a> {
   pub coin:  &'a str,
   pub network:  &'a str,
   pub magic: u32,
   pub consensus: super::Consensus,
   pub base58check: super::Base58check<'a>,
}

