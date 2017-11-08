use super::pushee::Pushee;
use super::opcode::*;

pub enum Instruction<'a> {
   Push(Pushee<'a>),
   Op(u8),
/*
   // Constats
   Nop,
   If,
   NotIf,
   Else,
   EndIf,
   Return,

   // Stack
   TotalStack,
   FromStack,
   Drop2,
   Dup2,
   Dup3,
   Over2,
   Rot2,
   Swap2,
   IfDup,
   Depth,
   Drop,
   Dup,
   Nip,
   Over,
   Pick,
   Roll,
   Rot,
   Swap,
   Tuck,

// Splice
   Cat,
   Substr,
   Left,
   Right,
   Size,

// Bitwise logic
   Invert,
   And,
   Or,
   Xor,
   Equal,

// Arithmetic
   Add1,
   Sub1,
   Mul2,
   Div2,
   Negate,
   Abs,
   Not,
   NotEqual0,
   Add,
   Sub,
   Mul,
   Div,
   Mod,
   LShift,
   RShift,
   BoolAnd,
   BoolOr,
   NumEqual,
   NumEqualVerify,
   NumNotEqual,
   LessThan,
   GreaterThan,
   LEssThanOrEqual,
   GreaterThanOrEqual,
   Min,
   Max,
   Within,

//crypto
   Ripemd160,
   Sha1,
   Sha256,
   Hash160,
   Hash256,
   CodeSeparator,
   CheckSig,
   CheckSigVerify,
   CheckMultisig,
   CheckMultisigVerify,

// Locktime
   CheckLockTimeVerify,
   CheckSequenceVerify,

// template matching params
   SmallInteger,
   PubKeys,
   PubKeyHash,
   PubKey,
*/
}

impl <'a> ::std::fmt::Display for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Push(ref x) => x.fmt(f),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}
impl <'a> ::std::fmt::Debug for Instruction<'a> {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      match self {
         &Instruction::Push(ref x) => x.fmt(f),
         &Instruction::Op(code) => write!(f, "{}", OPCODE_INFO[code as usize].name),
         _ => write!(f, "unimplemented"),
      }
   }
}

#[test]
fn test_infoarray() {
   //assert_eq!(256, OPCODE_INFO.len());
}


