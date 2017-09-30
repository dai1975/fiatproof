/*
// Constants
pub struct Push<T: Borrow<[u8]>>{ pub data:T }

// Flow control
pub struct Nop;
pub struct If;
pub struct NotIf;
pub struct Else;
pub struct EndIf;
pub struct Return;

// Stack
pub struct TotalStack;
pub struct FromStack;
pub struct Drop2;
pub struct Dup2;
pub struct Dup3;
pub struct Over2;
pub struct Rot2;
pub struct Swap2;
pub struct IfDup;
pub struct Depth;
pub struct Drop;
pub struct Dup;
pub struct Nip;
pub struct Over;
pub struct Pick;
pub struct Roll;
pub struct Rot;
pub struct Swap;
pub struct Tuck;

// Splice
pub struct Cat;
pub struct Substr;
pub struct Left;
pub struct Right;
pub struct Size;

// Bitwise logic
pub struct Invert;
pub struct And;
pub struct Or;
pub struct Xor;
pub struct Equal{ pub verify:bool }

// Arithmetic
pub struct Add1;
pub struct Sub1;
pub struct Mul2;
pub struct Div2;
pub struct Negate;
pub struct Abs;
pub struct Not;
pub struct NotEqual0;
pub struct Add;
pub struct Sub;
pub struct Mul;
pub struct Div;
pub struct Mod;
pub struct LShift;
pub struct RShift;
pub struct BoolAnd;
pub struct BoolOr;
pub struct NumEqual;
pub struct NumEqualVerify;
pub struct NumNotEqual;
pub struct LessThan;
pub struct GreaterThan;
pub struct LEssThanOrEqual;
pub struct GreaterThanOrEqual;
pub struct Min;
pub struct Max;
pub struct Within;

//crypto
pub struct Ripemd160;
pub struct Sha1;
pub struct Sha256;
pub struct Hash160;
pub struct Hash256;
pub struct CodeSeparator;
pub struct CheckSig { pub verify:bool }
pub struct CheckMultisig { pub verify:bool }

// Locktime
pub struct CheckLockTimeVerify;
pub struct CheckSequenceVerify;

// template matching params
pub struct SmallInteger;
pub struct PubKeys;
pub struct PubKeyHash;
pub struct PubKey;
*/

pub enum Instruction<'a> {
   PushData{ data:&'a [u8] },
   PushValue{ value:u64 },

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
}

#[test]
fn test_infoarray() {
   assert_eq!(256, OPCODE_INFO.len());
}


