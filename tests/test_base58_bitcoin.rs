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
struct TestCase {
   pub lineno: usize,
   pub hexes:  String,
   pub bytes:  Vec<u8>,
   pub base58: String,
}

fn as_string<'a>(v: &'a ::serde_json::Value) -> Result<&'a String, &'static str> {
   match v {
      &::serde_json::Value::String(ref s) => Ok(s),
      _ => Err("not a string"),
   }
}

fn parse_testcase(v: &Vec<::serde_json::Value>, lineno:usize) -> Result<TestCase, String> {
   if v.len() != 2 {
      Err(String::from("malformed test data"))
   } else {
      use ::std::error::Error;
      let hexes  = as_string(&v[0])?;
      let bytes  = ::fiatproof::utils::h2b(hexes.as_str()).map_err(|e|e.description().to_string())?;
      let base58 = as_string(&v[1])?;
      Ok(TestCase {
         lineno: lineno, 
         hexes:  hexes.to_string(),
         bytes:  bytes,
         base58: base58.to_string(),
      })
   }
}

fn read_testcases() -> Result<Vec<TestCase>, String> {
   let path = "tests/bitcoin-test-data/base58_encode_decode.json";
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
      println!("  hexes:  {}", t.hexes);
      println!("  base58: {}", t.base58);
      assert!(false, "test failed");
   };
   {
      let e = ::fiatproof::bitcoin::utils::BASE58.encode(t.bytes.as_ref());
      if e != t.base58 {
         fail("encode", t, "mismatch");
      }
   }
   {
      let d = ::fiatproof::bitcoin::utils::BASE58.decode(t.base58.as_str());
      match d {
         Err(err) => {
            use ::std::error::Error;
            fail("decode", t, err.description());
         },
         Ok(expect) => {
            if expect.as_ref() != t.bytes.as_slice() {
               fail("decode", t, "mismatch");
            }
         }
      }
   }
   assert!(true);
}

#[test]
fn test_base58_bitcoin() {
   let r = read_testcases();
   assert_matches!(r, Ok(_));
   let tests = r.unwrap();

   for tc in &tests {
      verify(tc);
   }
}


