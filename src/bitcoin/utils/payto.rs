use crate::bitcoin::chainparams::Base58check as Base58checkParams;

pub enum PayTo {
   P2PKH(crate::bitcoin::p2pkh::P2PKH),
}

fn _decode_address(addr:&str, table: &str, version: &[u8]) -> crate::Result<Box<[u8]>> {
   let check = crate::utils::Base58check::new(table, version);
   check.decode(addr)
}

impl PayTo {
   pub fn parse_address<'a>(addr:&str, params: &'a Base58checkParams) -> Option<PayTo> {
      if let Ok(pkh) = _decode_address(addr, params.table, params.versions.p2pkh) {
         if let Ok(p2) = crate::bitcoin::p2pkh::P2PKH::new_with_pkh(pkh.as_ref()) {
            return Some(PayTo::P2PKH(p2));
         }
      }
      None
   }
   pub fn compile(&self) -> Box<[u8]> {
      match self {
         PayTo::P2PKH(p2) => {
            crate::bitcoin::p2pkh::Compiler::compile(&p2)
         },
      }
   }
}

