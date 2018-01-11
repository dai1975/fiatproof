#[macro_use] extern crate assert_matches;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rsbitcoin;

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
struct Witness {
   pub witnesses: Vec<String>,
   pub amount: serde_json::Number,
}

#[derive(Debug)]
struct TestData {
   pub lineno: usize,
   pub witness: Option< Witness >,
   pub scriptSig: String,
   pub scriptPubKey: String,
   pub flags: String,
   pub expected_scripterror: String,
   pub comments: String,
}

#[derive(Debug)]
enum TestCase {
   Comment(String),
   T(TestData),
}

fn as_string<'a>(v: &'a serde_json::Value) -> Result<&'a String, &'static str> {
   match v {
      &serde_json::Value::String(ref s) => Ok(s),
      _ => Err("not a string"),
   }
}
fn as_strings<'a>(v: &'a [serde_json::Value]) -> Result<Vec<&'a String>, &'static str> {
   v.iter().fold(Ok(Vec::new()), |acc,item| {
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

fn parse_testcase(v: &Vec<serde_json::Value>, lineno:usize) -> Result<TestCase, &'static str> {
   if v.len() == 1 {
      if let serde_json::Value::String(ref s) = v[0] {
         Ok(TestCase::Comment(s.clone()))
      } else {
         Err("unexpected comment type")
      }
   } else if let serde_json::Value::String(_) = v[0] {
      if v.len() < 4 {
         Err("no enough fields")
      } else {
         Ok(TestCase::T(TestData {
            lineno: lineno, 
            witness: None,
            scriptSig: as_string(&v[0])?.clone(),
            scriptPubKey: as_string(&v[1])?.clone(),
            flags: as_string(&v[2])?.clone(),
            expected_scripterror: as_string(&v[3])?.clone(),
            comments: as_strings_join(&v[4..])?.clone(),
         }))
      }
   } else if let serde_json::Value::Array(ref v0) = v[0] {
      let len = v0.len();
      if len < 2 {
         Err("no enough witness fields")
      } else if let serde_json::Value::Number(ref n) = v0[len-1] {
         as_strings(&v0[0..(len-1)]).and_then(|witnesses| {
            Ok(TestCase::T(TestData {
               lineno: lineno,
               witness: Some(Witness {
                  witnesses: witnesses.into_iter().cloned().collect(),
                  amount: n.clone(),
               }),
               scriptSig: as_string(&v[1])?.clone(),
               scriptPubKey: as_string(&v[2])?.clone(),
               flags: as_string(&v[3])?.clone(),
               expected_scripterror: as_string(&v[4])?.clone(),
               comments: as_strings_join(&v[5..])?.clone(),
            }))
         })
      } else {
         Err("no witness amount")
      }
   } else {
      Err("unexpected format")
   }
}

fn read_testcases() -> Result<Vec<TestData>, String> {
   println!("cwd={}", ::std::env::current_dir().unwrap().display());
   let path = "tests/bitcoin-test-data/script_tests.json";
   let f = ::std::fs::File::open(path).unwrap();
   let lines:Vec< Vec<serde_json::Value> > = serde_json::from_reader(f).unwrap();
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
               Ok(TestCase::Comment(_)) => Ok(v),
               Ok(TestCase::T(d)) => {
                  v.push(d);
                  Ok(v)
               }
            }
         }
      }
   })
}

use rsbitcoin::script::interpreter::Flags;
fn parse_flags(input:&str, lineno:usize) -> Flags {
   let flags = Flags {
      script_verify: rsbitcoin::script::flags::ScriptVerify::default(),
      sig_version:   rsbitcoin::script::flags::SigVersion::WitnessV0,
   };
   input.split(',').fold(flags, |mut acc,s| {
      match s {
         "P2SH" => {
            acc.script_verify = acc.script_verify.p2sh(true);
         },
         "STRICTENC" => {
            acc.script_verify = acc.script_verify.strict_enc(true);
         },
         _ => {
            assert!(false, format!("test {}: unknown flags {}", lineno, s));
         }
      }
      acc
   })
}

#[test]
fn test_script_bitcoin() {
   let r = read_testcases();
   assert_matches!(r, Ok(_));
   let tests = r.unwrap();

   let dump = |head:&str, bytes:&[u8]| {
      use std::fmt::Write;
      let mut s = String::new();
      for b in bytes.into_iter() {
         write!(&mut s, "{:x} ", b);
      }
      println!("{}: {}", head, s);
   };
   
   let compile = |s:&str, line| {
      use rsbitcoin::script::compile;
      let r = compile(s);
      if r.is_err() {
         use std::error::Error;
         assert!(false, format!("test {}: script=\"{}\", err={}", line, s, r.unwrap_err().description()));
      }
      r.unwrap()
   };
   let verify = |sig:&[u8], pk:&[u8], flags:&Flags, line, src_sig:&str, src_pk:&str| {
      dump("sig", sig); dump("pk", pk);
      use rsbitcoin::script::verify;
      let tx = rsbitcoin::Tx::default();
      let r = verify(sig, pk, &tx, 0, flags);
      if r.is_err() {
         use std::error::Error;
         assert!(false, format!("test {}: sig=\"{}\", pk=\"{}\", err={}", line, src_sig, src_pk, r.unwrap_err().description()));
      }
   };
   for t in tests {
      let script_sig = compile(&t.scriptSig, t.lineno);
      let script_pk  = compile(&t.scriptPubKey, t.lineno);
      let flags = parse_flags(&t.flags, t.lineno);
      verify(script_sig.as_slice(), script_pk.as_slice(), &flags, t.lineno, &t.scriptSig, &t.scriptPubKey);
   }
}


