pub mod hasher;
pub use self::hasher::Hasher;

pub mod rust_crypto;
pub use self::rust_crypto::Sha256;
pub use self::rust_crypto::Ripemd160;

pub mod multi;
pub use self::multi::Multi2;
pub type DHash256 = Multi2<Sha256, Sha256>;
pub type Hash160  = Multi2<Sha256, Ripemd160>;


#[test]
fn test_dhash256() {
   let input  = b"hello";
   let expect = "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50";

   assert_eq!(32, DHash256::size_of());
   assert_eq!(expect, DHash256::hexhash(input));
}

#[test]
fn test_hash160() {
   let input  = b"hello";
   let expect = "b6a9c8c230722b7c748331a8b450f05566dc7d0f";

   assert_eq!(20, Hash160::size_of());
   assert_eq!(expect, Hash160::hexhash(input));
}

