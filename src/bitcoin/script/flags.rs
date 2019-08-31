#[derive(Debug,Clone,Copy)]
pub enum SigVersion {
   Base,
   WitnessV0,
}
impl Default for SigVersion {
   fn default() -> Self {
      SigVersion::Base
   }
}
impl SigVersion {
   pub fn is_base(&self) -> bool {
      match *self {
         SigVersion::Base => true,
         _ => false,
      }
   }
   pub fn is_witness_v0(&self) -> bool {
      match *self {
         SigVersion::WitnessV0 => true,
         _ => false,
      }
   }
}

#[derive(Debug,Clone,Copy)]
pub struct ScriptVerify(pub u32);

impl Default for ScriptVerify {
   fn default() -> Self { ScriptVerify(0) }
}

macro_rules! impl_flags {
   ($isfname:ident, $fname:ident, $x:expr) => {
      #[inline] pub fn $isfname(&self) -> bool {
         (self.0 & (1u32 << $x)) != 0
      }
      #[inline] pub fn $fname(&self, v:bool) -> Self {
         ScriptVerify(if v {
            self.0 | (1u32 << $x)
         } else {
            self.0 & !(1u32 << $x)
         })
      }
   }
}
impl ScriptVerify {
   #[inline] pub fn is_none(&self) -> bool { self.0 == 0 }
   
   impl_flags!{is_p2sh, p2sh, 0}
   impl_flags!{is_strict_enc, strict_enc, 1}
   impl_flags!{is_der_sig, der_sig, 2}
   impl_flags!{is_low_s,low_s, 3}
   impl_flags!{is_null_dummy, null_dummy, 4}
   impl_flags!{is_sig_push_only, sig_push_only, 5}
   impl_flags!{is_minimal_data, minimal_data, 6}
   impl_flags!{is_discourage_upgradable_nops, discourage_upgradable_nops, 7}
   impl_flags!{is_clean_stack, clean_stack, 8}
   impl_flags!{is_check_locktime_verify, check_locktime_verify, 9}
   impl_flags!{is_check_sequence_verify, check_sequence_verify, 10}
   impl_flags!{is_witness, witness, 11}
   impl_flags!{is_discourage_upgradable_witness_program, discourage_upgradable_witness_program, 12}
   impl_flags!{is_minimal_if, minimal_if, 13}
   impl_flags!{is_null_fail, null_fail, 14}
   impl_flags!{is_witness_pubkey_type, witness_pubkey_type, 15}
   
   pub fn with<O,F>(&self, f:F) -> O where F: Fn(&Self)->O {
      f(self)
   }
   
   // policy/policy.h
   #[inline] pub fn is_mandatory(&self) -> bool {
      self.is_p2sh()
   }
   #[inline] pub fn is_standard_not_mandatory(&self) -> bool {
      self.with(|f|
               f.is_der_sig()
               && f.is_strict_enc()
               && f.is_minimal_data()
               && f.is_null_dummy()
               && f.is_discourage_upgradable_nops()
               && f.is_clean_stack()
               && f.is_check_locktime_verify()
               && f.is_check_sequence_verify()
               && f.is_low_s())
   }
   #[inline] pub fn is_standard(&self) -> bool {
      self.with(|f| f.is_mandatory() && f.is_standard_not_mandatory())
   }

      // interpreter.cpp
   #[inline] pub fn is_require_minimal(&self) -> bool {
      self.is_minimal_data()
   }
}

#[derive(Debug,Clone,Copy,Default)]
pub struct Flags {
   pub script_verify: super::flags::ScriptVerify,
//   pub sig_version:   super::flags::SigVersion,
}
