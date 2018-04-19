def_error! { Error }
def_error! { ParseError }
def_error! { InterpretError }

#[macro_export]
macro_rules! script_error {
   ($m:expr) => {
      ::bitcoin::script::Error::new($m, 0)
   }
}

#[macro_export]
macro_rules! raise_script_error {
   ($m:expr) => {
      try!( Err( script_error!($m) ) )
   }
}

#[macro_export]
macro_rules! parse_script_error {
   ($m:expr) => {
      ::bitcoin::script::ParseError::new($m, 0)
   }
}
#[macro_export]
macro_rules! raise_parse_script_error {
   ($m:expr) => {
      try!( Err( parse_script_error!($m) ))
   }
}

pub enum InterpretErrorCode {
   UnknownError,
   EvalFalse,
   OpReturn,

   /* Max sizes */
   ScriptSize,
   PushSize,
   OpCount,
   StackSize,
   SigCount,
   PubkeyCount,

   /* Failed verify operations */
   Verify,
   EqualVerify,
   CheckMultisigVerify,
   CheckSigVerify,
   NumEqualVerify,
   
   /* Logical/Format/Canonical errors */
   BadOpcode,
   DisabledOpcode,
   InvalidStackOperation,
   InvalidAltstackOperation,
   UnbalancedConditional,

   /* CHECKLOCKTIMEVERIFY and CHECKSEQUENCEVERIFY */
   NegativeLocktime,
   UnsatisfiedLocktime,

   /* Malleability */
   SigHashType,
   SigDer,
   MinimalData,
   SigPushOnly,
   SigHighS,
   SigNullDummy,
   PubkeyType,
   CleanStack,
   MinimalIf,
   SigNullFail,

   /* softfork safeness */
   DiscourageUpgradableNops,
   DiscourageUpgradableWitnessProgram,

   /* segregated witness */
   WitnessProgramWrongLength,
   WitnessProgramWitnessEmpty,
   WitnessProgramMismatch,
   WitnessMalleated,
   WitnessMalleatedP2sh,
   WitnessUnexpected,
   WitnessPubkeyType,

   ErrorCount,
}

impl InterpretError {
   pub fn is(&self, code: InterpretErrorCode) -> bool {
      self.code == code as u32
   }
}


#[macro_export]
macro_rules! script_interpret_error {
   ($c:tt) => {
      ::bitcoin::script::InterpretError::new(stringify!($c), ::bitcoin::script::error::InterpretErrorCode::$c as u32)
   };
   ($c:tt, $msg:expr) => {
      ::bitcoin::script::InterpretError::new(format!("{}: {}", stringify!($c), $msg),
                                             ::bitcoin::script::error::InterpretErrorCode::$c as u32)
   };
}
#[macro_export]
macro_rules! raise_script_interpret_error {
   ($c:tt) => {
      try!( Err( script_interpret_error!($c) ))
   }
}


