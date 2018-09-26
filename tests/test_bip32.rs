#![feature(plugin)]
#![plugin(hex_literals)]
extern crate fiatproof;
use fiatproof::crypto::bip32;

#[macro_use] extern crate lazy_static;

struct TestVector {
   path: &'static str,
   index: u32,
   xpub_hex: &'static str,
   xprv_hex: &'static str,
}
impl TestVector {
   pub fn new(path: &'static str, i:u32, xpub_hex: &'static str, xprv_hex: &'static str) -> Self {
      Self { path:path, index:i, xpub_hex:xpub_hex, xprv_hex:xprv_hex }
   }
   pub fn xpub(&self) -> bip32::XPub {
      let dec = fiatproof::ui::bitcoin::MAINNET.create_xpub_decoder();
      dec.decode(self.xpub_hex).unwrap()
   }
   pub fn xprv(&self) -> bip32::XPrv {
      let dec = fiatproof::ui::bitcoin::MAINNET.create_xprv_decoder();
      dec.decode(self.xprv_hex).unwrap()
   }
   pub fn is_hardened(&self) -> bool {
      self.path.chars().last() == Some('H')
   }
}
struct TestSet {
   seed: &'static str,
   v:    Vec<TestVector>,
}

lazy_static! {
   static ref TEST_VECTOR_1:TestSet = TestSet {
      seed: "000102030405060708090a0b0c0d0e0f",
      v: vec![
         TestVector::new(
            "m", 0,
            "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8",
            "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
         ),
         TestVector::new(
            "m/0H", 0x80000000,
            "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw",
            "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"
         ),
         TestVector::new(
            "m/0H/1", 1, 
            "xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ",
            "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs"
         ),
         TestVector::new(
            "m/0H/1/2H", 0x80000002,
            "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5",
            "xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM"
         ),
         TestVector::new(
            "m/0H/1/2H/2", 2,
            "xpub6FHa3pjLCk84BayeJxFW2SP4XRrFd1JYnxeLeU8EqN3vDfZmbqBqaGJAyiLjTAwm6ZLRQUMv1ZACTj37sR62cfN7fe5JnJ7dh8zL4fiyLHV",
            "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334"
         ),
         TestVector::new(
            "m/0H/1/2H/2/1000000000", 1000000000,
            "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy",
            "xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"
         ),
      ],
   };
}

#[test]
fn test_decode_v1() {
   TEST_VECTOR_1.v.iter().enumerate().for_each(|(i,tv)| {
      let xpub = tv.xpub();
      assert_eq!(xpub.depth, i as u8);
      assert_eq!(xpub.index, tv.index);
      let xprv = tv.xprv();
      assert_eq!(xprv.xpub.depth, xpub.depth);
      assert_eq!(xprv.xpub.index, xpub.index);
      assert_eq!(xprv.xpub.public_key, xpub.public_key);
   });
}

#[test]
fn test_encode_v1() {
   let enc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   TEST_VECTOR_1.v.iter().for_each(|tv| {
      let s = enc.encode(&tv.xpub());
      assert_eq!(tv.xpub_hex, s.as_str());
   });

   let enc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
   TEST_VECTOR_1.v.iter().for_each(|tv| {
      let s = enc.encode(&tv.xprv());
      assert_eq!(tv.xprv_hex, s.as_str());
   });
}

#[test]
fn test_xpub_derive_v1() {
   let enc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   let t = |parent:bip32::XPub, tv:&TestVector| -> bip32::XPub {
      let xpub0 = tv.xpub();
      let xpub  = parent.derive(tv.index);
      if tv.is_hardened() {
         assert!(xpub.is_err());
      } else {
         assert!(xpub.is_ok());
         let xpub = xpub.unwrap();
         assert_eq!(xpub.depth, xpub0.depth);
         assert_eq!(xpub.index, xpub0.index);
         assert_eq!(xpub.parent_fingerprint, xpub0.parent_fingerprint);
         assert_eq!(xpub.chain_code, xpub0.chain_code);
         assert_eq!(xpub.public_key, xpub0.public_key);
         assert_eq!(enc.encode(&xpub).as_str(), tv.xpub_hex);
      }
      xpub0
   };
   
   let xpub = TEST_VECTOR_1.v[0].xpub();
   let xpub = t(xpub, &TEST_VECTOR_1.v[1]);
   let xpub = t(xpub, &TEST_VECTOR_1.v[2]);
   let xpub = t(xpub, &TEST_VECTOR_1.v[3]);
   let xpub = t(xpub, &TEST_VECTOR_1.v[4]);
   let xpub = t(xpub, &TEST_VECTOR_1.v[5]);   
}

#[test]
fn test_xprv_derive_v1() {
   let prvenc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
   let pubenc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   let t = |parent:bip32::XPrv, tv:&TestVector| -> bip32::XPrv {
      let xprv0 = tv.xprv();
      let xprv = parent.derive(tv.index);
      assert!(xprv.is_ok());
      let xprv = xprv.unwrap();
      assert_eq!(xprv.secret_key, xprv0.secret_key);
      assert_eq!(xprv.xpub.depth, xprv0.xpub.depth);
      assert_eq!(xprv.xpub.index, xprv0.xpub.index);
      assert_eq!(xprv.xpub.parent_fingerprint, xprv0.xpub.parent_fingerprint);
      assert_eq!(xprv.xpub.chain_code, xprv0.xpub.chain_code);
      assert_eq!(prvenc.encode(&xprv).as_str(), tv.xprv_hex);
      assert_eq!(pubenc.encode(&xprv.xpub).as_str(), tv.xpub_hex);
      xprv
   };
   
   let xprv = TEST_VECTOR_1.v[0].xprv();
   let xprv = t(xprv, &TEST_VECTOR_1.v[1]);
   let xprv = t(xprv, &TEST_VECTOR_1.v[2]);
   let xprv = t(xprv, &TEST_VECTOR_1.v[3]);
   let xprv = t(xprv, &TEST_VECTOR_1.v[4]);
   let xprv = t(xprv, &TEST_VECTOR_1.v[5]);
}

#[test]
fn test_seed() {
   let prvenc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
   let seed = fiatproof::utils::h2b(TEST_VECTOR_1.seed).unwrap();
   let xprv = bip32::XPrv::from_seed(&seed);
   assert!(xprv.is_ok());
   let xprv = xprv.unwrap();
   assert_eq!(prvenc.encode(&xprv).as_str(), TEST_VECTOR_1.v[0].xprv_hex);
}

