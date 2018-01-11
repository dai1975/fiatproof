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
fn parse_expect(s:&str) -> Result<(),u32> {
   use rsbitcoin::script::error::InterpretErrorCode;
   match s {
      "OK" => Ok(()),
      "UNKNOWN_ERROR" => Err(InterpretErrorCode::UnknownError as u32),
      "EVAL_FALSE" => Err(InterpretErrorCode::EvalFalse as u32),
      "OP_RETURN" => Err(InterpretErrorCode::OpReturn as u32),
      "SCRIPT_SIZE" => Err(InterpretErrorCode::ScriptSize as u32),
      "PUSH_SIZE" => Err(InterpretErrorCode::PushSize as u32),
      "OP_COUNT" => Err(InterpretErrorCode::OpCount as u32),
      "STACK_SIZE" => Err(InterpretErrorCode::StackSize as u32),
      "SIG_COUNT" => Err(InterpretErrorCode::SigCount as u32),
      "PUBKEY_COUNT" => Err(InterpretErrorCode::PubkeyCount as u32),
      "VERIFY" => Err(InterpretErrorCode::Verify as u32),
      "EQUALVERIFY" => Err(InterpretErrorCode::EqualVerify as u32),
      "CHECKMULTISIGVERIFY" => Err(InterpretErrorCode::CheckMultisigVerify as u32),
      "CHECKSIGVERIFY" => Err(InterpretErrorCode::CheckSigVerify as u32),
      "NUMEQUALVERIFY" => Err(InterpretErrorCode::NumEqualVerify as u32),
      "BAD_OPCODE" => Err(InterpretErrorCode::BadOpcode as u32),
      "DISABLED_OPCODE" => Err(InterpretErrorCode::DisabledOpcode as u32),
      "INVALID_STACK_OPERATION" => Err(InterpretErrorCode::InvalidStackOperation as u32),
      "INVALID_ALTSTACK_OPERATION" => Err(InterpretErrorCode::InvalidAltstackOperation as u32),
      "UNBALANCED_CONDITIONAL" => Err(InterpretErrorCode::UnbalancedConditional as u32),
      "NEGATIVE_LOCKTIME" => Err(InterpretErrorCode::NegativeLocktime as u32),
      "UNSATISFIED_LOCKTIME" => Err(InterpretErrorCode::UnsatisfiedLocktime as u32),
      "SIG_HASHTYPE" => Err(InterpretErrorCode::SigHashType as u32),
      "SIG_DER" => Err(InterpretErrorCode::SigDer as u32),
      "MINIMALDATA" => Err(InterpretErrorCode::MinimalData as u32),
      "SIG_PUSHONLY" => Err(InterpretErrorCode::SigPushOnly as u32),
      "SIG_HIGH_S" => Err(InterpretErrorCode::SigHighS as u32),
      "SIG_NULLDUMMY" => Err(InterpretErrorCode::SigNullDummy as u32),
      "PUBKEYTYPE" => Err(InterpretErrorCode::PubkeyType as u32),
      "CLEANSTACK" => Err(InterpretErrorCode::CleanStack as u32),
      "MINIMALIF" => Err(InterpretErrorCode::MinimalIf as u32),
      "SIG_NULLFAIL" => Err(InterpretErrorCode::SigNullFail as u32),
      "DISCOURAGE_UPGRADABLE_NOPS" => Err(InterpretErrorCode::DiscourageUpgradableNops as u32),
      "UPGRADABLE_WITNESS_PROGRAM" => Err(InterpretErrorCode::DiscourageUpgradableWitnessProgram as u32),
      "WITNESS_PROGRAM_WRONG_LENGTH" => Err(InterpretErrorCode::WitnessProgramWrongLength as u32),
      "PROGRAM_WITNESS_EMPTY" => Err(InterpretErrorCode::WitnessProgramWitnessEmpty as u32),
      "PROGRAM_MISMATCH" => Err(InterpretErrorCode::WitnessProgramMismatch as u32),
      "MALLEATED" => Err(InterpretErrorCode::WitnessMalleated as u32),
      "MALLEATED_P2SH" => Err(InterpretErrorCode::WitnessMalleatedP2sh as u32),
      "UNEXPECTED" => Err(InterpretErrorCode::WitnessUnexpected as u32),
      "PUBKEYTYPE" => Err(InterpretErrorCode::WitnessPubkeyType as u32),
      "ERROR_COUNT" => Err(InterpretErrorCode::ErrorCount as u32),
      _ => {
         panic!(format!("unknown error code: {}", s));
         Ok(())
      }
   }
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
   let verify = |sig:&[u8], pk:&[u8], flags:&Flags, expect:&Result<(),u32>, line, src_sig:&str, src_pk:&str| {
      dump("sig", sig); dump("pk", pk);
      use rsbitcoin::script::verify;
      let tx = rsbitcoin::Tx::default();
      let r = verify(sig, pk, &tx, 0, flags);
      use std::error::Error;
      let headmsg = format!("test {}: sig=\"{}\", pk=\"{}\"", line, src_sig, src_pk);
      match (r, *expect) {
         (Ok(()), Ok(())) => (),
         (Ok(()), Err(ecode)) => {
            assert!(false, 
                    format!("{}, err={} but {}", headmsg, "OK", ecode));
         },
         (Err(rsbitcoin::Error::InterpretScript(re)), Ok(())) => {
            format!("{}, err={} but {}", headmsg, re.code, "OK");
         },
         (Err(rsbitcoin::Error::InterpretScript(re)), Err(ecode)) => {
            assert!(re.code == ecode,
                    format!("{}, err={} but {}", headmsg, re.code, ecode));
         },
         (Err(e), _) => {
            assert!(false,
                    format!("{}, err={}", headmsg, e.description()));
         },
      }
   };
   for t in tests {
      let script_sig = compile(&t.scriptSig, t.lineno);
      let script_pk  = compile(&t.scriptPubKey, t.lineno);
      let flags = parse_flags(&t.flags, t.lineno);
      let expect = parse_expect(&t.expected_scripterror);
      verify(script_sig.as_slice(), script_pk.as_slice(), &flags, &expect, t.lineno, &t.scriptSig, &t.scriptPubKey);
   }
}


