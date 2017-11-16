#[macro_use] extern crate assert_matches;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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
impl_error!( serde_json::error::Error );

#[derive(Debug)]
struct Witness<'a> {
   pub witnesses: Vec<&'a String>,
   pub amount: &'a serde_json::Number,
}

#[derive(Debug)]
struct TestData<'a> {
   pub witness: Option< Witness<'a> >,
   pub scriptSig: &'a String,
   pub scriptPubKey: &'a String,
   pub flags: &'a String,
   pub expected_scripterror: &'a String,
   pub comments: String,
}

#[derive(Debug)]
enum TestCase<'a> {
   Comment(&'a String),
   T(TestData<'a>),
}

fn as_string<'a>(v: &'a serde_json::Value) -> Result<&'a String, &'static str> {
   match v {
      &serde_json::Value::String(ref s) => Ok(s),
      _ => Err("not a string"),
   }
}
fn as_strings<'a>(v: &'a [serde_json::Value]) -> Result<Vec<&'a String>, &'static str> {
   v.iter().fold(Ok(Vec::new()), |mut acc,item| {
      match acc {
         Err(e) => Err(e),
         Ok(mut a) => {
            match item {
               &serde_json::Value::String(ref s) => {
                  a.push(s);
                  Ok(a)
               },
               _ => Err("not a string"),
            }
         }
      }
   })
}
fn as_strings_join<'a>(vv: &'a [serde_json::Value]) -> Result<String, &'static str> {
   as_strings(vv).and_then(|v| {
      let s = v.iter().fold(String::new(), |mut acc, item| {
         acc.push_str(item.as_str());
         acc
      });
      Ok(s)
   })
}

fn parse_testcase<'a>(v: &'a Vec<serde_json::Value>) -> Result<TestCase<'a>, &'static str> {
   if v.len() == 1 {
      if let serde_json::Value::String(ref s) = v[0] {
         Ok(TestCase::Comment(s))
      } else {
         Err("unexpected comment type")
      }
   } else if let serde_json::Value::String(_) = v[0] {
      if v.len() < 4 {
         Err("no enough fields")
      } else {
         Ok(TestCase::T(TestData {
            witness: None,
            scriptSig: as_string(&v[0])?,
            scriptPubKey: as_string(&v[1])?,
            flags: as_string(&v[2])?,
            expected_scripterror: as_string(&v[3])?,
            comments: as_strings_join(&v[4..])?,
         }))
      }
   } else if let serde_json::Value::Array(ref v0) = v[0] {
      let len = v0.len();
      if len < 2 {
         Err("no enough witness fields")
      } else if let serde_json::Value::Number(ref n) = v0[len-1] {
         as_strings(&v0[0..(len-1)]).and_then(|witnesses| {
            Ok(TestCase::T(TestData {
               witness: Some(Witness {
                  witnesses: witnesses,
                  amount: n,
               }),
               scriptSig: as_string(&v[1])?,
               scriptPubKey: as_string(&v[2])?,
               flags: as_string(&v[3])?,
               expected_scripterror: as_string(&v[4])?,
               comments: as_strings_join(&v[5..])?,
            }))
         })
      } else {
         Err("no witness amount")
      }
   } else {
      Err("unexpected format")
   }
}

fn read_testcases() {
   println!("cwd={}", ::std::env::current_dir().unwrap().display());
   let path = "tests/bitcoin-test-data/script_tests.json";
   let f = ::std::fs::File::open(path).unwrap();
   let tests:Vec< Vec<serde_json::Value> > = serde_json::from_reader(f).unwrap();
   for (n, test) in tests.iter().enumerate() {
      let r = parse_testcase(test);
      match r {
         Err(msg) => {
            let msg = format!("{} at {}: {:?}", msg, n, test);
            assert!(false, msg);
         }
         Ok(TestCase::Comment(_)) => (),
         Ok(TestCase::T(_)) => (),
      }
   }
}

#[test]
fn test_script_bitcoin() {
   read_testcases();
}
