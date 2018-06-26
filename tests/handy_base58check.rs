extern crate rsbitcoin;
#[macro_use] extern crate assert_matches;

#[test]
fn test_encode_b58check() {
   let base58check = ::rsbitcoin::handy::BITCOIN_MAINNET.create_base58check();
   let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e];
   let enc = "13op3it3Aaiu";
   let result = base58check.encode(&data);
   assert_eq!(enc, result);
}

#[test]
fn test_decode_b58check() {
   let base58check = ::rsbitcoin::handy::BITCOIN_MAINNET.create_base58check();
   let data:&[u8] = &[0x10, 0xc8, 0x51, 0x1e]; //0x10c8511e = 281563422
   let enc = "13op3it3Aaiu";
   let result = base58check.decode(enc);
   assert_matches!(result, Ok(_));
   let result = result.unwrap();
   assert_eq!(0u8, result[0]);
   assert_eq!(data, &result[1..]);
}
