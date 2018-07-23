pub struct Versions<'a> {
   pub pubkey_hash: &'a [u8],
   pub script_hash: &'a [u8],
}

pub struct Base58check<'a> {
   pub table:   &'a str,
   pub versions: Versions<'a>,
}

