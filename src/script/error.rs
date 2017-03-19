def_error! { ScriptError }
def_error! { ParseScriptError }

#[macro_export]
macro_rules! script_error {
   ($m:expr) => {
      try!( Err(::script::ScriptError::new($m)) )
   }
}

#[macro_export]
macro_rules! parse_script_error {
   ($m:expr) => {
      try!( Err(::script::ParseScriptError::new($m)) )
   }
}


