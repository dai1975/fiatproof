#![feature(plugin)]
extern crate fiatproof;
use fiatproof::crypto::bip32;

use lazy_static::lazy_static;

struct TestEntity {
   path: &'static str,
   index: u32,
   xpub_hex: &'static str,
   xprv_hex: &'static str,
}
impl TestEntity {
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
struct TestVector {
   seed: &'static str,
   e:    Vec<TestEntity>,
}

lazy_static! {
   static ref TEST_VECTOR_1:TestVector = TestVector {
      seed: "000102030405060708090a0b0c0d0e0f",
      e: vec![
         TestEntity::new(
            "m", 0,
            "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8",
            "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
         ),
         TestEntity::new(
            "m/0H", 0x80000000,
            "xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw",
            "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"
         ),
         TestEntity::new(
            "m/0H/1", 1, 
            "xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ",
            "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs"
         ),
         TestEntity::new(
            "m/0H/1/2H", 0x80000002,
            "xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5",
            "xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM"
         ),
         TestEntity::new(
            "m/0H/1/2H/2", 2,
            "xpub6FHa3pjLCk84BayeJxFW2SP4XRrFd1JYnxeLeU8EqN3vDfZmbqBqaGJAyiLjTAwm6ZLRQUMv1ZACTj37sR62cfN7fe5JnJ7dh8zL4fiyLHV",
            "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334"
         ),
         TestEntity::new(
            "m/0H/1/2H/2/1000000000", 1000000000,
            "xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy",
            "xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"
         ),
      ],
   };
   static ref TEST_VECTOR_2:TestVector = TestVector {
      seed: "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
      e: vec![
         TestEntity::new(
            "m", 0, 
            "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB",
            "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U"
         ),
         TestEntity::new(
            "m/0", 0, 
            "xpub69H7F5d8KSRgmmdJg2KhpAK8SR3DjMwAdkxj3ZuxV27CprR9LgpeyGmXUbC6wb7ERfvrnKZjXoUmmDznezpbZb7ap6r1D3tgFxHmwMkQTPH",
            "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt"
         ),
         TestEntity::new(
            "m/0/2147483647H", 2147483647 | 0x80000000,
            "xpub6ASAVgeehLbnwdqV6UKMHVzgqAG8Gr6riv3Fxxpj8ksbH9ebxaEyBLZ85ySDhKiLDBrQSARLq1uNRts8RuJiHjaDMBU4Zn9h8LZNnBC5y4a",
            "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9"
         ),
         TestEntity::new(
            "m/0/2147483647H/1", 1,
            "xpub6DF8uhdarytz3FWdA8TvFSvvAh8dP3283MY7p2V4SeE2wyWmG5mg5EwVvmdMVCQcoNJxGoWaU9DCWh89LojfZ537wTfunKau47EL2dhHKon",
            "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef"
         ),
         TestEntity::new(
            "m/0/2147483647H/1/2147483646H", 2147483646 | 0x80000000,
            "xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL",
            "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc"
         ),
         TestEntity::new(
            "m/0/2147483647H/1/2147483646H/2", 2,
            "xpub6FnCn6nSzZAw5Tw7cgR9bi15UV96gLZhjDstkXXxvCLsUXBGXPdSnLFbdpq8p9HmGsApME5hQTZ3emM2rnY5agb9rXpVGyy3bdW6EEgAtqt",
            "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j"
         )
      ],
   };
   static ref TEST_VECTOR_3:TestVector = TestVector {
      seed: "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4acba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be",
      e: vec![
         TestEntity::new(
            "m", 0,
            "xpub661MyMwAqRbcEZVB4dScxMAdx6d4nFc9nvyvH3v4gJL378CSRZiYmhRoP7mBy6gSPSCYk6SzXPTf3ND1cZAceL7SfJ1Z3GC8vBgp2epUt13",
            "xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6"
         ),
         TestEntity::new(
            "m/0H", 0x80000000,
            "xpub68NZiKmJWnxxS6aaHmn81bvJeTESw724CRDs6HbuccFQN9Ku14VQrADWgqbhhTHBaohPX4CjNLf9fq9MYo6oDaPPLPxSb7gwQN3ih19Zm4Y",
            "xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L"
         )
      ]
   };
}

#[test]
fn test_decode() {
   let t = |tv:&TestVector| {
      tv.e.iter().enumerate().for_each(|(i,te)| {
         let xpub = te.xpub();
         assert_eq!(xpub.depth, i as u8);
         assert_eq!(xpub.index, te.index);
         let xprv = te.xprv();
         assert_eq!(xprv.xpub.depth, xpub.depth);
         assert_eq!(xprv.xpub.index, xpub.index);
         assert_eq!(xprv.xpub.public_key, xpub.public_key);
      });
   };
   t(&TEST_VECTOR_1);
   t(&TEST_VECTOR_2);
   t(&TEST_VECTOR_3);
}

#[test]
fn test_xpub_encode() {
   let enc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   TEST_VECTOR_1.e.iter().for_each(|te| {
      let s = enc.encode(&te.xpub());
      assert_eq!(te.xpub_hex, s.as_str());
   });
}

#[test]
fn test_xprv_encode() {
   let enc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
   TEST_VECTOR_1.e.iter().for_each(|te| {
      let s = enc.encode(&te.xprv());
      assert_eq!(te.xprv_hex, s.as_str());
   });
}

#[test]
fn test_xpub_derive() {
   let enc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   let t = |parent:bip32::XPub, te:&TestEntity| -> bip32::XPub {
      let xpub0 = te.xpub();
      let xpub  = parent.derive(te.index);
      if te.is_hardened() {
         assert!(xpub.is_err());
      } else {
         assert!(xpub.is_ok());
         let xpub = xpub.unwrap();
         assert_eq!(xpub.depth, xpub0.depth);
         assert_eq!(xpub.index, xpub0.index);
         assert_eq!(xpub.parent_fingerprint, xpub0.parent_fingerprint);
         assert_eq!(xpub.chain_code, xpub0.chain_code);
         assert_eq!(xpub.public_key, xpub0.public_key);
         assert_eq!(enc.encode(&xpub).as_str(), te.xpub_hex);
      }
      xpub0
   };
   
   let xpub = TEST_VECTOR_1.e[0].xpub();
   let xpub = t(xpub, &TEST_VECTOR_1.e[1]);
   let xpub = t(xpub, &TEST_VECTOR_1.e[2]);
   let xpub = t(xpub, &TEST_VECTOR_1.e[3]);
   let xpub = t(xpub, &TEST_VECTOR_1.e[4]);
   let _pub = t(xpub, &TEST_VECTOR_1.e[5]);   

   let xpub = TEST_VECTOR_2.e[0].xpub();
   let xpub = t(xpub, &TEST_VECTOR_2.e[1]);
   let xpub = t(xpub, &TEST_VECTOR_2.e[2]);
   let xpub = t(xpub, &TEST_VECTOR_2.e[3]);
   let xpub = t(xpub, &TEST_VECTOR_2.e[4]);
   let _pub = t(xpub, &TEST_VECTOR_2.e[5]);
   
   let xpub = TEST_VECTOR_3.e[0].xpub();
   let _pub = t(xpub, &TEST_VECTOR_3.e[1]);
}

#[test]
fn test_xprv_derive() {
   let prvenc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
   let pubenc = fiatproof::ui::bitcoin::MAINNET.create_xpub_encoder();
   let t = |parent:bip32::XPrv, te:&TestEntity| -> bip32::XPrv {
      let xprv0 = te.xprv();
      let xprv = parent.derive(te.index);
      assert!(xprv.is_ok());
      let xprv = xprv.unwrap();
      assert_eq!(xprv.secret_key, xprv0.secret_key);
      assert_eq!(xprv.xpub.depth, xprv0.xpub.depth);
      assert_eq!(xprv.xpub.index, xprv0.xpub.index);
      assert_eq!(xprv.xpub.parent_fingerprint, xprv0.xpub.parent_fingerprint);
      assert_eq!(xprv.xpub.chain_code, xprv0.xpub.chain_code);
      assert_eq!(prvenc.encode(&xprv).as_str(), te.xprv_hex);
      assert_eq!(pubenc.encode(&xprv.xpub).as_str(), te.xpub_hex);
      xprv
   };
   
   let xprv = TEST_VECTOR_1.e[0].xprv();
   let xprv = t(xprv, &TEST_VECTOR_1.e[1]);
   let xprv = t(xprv, &TEST_VECTOR_1.e[2]);
   let xprv = t(xprv, &TEST_VECTOR_1.e[3]);
   let xprv = t(xprv, &TEST_VECTOR_1.e[4]);
   let _prv = t(xprv, &TEST_VECTOR_1.e[5]);

   let xprv = TEST_VECTOR_2.e[0].xprv();
   let xprv = t(xprv, &TEST_VECTOR_2.e[1]);
   let xprv = t(xprv, &TEST_VECTOR_2.e[2]);
   let xprv = t(xprv, &TEST_VECTOR_2.e[3]);
   let xprv = t(xprv, &TEST_VECTOR_2.e[4]);
   let _prv = t(xprv, &TEST_VECTOR_2.e[5]);

   let xprv = TEST_VECTOR_3.e[0].xprv();
   let _prv = t(xprv, &TEST_VECTOR_3.e[1]);
}

#[test]
fn test_seed() {
   let t = |tv:&TestVector| {
      let prvenc = fiatproof::ui::bitcoin::MAINNET.create_xprv_encoder();
      let seed = fiatproof::utils::h2b(tv.seed).unwrap();
      let xprv = bip32::XPrv::from_seed(seed);
      assert!(xprv.is_ok());
      let xprv = xprv.unwrap();
      assert_eq!(prvenc.encode(&xprv).as_str(), tv.e[0].xprv_hex);
   };
   t(&TEST_VECTOR_1);
   t(&TEST_VECTOR_2);
   t(&TEST_VECTOR_3);
}

