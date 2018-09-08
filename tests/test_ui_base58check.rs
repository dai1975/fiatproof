extern crate fiatproof;
#[macro_use] extern crate assert_matches;

#[test]
fn test_serialize_b58check() {
   let base58check = ::fiatproof::ui::bitcoin::MAINNET.create_base58check_p2pkh();
   let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e];
   let enc = "13op3it3Aaiu";
   let result = base58check.serialize(&data);
   assert_eq!(enc, result);
}

#[test]
fn test_deserialize_b58check() {
   let base58check = ::fiatproof::ui::bitcoin::MAINNET.create_base58check_p2pkh();
   let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
   let enc = "13op3it3Aaiu";
   let result = base58check.deserialize(enc);
   assert_matches!(result, Ok(_));
   assert_eq!(data, result.unwrap().as_ref());
}
