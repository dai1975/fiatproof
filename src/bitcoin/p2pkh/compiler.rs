use ::bitcoin::script::error;
use ::bitcoin::script::opcode::*;

#[derive(Debug,Default,Clone)]
pub struct P2PKH {
   pk_script: [u8; 25],
}

fn compile_pk(hash: &[u8], out: &mut [u8; 25]) {
   out[0] = OP_DUP;
   out[1] = OP_HASH160;
   out[2] = OP_PUSHDATAFIX_14;
   out[3..23].clone_from_slice(&hash[0..20]);
   out[23] = OP_EQUALVERIFY;
   out[24] = OP_CHECKSIG;
}
fn is_match_pk(script: &[u8]) -> bool {
   if script.len() != 25 { return false; }
   
   if script[0] != OP_DUP ||
      script[1] != OP_HASH160 ||
      script[2] != OP_PUSHDATAFIX_14 ||
      script[23] != OP_EQUALVERIFY ||
      script[24] != OP_CHECKSIG
   {
      return false;
   }
   true
}

impl P2PKH {
   pub fn new_with_hash(hash: &[u8]) -> Self {
      let mut ret = Self { pk_script: [0u8; 25] };
      compile_pk(hash, &mut ret.pk_script);
      ret
   }
   pub fn pk_script(&self) -> &[u8] { &self.pk_script }
   pub fn pk_hash(&self) -> &[u8] { &self.pk_script[3..23] }
   
   pub fn compile_pk(hash: &[u8]) -> ::Result<Vec<u8>> {
      let p2pkh = P2PKH::new_with_hash(&hash[0..20]);
      Ok(p2pkh.pk_script.to_vec())
   }

   pub fn parse_pk(inp: &[u8]) -> ::Result<P2PKH> {
      if !is_match_pk(inp) {
         raise_parse_script_error!(format!("not a p2pkh pkScript"))
      }
      Ok(P2PKH::new_with_hash(&inp[3..23]))
   }
}


#[cfg(test)]
mod tests {
   const PUBKEY:&[u8]    = hex!("038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508");
   const HASH:&[u8]      = hex!("1018853670f9f3b0582c5b9ee8ce93764ac32b93");
   const PK_SCRIPT:&[u8] = hex!("76 A9 14 1018853670f9f3b0582c5b9ee8ce93764ac32b93 88 AC");

   #[test]
   fn test_compile() {
      use ::bitcoin::P2PKH;
      let p2pkh = P2PKH::new_with_hash(HASH);
      assert_eq!(p2pkh.pk_script(), PK_SCRIPT);
   }

   #[test]
   fn test_parse() {
      use ::bitcoin::P2PKH;
      let p2pkh = P2PKH::parse_pk(PK_SCRIPT);
      assert_matches!(p2pkh, Ok(_));
      let p2pkh = p2pkh.unwrap();
      let p2pkh = P2PKH::new_with_hash(HASH);
      assert_eq!(p2pkh.pk_hash(), HASH);
   }
}
