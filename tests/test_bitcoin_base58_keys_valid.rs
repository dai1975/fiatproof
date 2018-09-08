#[macro_use] extern crate assert_matches;
extern crate serde;
extern crate serde_json;
extern crate fiatproof;

#[derive(Debug)]
struct EMessage(String);

macro_rules! impl_error {
   ($from:ty) => {
      impl From<$from> for EMessage {
         fn from(err: $from) -> EMessage {
            EMessage(format!("{:?}", err))
         }
      }
   }
}
impl_error!( ::serde_json::error::Error );

#[derive(Debug)]
struct Key {
   chain:         ::fiatproof::ui::bitcoin::Chain,
   try_case_flip: Option<bool>,
   is_privkey:    bool,
   is_compressed: bool,
}
#[derive(Debug)]
struct TestCase {
   pub lineno:  usize,
   pub base58:  String,
   pub payload_hexes: String,
   pub payload_bytes: Box<[u8]>,
   pub key: Key,
}

fn as_string<'a>(v: &'a ::serde_json::Value) -> Result<&'a String, String> {
   match v {
      &::serde_json::Value::String(ref s) => Ok(s),
      _ => Err(String::from("not a string")),
   }
}
fn parse_key<'a>(v: &'a ::serde_json::Value) -> Result<Key, String> {
   match v {
      &::serde_json::Value::Object(ref m) => {
         let is_privkey = match m.get("isPrivkey") {
            Some(&::serde_json::Value::Bool(b)) => Ok(b),
            Some(_) => Err("isPrivkey is not a bool"),
            None => Err("isPrivkey not found"),
         }?;
         let is_compressed = match m.get("isCompressed") {
            Some(&::serde_json::Value::Bool(b)) => Ok(b),
            Some(_) => Err("isCompressed is not a bool"),
            None => {
               if is_privkey {
                  Err("isCompressed not found")
               } else {
                  Ok(true)
               }
            }
         }?;
         let chain = match m.get("chain") {
            Some(&::serde_json::Value::String(ref s)) => {
               if let Some(chain) = ::fiatproof::ui::bitcoin::get_chain(s.as_str()) {
                  Ok(chain)
               } else {
                  Err(format!("unknown chain: {}", s))
               }
            },
            Some(_) => Err(String::from("chain is not a string")),
            None => Err(String::from("chain not found")),
         }?;
         let try_case_flip = match m.get("tryCaseFlip") {
            Some(&::serde_json::Value::Bool(b)) => Ok(Some(b)),
            Some(_) => Err(String::from("tryCaseFlip is not a bool")),
            None => Ok(None),
         }?;
         Ok(Key {
            chain:         chain,
            try_case_flip: try_case_flip,
            is_privkey:    is_privkey,
            is_compressed: is_compressed,
         })
      },
      _ => Err(String::from("not a object")),
   }
}

fn parse_testcase(v: &Vec<::serde_json::Value>, lineno:usize) -> Result<TestCase, String> {
   if v.len() != 3 {
      let base58  = as_string(&v[0])?;
      println!("parse...{:?}", base58);
      Err(String::from("malformed test data"))
   } else {
      use ::std::error::Error;
      let base58  = as_string(&v[0])?;
      let payload = as_string(&v[1])?;
      let payload_bytes = ::fiatproof::utils::h2b(payload.as_str()).map_err(|e|e.description().to_string())?;
      let key = parse_key(&v[2])?;
      Ok(TestCase {
         lineno: lineno,
         base58: base58.to_string(),
         payload_hexes: payload.to_string(),
         payload_bytes: payload_bytes,
         key: key,
      })
   }
}

fn read_testcases() -> Result<Vec<TestCase>, String> {
   let path = "tests/bitcoin-test-data/base58_keys_valid.json";
   let f = ::std::fs::File::open(path).unwrap();
   let lines:Vec< Vec<::serde_json::Value> > = ::serde_json::from_reader(f).unwrap();
   lines.iter().enumerate().fold(Ok(Vec::new()), |acc, (n,s)| {
      match (acc, n, s) {
         (Err(e), _, _) => { Err(e) }
         (Ok(mut v), _, _) => {
            let r = parse_testcase(s, n+1);
            match r {
               Err(msg) => {
                  let msg = format!("{} at {}: {:?}", msg, n, s);
                  Err(msg)
               },
               Ok(tc) => {
                  v.push(tc);
                  Ok(v)
               }
            }
         }
      }
   })
}

fn verify(t: &TestCase) {
   //println!("verify: hexes={}", t.hexes);
   let fail = | head:&str, t: &TestCase, description: &str | {
      println!("");
      println!("FAIL: {}: {}", head, description);
      println!("  payload: {}", t.payload_hexes);
      println!("  base58:  {}", t.base58);
      assert!(false, "test failed");
   };

   if t.base58.chars().nth(0).unwrap() != '1' {
      return;
   }
   
   // param.h 
   if t.key.is_privkey {
      // base58 は秘密鍵らしい
   } else {
      let payto = {
         let tmp = t.key.chain.parse_address(t.base58.as_str());
         if tmp.is_none() {
            fail("parse_address", t, "unknown format");
         }
         tmp.unwrap()
      };
      if t.payload_bytes.as_ref() != payto.compile().as_ref() {
         println!("expected: {:?}", t.payload_bytes.as_ref());
         println!("actual:   {:?}", payto.compile().as_ref());
         fail("script mismatch", t, "unknown format");
      }
   }
   assert!(true);
}

#[test]
fn test_bitcoin_base58_keys_valid() {
   let r = read_testcases();
   assert_matches!(r, Ok(_));
   let tests = r.unwrap();

   for tc in &tests {
      verify(tc);
   }
}


