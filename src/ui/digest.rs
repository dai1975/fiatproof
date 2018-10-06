
macro_rules! deffn {
   ($fname:ident, $t:path) => {
      pub fn $fname() -> $t { <$t>::new() }
   }
}

deffn! { create_sha1,      ::crypto::digest::Sha1Helper }
deffn! { create_sha256,    ::crypto::digest::Sha256Helper }
deffn! { create_ripemd160, ::crypto::digest::Ripemd160Helper }
deffn! { create_dhash256,  ::crypto::digest::DHash256Helper }
deffn! { create_hash160,   ::crypto::digest::Hash160Helper }

