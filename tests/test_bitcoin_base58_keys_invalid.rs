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
   pub lineno:  usize,
   pub base58:  String,
}

fn as_string<'a>(v: &'a ::serde_json::Value) -> Result<&'a String, String> {
   match v {
      &::serde_json::Value::String(ref s) => Ok(s),
      _ => Err(String::from("not a string")),
   }
}

fn parse_testcase(v: &Vec<::serde_json::Value>, lineno:usize) -> Result<TestCase, String> {
   if v.len() != 1 {
      Err(String::from("malformed test data"))
   } else {
      let base58  = as_string(&v[0])?;
      Ok(TestCase {
         lineno: lineno,
         base58: base58.to_string(),
      })
   }
}

fn read_testcases() -> Result<Vec<TestCase>, String> {
   let path = "tests/bitcoin-test-data/base58_keys_invalid.json";
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
      println!("  base58:  {}", $t.base58);
      assert!(false, "test failed");
   }
}

fn verify(t: &TestCase) {
   let s = t.base58.as_str();
   let chains:[&::fiatproof::ui::bitcoin::Chain; 3] = [
      &::fiatproof::ui::bitcoin::MAINNET,
      &::fiatproof::ui::bitcoin::TESTNET,
      &::fiatproof::ui::bitcoin::REGTEST,
   ];
   for c in chains.iter() {
      if c.parse_address(s).is_some() {
         fail!("parse_address", t, c.params.network);
      }
      if c.parse_secret_key_base58check(s).is_ok() {
         fail!("parse_secret_key", t, c.params.network);
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
