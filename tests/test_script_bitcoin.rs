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
struct Witness {
   pub witnesses: Vec<String>,
   pub amount: ::serde_json::Number,
}

#[derive(Debug)]
struct TestData {
   pub lineno: usize,
   pub witness: Option< Witness >,
   pub script_sig: String,
   pub script_pubkey: String,
   pub flags: String,
   pub expect: String,
   pub comments: String,
}

#[derive(Debug)]
enum TestCase {
   Comment(String),
   T(TestData),
}

fn as_string<'a>(v: &'a ::serde_json::Value) -> Result<&'a String, &'static str> {
   match v {
      &::serde_json::Value::String(ref s) => Ok(s),
      _ => Err("not a string"),
   }
}
fn as_strings<'a>(v: &'a [::serde_json::Value]) -> Result<Vec<&'a String>, &'static str> {
   v.iter().fold(Ok(Vec::new()), |acc,item| {
      match acc {
         Err(e) => Err(e),
         Ok(mut a) => {
            match item {
               &::serde_json::Value::String(ref s) => {
                  a.push(s);
                  Ok(a)
               },
               _ => Err("not a string"),
            }
         }
      }
   })
}
fn as_strings_join<'a>(vv: &'a [::serde_json::Value]) -> Result<String, &'static str> {
   as_strings(vv).and_then(|v| {
      let s = v.iter().fold(String::new(), |mut acc, item| {
         acc.push_str(item.as_str());
         acc
      });
      Ok(s)
   })
}

fn parse_testcase(v: &Vec<::serde_json::Value>, lineno:usize) -> Result<TestCase, &'static str> {
   if v.len() == 1 {
      if let ::serde_json::Value::String(ref s) = v[0] {
         Ok(TestCase::Comment(s.clone()))
      } else {
         Err("unexpected comment type")
      }
   } else if let ::serde_json::Value::String(_) = v[0] {
      if v.len() < 4 {
         Err("no enough fields")
      } else {
         Ok(TestCase::T(TestData {
            lineno: lineno, 
            witness: None,
            script_sig: as_string(&v[0])?.clone(),
            script_pubkey: as_string(&v[1])?.clone(),
            flags: as_string(&v[2])?.clone(),
            expect: as_string(&v[3])?.clone(),
            comments: as_strings_join(&v[4..])?.clone(),
         }))
      }
   } else if let ::serde_json::Value::Array(ref v0) = v[0] {
      let len = v0.len();
      if len < 2 {
         Err("no enough witness fields")
      } else if let ::serde_json::Value::Number(ref n) = v0[len-1] {
         as_strings(&v0[0..(len-1)]).and_then(|witnesses| {
            Ok(TestCase::T(TestData {
               lineno: lineno,
               witness: Some(Witness {
                  witnesses: witnesses.into_iter().cloned().collect(),
                  amount: n.clone(),
               }),
               script_sig: as_string(&v[1])?.clone(),
               script_pubkey: as_string(&v[2])?.clone(),
               flags: as_string(&v[3])?.clone(),
               expect: as_string(&v[4])?.clone(),
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

fn read_testcases() -> Result<Vec<TestCase>, String> {
   let path = "tests/bitcoin-test-data/script_tests.json";
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

use ::fiatproof::bitcoin::script::Flags;
fn parse_flags(input:&str) -> Flags {
   let flags = Flags {
      script_verify: ::fiatproof::bitcoin::script::flags::ScriptVerify::default(),
      sig_version:   ::fiatproof::bitcoin::script::flags::SigVersion::WitnessV0,
   };
   input.split(',').fold(flags, |mut acc,s| {
      match s {
         "" => (), 
         "P2SH" => {
            acc.script_verify = acc.script_verify.p2sh(true);
         },
         "STRICTENC" => {
            acc.script_verify = acc.script_verify.strict_enc(true);
         },
         "DERSIG" => {
            acc.script_verify = acc.script_verify.der_sig(true);
         },
         "LOW_S" => {
            acc.script_verify = acc.script_verify.low_s(true);
         },
         "NULLDUMMY" => {
            acc.script_verify = acc.script_verify.null_dummy(true);
         },
         "SIGPUSHONLY" => {
            acc.script_verify = acc.script_verify.sig_push_only(true);
         },
         "MINIMALDATA" => {
            acc.script_verify = acc.script_verify.minimal_data(true);
         },
         "DISCOURAGE_UPGRADABLE_NOPS" => {
            acc.script_verify = acc.script_verify.discourage_upgradable_nops(true);
         },
         "CLEANSTACK" => {
            acc.script_verify = acc.script_verify.clean_stack(true);
         },
         "CHECKLOCKTIMEVERIFY" => {
            acc.script_verify = acc.script_verify.check_locktime_verify(true);
         },
         "CHECKSEQUENCEVERIFY" => {
            acc.script_verify = acc.script_verify.check_sequence_verify(true);
         },
         "WITNESS" => {
            acc.script_verify = acc.script_verify.witness(true);
         },
         "DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM" => {
            acc.script_verify = acc.script_verify.discourage_upgradable_witness_program(true);
         },
         "MINIMALIF" => {
            acc.script_verify = acc.script_verify.minimal_if(true);
         },
         "NULLFAIL" => {
            acc.script_verify = acc.script_verify.null_fail(true);
         },
         "WITNESS_PUBKEYTYPE" => {
            acc.script_verify = acc.script_verify.witness_pubkey_type(true);
         },
         _ => {
            assert!(false, format!("  unknown flags {}", s));
         }
      }
      acc
   })
}

fn check_verify_result(result: ::fiatproof::Result<()>, t: &TestData, tx: &::fiatproof::bitcoin::Tx) {
   use std::error::Error; //description()
   let fail = | head:&str, t: &TestData, r: &::fiatproof::Result<()> | {
      let description = match r {
         &Ok(_) => "OK",
         &Err(ref e) => e.description().clone(),
      };
      println!("");
      if let Err(::fiatproof::Error::BitcoinInterpretScript(ref e)) = result {
         println!("{}", e.backtrace);
      }
      println!("FAIL: {}", head);
      println!("  comment={}", t.comments);
      println!("  sig='{}'", t.script_sig);
      println!("  key='{}'", t.script_pubkey);
      println!("   verify fail: expect {} but {}", t.expect, description);
      println!("credit.txid = {}", ::fiatproof::ui::b2h(&tx.ins[0].prevout.txid.data[..]));
      println!("spending = {}", ::fiatproof::ui::bitcoin::tx_to_hex(&tx).unwrap());
      assert!(false, "verify failed");
   };
   use ::fiatproof::Error::BitcoinInterpretScript as IS;
   use ::fiatproof::bitcoin::script::InterpretErrorCode as C;
   match (t.expect.as_str(), &result) {
      ("OK", &Ok(_)) => (),
      ("UNKNOWN_ERROR", &Err(IS(_))) => { fail("", t, &result); },
      ("UNKNOWN_ERROR", &Err(_)) => (),
      ("EVAL_FALSE", &Err(IS(ref e))) if e.is(C::EvalFalse) => (),
      ("OP_RETURN", &Err(IS(ref e))) if e.is(C::OpReturn) => (),
      
      ("SCRIPT_SIZE", &Err(IS(ref e))) if e.is(C::ScriptSize) => (),
      ("PUSH_SIZE", &Err(IS(ref e))) if e.is(C::PushSize) => (),
      ("OP_COUNT", &Err(IS(ref e))) if e.is(C::OpCount) => (),
      ("STACK_SIZE", &Err(IS(ref e))) if e.is(C::StackSize) => (),
      ("SIG_COUNT", &Err(IS(ref e))) if e.is(C::SigCount) => (),
      ("PUBKEY_COUNT", &Err(IS(ref e))) if e.is(C::PubkeyCount) => (),
      
      ("VERIFY", &Err(IS(ref e))) if e.is(C::Verify) => (),
      ("EQUALVERIFY", &Err(IS(ref e))) if e.is(C::EqualVerify) => (),

      //("CHECKMULTISIGVERIFY", &Err(IS(ref e))) if e.is(C::CheckMultisigVerify) => (),
      //("CHECKSIGVERIFY", &Err(IS(ref e))) if e.is(C::CheckSigVerify) => (),
      //("NUMEQUALVERIFY", &Err(IS(ref e))) if e.is(C::NumEqualVerify) => (),

      ("BAD_OPCODE", &Err(IS(ref e))) if e.is(C::BadOpcode) => (),
      ("DISABLED_OPCODE", &Err(IS(ref e))) if e.is(C::DisabledOpcode) => (),
      ("INVALID_STACK_OPERATION", &Err(IS(ref e))) if e.is(C::InvalidStackOperation) => (),
      ("INVALID_ALTSTACK_OPERATION", &Err(IS(ref e))) if e.is(C::InvalidAltstackOperation) => (),
      ("UNBALANCED_CONDITIONAL", &Err(IS(ref e))) if e.is(C::UnbalancedConditional) => (),

      ("NEGATIVE_LOCKTIME", &Err(IS(ref e))) if e.is(C::NegativeLocktime) => (),
      ("UNSATISFIED_LOCKTIME", &Err(IS(ref e))) if e.is(C::UnsatisfiedLocktime) => (),
      
      ("SIG_DER", &Err(IS(ref e))) if e.is(C::SigDer) => (),
      ("SIG_HASHTYPE", &Err(IS(ref e))) if e.is(C::SigHashType) => (),
      ("MINIMALDATA", &Err(IS(ref e))) if e.is(C::MinimalData) => (),
      ("SIG_PUSHONLY", &Err(IS(ref e))) if e.is(C::SigPushOnly) => (),
      ("SIG_HIGH_S", &Err(IS(ref e))) if e.is(C::SigHighS) => (),
      ("SIG_NULLDUMMY", &Err(IS(ref e))) if e.is(C::SigNullDummy) => (),
      ("PUBKEYTYPE", &Err(IS(ref e))) if e.is(C::PubkeyType) => (),
      ("CLEANSTACK", &Err(IS(ref e))) if e.is(C::CleanStack) => (),
      ("MINIMALIF", &Err(IS(ref e))) if e.is(C::MinimalIf) => (),
      ("NULLFAIL", &Err(IS(ref e))) if e.is(C::SigNullFail) => (),
      
      ("DISCOURAGE_UPGRADABLE_NOPS", &Err(IS(ref e))) if e.is(C::DiscourageUpgradableNops) => (),
      ("DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM", &Err(IS(ref e))) if e.is(C::DiscourageUpgradableWitnessProgram) => (),
      
      ("WITNESS_PROGRAM_WRONG_LENGTH", &Err(IS(ref e))) if e.is(C::WitnessProgramWrongLength) => (),
      ("WITNESS_PROGRAM_WITNESS_EMPTY", &Err(IS(ref e))) if e.is(C::WitnessProgramWitnessEmpty) => (),
      ("WITNESS_PROGRAM_MISMATCH", &Err(IS(ref e))) if e.is(C::WitnessProgramMismatch) => (),
      ("WITNESS_MALLEATED", &Err(IS(ref e))) if e.is(C::WitnessMalleated) => (),
      ("WITNESS_MALLEATED_P2SH", &Err(IS(ref e))) if e.is(C::WitnessMalleatedP2sh) => (),
      ("WITNESS_UNEXPECTED", &Err(IS(ref e))) if e.is(C::WitnessUnexpected) => (),
      ("WITNESS_PUBKEYTYPE", &Err(IS(ref e))) if e.is(C::WitnessPubkeyType) => (),

      //("ERROR_COUNT", &Err(IS(ref e))) if e.is(C::ErrorCount) => (),
      
      (_, &Ok(_)) => { fail("", t, &result); },
      (_, &Err(_)) => { fail("", t, &result); },
   }
   assert!(true);
}

fn build_test_transaction(script_pubkey:&[u8], script_sig:&[u8]) -> (Vec<::fiatproof::bitcoin::Tx>, ::fiatproof::bitcoin::Tx) {
   use ::fiatproof::bitcoin::datatypes::*;
   let utx = {
      let mut tx = Tx::new_null();
      tx.version = 1;
      tx.locktime = LockTime::NoLock;
      tx.ins.push(TxIn {
         prevout:    TxOutPoint::new_null(),
         script_sig: Script::new( ::fiatproof::bitcoin::script::compile("0 0").unwrap() ),
         sequence:   TxIn::SEQUENCE_FINAL,
      });
      tx.outs.push(TxOut {
         value: 0,
         script_pubkey: Script::new(script_pubkey),
      });
      tx
   };
   let tx = {
      let mut tx = Tx::new_null();
      tx.version = 1;
      tx.locktime = LockTime::NoLock;
      tx.ins.push(TxIn {
         prevout:    TxOutPoint {
            txid: utx.get_hash().unwrap(),
            n:    0,
         },
         script_sig: Script::new(script_sig),
         sequence:   TxIn::SEQUENCE_FINAL,
      });
      tx.outs.push(TxOut {
         value: utx.outs[0].value,
         script_pubkey: Script::new_null(),
      });
      tx
   };
   (vec![utx], tx)
}

#[test]
fn test_script_bitcoin() {
   let r = read_testcases();
   assert_matches!(r, Ok(_));
   let tests = r.unwrap();

   let compile = |s:&str| {
      use ::fiatproof::bitcoin::script::compile;
      let r = compile(s);
      if r.is_err() {
         use std::error::Error;
         assert!(false, format!("  compile fail: script=\"{}\", err={}", s, r.unwrap_err().description()));
      }
      r.unwrap()
   };
   let verify = |sig:&[u8], pk:&[u8], flags:&Flags, t: &TestData| {
      if flags.script_verify.is_witness() {
         return;
      }
      use ::fiatproof::bitcoin::script::verify;
      let tx = build_test_transaction(pk, sig).1;
      let r = verify(sig, pk, &tx, 0, flags);
      check_verify_result(r, t, &tx);
   };
   let mut _last_comment = String::new();
   for tc in tests {
      match tc {
         TestCase::Comment(c) => {
            _last_comment = c.clone();
         },
         TestCase::T(ref t) if t.witness.is_none() => {
            let script_sig = compile(&t.script_sig);
            let script_pk  = compile(&t.script_pubkey);
            let flags = parse_flags(&t.flags);
            verify(script_sig.as_slice(), script_pk.as_slice(), &flags, &t);
         },
         TestCase::T(ref t) if t.witness.is_some() => (),
         _ => {
            panic!("unknown testcase");
         }
      }
   }
}


