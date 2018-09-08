use ::bitcoin::script::error;
use ::bitcoin::script::opcode::*;
use super::P2PKH;

pub struct Compiler();

impl Compiler {
   pub fn compile_to(p2pkh: &P2PKH, pk_script: &mut [u8; 25]) {
      pk_script[0] = OP_DUP;
      pk_script[1] = OP_HASH160;
      pk_script[2] = OP_PUSHDATAFIX_14;
      pk_script[3..23].clone_from_slice(p2pkh.pkh());
      pk_script[23] = OP_EQUALVERIFY;
      pk_script[24] = OP_CHECKSIG;
   }
   pub fn compile(p2pkh: &P2PKH) -> Box<[u8]> {
      let mut pk_script = [0u8; 25];
      Self::compile_to(p2pkh, &mut pk_script);
      Box::new(pk_script)
   }
}

pub struct Parser();

impl Parser {
   pub fn check(script: &[u8]) -> ::Result<()> {
      if script.len() != 25 {
         raise_parse_script_error!(format!("length mismatch: {}", script.len()));
      }
      
      if script[0] != OP_DUP ||
         script[1] != OP_HASH160 ||
         script[2] != OP_PUSHDATAFIX_14 ||
         script[23] != OP_EQUALVERIFY ||
         script[24] != OP_CHECKSIG
      {
         raise_parse_script_error!(format!("script mismatch"));
      }
      Ok(())
   }

   pub fn parse(script: &[u8]) -> ::Result<P2PKH> {
      if let Err(e) = Self::check(script) {
         use ::std::error::Error;
         raise_parse_script_error!(format!("not a p2pkh pkScript: {}", e.description()));
      }
      Ok(P2PKH::new([script[3], script[4], script[5], script[6], script[7],
                     script[8], script[9], script[10], script[11], script[12],
                     script[13], script[14], script[15], script[16], script[17],
                     script[18], script[19], script[20], script[21], script[22]]))
   }
}




#[cfg(test)]
mod tests {
   //const PUBKEY:&[u8]    = hex!("038282263212c609d9ea2a6e3e172de238d8c39cabd5ac1ca10646e23fd5f51508");
   const HASH:&[u8]      = hex!("1018853670f9f3b0582c5b9ee8ce93764ac32b93");
   const PK_SCRIPT:&[u8] = hex!("76 A9 14 1018853670f9f3b0582c5b9ee8ce93764ac32b93 88 AC");

   #[test]
   fn test_compile() {
      let p2pkh = ::bitcoin::p2pkh::P2PKH::new_with_pkh(HASH).unwrap();
      let pk_script = ::bitcoin::p2pkh::Compiler::compile(&p2pkh);
      assert_eq!(pk_script.as_ref(), PK_SCRIPT);
   }

   #[test]
   fn test_parse() {
      let p2pkh = ::bitcoin::p2pkh::Parser::parse(PK_SCRIPT);
      assert_matches!(p2pkh, Ok(_));
      assert_eq!(p2pkh.unwrap().pkh(), HASH);
   }
}
