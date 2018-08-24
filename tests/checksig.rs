/*
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize)]
struct Data {
   transaction: String,
   script:      String,
   input_index: usize,
   hash_type:   i32,
   sighash:     String,
}

#[derive(Deserialize)]
struct Envelope {
   _comment: Vec<String>,
   data:     Vec<Data>,
}

fn load(path:&str) -> Vec<Data> {
   let file = std::fs::File::open(path).unwrap();
   let reader = std::io::BufReader::new(file);
   let e:Envelope = ::serde_json::from_reader(reader).unwrap();
   e.data
}

extern crate fiatproof;
#[test]
#[ignore]
fn test_sighash() {
   use ::fiatproof::{Script, Transaction};
   use ::fiatproof::serialize::{ToBytes,WithBytes};
   use ::fiatproof::script::checksig::CheckSig;

   let data = load("tests/data/checksig.json");
   for d in data {
      let tx        = Transaction::with_hex(d.transaction).unwrap();
      let script    = Script::with_hex(d.script).unwrap();
      let in_idx    = d.input_index;
      let hash_type = d.hash_type;
      let expect    = d.sighash;
      
      let c = CheckSig::new(&tx, in_idx);
      let sighash = c.get_hash(script.bytecode(), hash_type).unwrap();
      let sighash_hex = sighash.to_rhex().unwrap();
            
      assert_eq!(sighash_hex, expect);
   }
}
*/
