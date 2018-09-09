pub const TABLE:&str = &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

lazy_static! {
   pub static ref BASE58: ::utils::BaseN = ::utils::BaseN::new(TABLE);
}

pub fn new_base58check(version: &[u8]) -> ::utils::Base58check {
   ::utils::Base58check::new(TABLE, version)
}


