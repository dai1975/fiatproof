pub const TABLE:&str = &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

lazy_static! {
   pub static ref BASE58: crate::utils::BaseN = crate::utils::BaseN::new(TABLE);
}

pub fn new_base58check(version: &[u8]) -> crate::utils::Base58check {
   crate::utils::Base58check::new(TABLE, version)
}


