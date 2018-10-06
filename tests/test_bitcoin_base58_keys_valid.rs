#[macro_use] extern crate assert_matches;
extern crate serde;
extern crate serde_json;
extern crate fiatproof;
use ::std::error::Error;

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
      Err(String::from("malformed test data"))
   } else {
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

macro_rules! fail {
   //fn fail(head:&str, t: &TestCase, description: &str) {
   ($head:expr, $t:expr, $description:expr) => {
      println!("");
      println!("FAIL: {}: {}", $head, $description);
      println!("  payload: {}", $t.payload_hexes);
      println!("  base58:  {}", $t.base58);
      assert!(false, "test failed");
   }
}

fn verify_privkey(t: &TestCase) {
   let b58c = t.key.chain.create_base58check_secret_key();
   let (skey_bytes, is_compressed) = {
      let dec  = ::fiatproof::crypto::secp256k1::secret_key::Base58checkDecoder::new(&b58c);
      let tmp = dec.decode_base58check(t.base58.as_str());
      if tmp.is_err() {
         fail!("parse_secret_key", t, "malformed bytes");
      }
      tmp.unwrap()
   };
   let _ = match (t.key.is_compressed, is_compressed) {
      (true,  false) => { fail!("is_compressed", t, "uncompressed"); },
      (false, true)  => { fail!("is_compressed", t, "compressed"); },
      _ => (),
   };
   let skey = {
      let dec = ::fiatproof::crypto::secp256k1::secret_key::RawDecoder::new();
      let tmp = dec.decode(&skey_bytes[0..32]);
      if let Err(ref e) = tmp {
         fail!("parse_secret_key", t, e.description());
      }
      tmp.unwrap()
   };
   let inner_bytes = &skey[..];
   if t.payload_bytes.as_ref() != inner_bytes {
      println!("expected: {:?}", t.payload_bytes.as_ref());
      println!("actual:   {:?}", inner_bytes);
      fail!("payload", t, "payload mismatch");
   }
}

fn verify_pubkey(t: &TestCase) {
   let payto = {
      let tmp = t.key.chain.parse_address(t.base58.as_str());
      if tmp.is_none() {
         fail!("parse_address", t, "unknown format");
      }
      tmp.unwrap()
   };
   let script = payto.compile();
   if t.payload_bytes.as_ref() != script.as_ref() {
      println!("expected: {:?}", t.payload_bytes.as_ref());
      println!("actual:   {:?}", script.as_ref());
      fail!("script", t, "script mismatch");
   }
   let flip_base58:String = t.base58.chars().map(|c| {
      if c.is_uppercase() {
         c.to_ascii_lowercase()
      } else if c.is_lowercase() {
         c.to_ascii_uppercase()
      } else {
         c
      }
   }).collect();
   let flip_payto = t.key.chain.parse_address(flip_base58.as_str());
   let _ = match (t.key.try_case_flip, flip_payto.is_none()) {
      (Some(true),  false) => { fail!("flip", t, "flip failed"); },
      (Some(false), true)  => { fail!("flip", t, "flip succeeded"); },
      (Some(false), false) => (),
      (Some(true), true)   => { //bech32
         let script = flip_payto.unwrap().compile();
         if t.payload_bytes.as_ref() != script.as_ref() {
            println!("expected: {:?}", t.payload_bytes.as_ref());
            println!("actual:   {:?}", script.as_ref());
            fail!("flip script", t, "script mismatch");
         }
      },
      (None, _) => (),
   };
   assert!(t.key.chain.parse_secret_key_base58check(t.base58.as_str()).is_err());
   assert!(t.key.chain.parse_secret_key_base58check(flip_base58.as_str()).is_err());
}

fn verify(t: &TestCase) {
   if t.key.is_privkey {
      verify_privkey(t);
   } else {
      if t.base58.chars().nth(0).unwrap() != '1' {
         return;
      }
      verify_pubkey(t);
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
