extern crate rsbitcoin;
#[macro_use] extern crate assert_matches;

#[test]
fn test_sha1() {
   let mut hasher = rsbitcoin::handy::DIGEST.create("sha1");
   assert_eq!("63ab596f88cb4c8e37e09ba69e9b6f7c85c24304", hasher.u8_to_hex(b"Hatsune Miku"));
}

