#![feature(plugin)]
#![plugin(hex_literals)]
extern crate rsbitcoin;
//#[macro_use] extern crate assert_matches;

const IN:&[u8] = b"Hatsune Miku";

#[test]
fn test_sha1() {
   let mut dig = rsbitcoin::ui::DIGEST.create_sha1();
   assert_eq!("63ab596f88cb4c8e37e09ba69e9b6f7c85c24304", dig.u8_to_hex(IN));
}

#[test]
fn test_sha256() {
   let mut dig = rsbitcoin::ui::DIGEST.create_sha256();
   assert_eq!("4fc2d17a36a087dc7b3df2b4d214c1f704f3b24fbe6417cdeaaf8b5d864700e5", dig.u8_to_hex(IN));
}

#[test]
fn test_ripemd160() {
   let mut dig = rsbitcoin::ui::DIGEST.create_ripemd160();
   assert_eq!("6e7c1f0dd5ce2c69db52dd0ae5820cc0b6f9c7b3", dig.u8_to_hex(IN));
}

#[test]
fn test_dhash256() {
   let mut dig = rsbitcoin::ui::DIGEST.create_dhash256();
   assert_eq!("e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9", dig.u8_to_hex(IN));   
   let mut dig = rsbitcoin::ui::DIGEST.create_sha256();
   let in1:&[u8] = hex!("4fc2d17a36a087dc7b3df2b4d214c1f704f3b24fbe6417cdeaaf8b5d864700e5");
   assert_eq!("e5d17f17a6ad7a94eec6add232a2fb1c2a848465cc8ad1dc030b6d0caa9294d9", dig.u8_to_hex(in1));
}

#[test]
fn test_hash160() {
   let mut dig = rsbitcoin::ui::DIGEST.create_hash160();
   assert_eq!("b7233a798e6ea977644ded49241c2b153a6617b9", dig.u8_to_hex(IN));   
   let mut dig = rsbitcoin::ui::DIGEST.create_ripemd160();
   let in1:&[u8] = hex!("4fc2d17a36a087dc7b3df2b4d214c1f704f3b24fbe6417cdeaaf8b5d864700e5");
   assert_eq!("b7233a798e6ea977644ded49241c2b153a6617b9", dig.u8_to_hex(in1));
}


