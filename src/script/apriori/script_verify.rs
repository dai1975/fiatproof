#[allow(dead_code)] pub const NONE:u32                       = 0u32;
#[allow(dead_code)] pub const P2SH:u32                       = (1u32 << 0);
#[allow(dead_code)] pub const STRICTENC:u32                  = (1u32 << 1);
#[allow(dead_code)] pub const DERSIG:u32                     = (1u32 << 2);
#[allow(dead_code)] pub const LOW_S:u32                      = (1u32 << 3);
#[allow(dead_code)] pub const NULLDUMMY:u32                  = (1u32 << 4);
#[allow(dead_code)] pub const SIGPUSHONLY:u32                = (1u32 << 5);
#[allow(dead_code)] pub const MINIMALDATA:u32                = (1u32 << 6);
#[allow(dead_code)] pub const DISCOURAGE_UPGRADABLE_NOPS:u32 = (1u32 << 7);
#[allow(dead_code)] pub const CLEANSTACK:u32                 = (1u32 << 8);
#[allow(dead_code)] pub const CHECKLOCKTIMEVERIFY:u32        = (1u32 << 9);
#[allow(dead_code)] pub const CHECKSEQUENCEVERIFY:u32        = (1u32 << 10);


#[allow(dead_code)] pub const MANDATORY:u32
   = P2SH;

#[allow(dead_code)] pub const STANDARD:u32
   = MANDATORY
   | DERSIG
   | STRICTENC
   | MINIMALDATA
   | NULLDUMMY
   | DISCOURAGE_UPGRADABLE_NOPS
   | CLEANSTACK
   | CHECKLOCKTIMEVERIFY
   | CHECKSEQUENCEVERIFY
   | LOW_S
   ;

#[allow(dead_code)] pub const STANDARD_NOT_MANDATORY:u32
   = STANDARD & !MANDATORY;


