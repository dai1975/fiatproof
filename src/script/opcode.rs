#[allow(dead_code)]

// push value

macro_rules! defop (
   ($k:ident, $v:expr) => {
      #[allow(dead_code,non_upper_case_globals)] pub const $k:u8 = $v;
   }
);

defop!(OP_0, 0x00);
defop!(OP_FALSE, OP_0);

defop!(OP_PUSHDATA1, 0x4c);
defop!(OP_PUSHDATA2, 0x4d);
defop!(OP_PUSHDATA4, 0x4e);
defop!(OP_1NEGATE, 0x4f);
defop!(OP_RESERVED, 0x50);
defop!(OP_1, 0x51);
defop!(OP_TRUE, OP_1);
defop!(OP_2, 0x52);
defop!(OP_3, 0x53);
defop!(OP_4, 0x54);
defop!(OP_5, 0x55);
defop!(OP_6, 0x56);
defop!(OP_7, 0x57);
defop!(OP_8, 0x58);
defop!(OP_9, 0x59);
defop!(OP_10, 0x5a);
defop!(OP_11, 0x5b);
defop!(OP_12, 0x5c);
defop!(OP_13, 0x5d);
defop!(OP_14, 0x5e);
defop!(OP_15, 0x5f);
defop!(OP_16, 0x60);

// control
defop!(OP_NOP, 0x61);
defop!(OP_VER, 0x62); //reserved
defop!(OP_IF, 0x63);
defop!(OP_NOTIF, 0x64);
defop!(OP_VERIF, 0x65); //reserved
defop!(OP_VERNOTIF, 0x66); //reserved
defop!(OP_ELSE, 0x67);
defop!(OP_ENDIF, 0x68);
defop!(OP_VERIFY, 0x69); //reserved
defop!(OP_RETURN, 0x6a);
// stack ops
defop!(OP_TOALTSTACK, 0x6b);
defop!(OP_FROMALTSTACK, 0x6c);
defop!(OP_2DROP, 0x6d);
defop!(OP_2DUP, 0x6e);
defop!(OP_3DUP, 0x6f);
defop!(OP_2OVER, 0x70);
defop!(OP_2ROT, 0x71);
defop!(OP_2SWAP, 0x72);
defop!(OP_IFDUP, 0x73);
defop!(OP_DEPTH, 0x74);
defop!(OP_DROP, 0x75);
defop!(OP_DUP, 0x76);
defop!(OP_NIP, 0x77);
defop!(OP_OVER, 0x78);
defop!(OP_PICK, 0x79);
defop!(OP_ROLL, 0x7a);
defop!(OP_ROT, 0x7b);
defop!(OP_SWAP, 0x7c);
defop!(OP_TUCK, 0x7d);

// splice ops
defop!(OP_CAT, 0x7e);
defop!(OP_SUBSTR, 0x7f);
defop!(OP_LEFT, 0x80);
defop!(OP_RIGHT, 0x81);
defop!(OP_SIZE, 0x82);
// bit logic
defop!(OP_INVERT, 0x83);
defop!(OP_AND, 0x84);
defop!(OP_OR, 0x85);
defop!(OP_XOR, 0x86);
defop!(OP_EQUAL, 0x87);
defop!(OP_EQUALVERIFY, 0x88);
defop!(OP_RESERVED1, 0x89);
defop!(OP_RESERVED2, 0x8a);

// numeric
defop!(OP_1ADD, 0x8b);
defop!(OP_1SUB, 0x8c);
defop!(OP_2MUL, 0x8d);
defop!(OP_2DIV, 0x8e);
defop!(OP_NEGATE, 0x8f);
defop!(OP_ABS, 0x90);
defop!(OP_NOT, 0x91);
defop!(OP_0NOTEQUAL, 0x92);

defop!(OP_ADD, 0x93);
defop!(OP_SUB, 0x94);
defop!(OP_MUL, 0x95);
defop!(OP_DIV, 0x96);
defop!(OP_MOD, 0x97);
defop!(OP_LSHIFT, 0x98);
defop!(OP_RSHIFT, 0x99);

defop!(OP_BOOLAND, 0x9a);
defop!(OP_BOOLOR, 0x9b);
defop!(OP_NUMEQUAL, 0x9c);
defop!(OP_NUMEQUALVERIFY, 0x9d);
defop!(OP_NUMNOTEQUAL, 0x9e);
defop!(OP_LESSTHAN, 0x9f);
defop!(OP_GREATERTHAN, 0xa0);
defop!(OP_LESSTHANOREQUAL, 0xa1);
defop!(OP_GREATERTHANOREQUAL, 0xa2);
defop!(OP_MIN, 0xa3);
defop!(OP_MAX, 0xa4);

defop!(OP_WITHIN, 0xa5);
// crypto
defop!(OP_RIPEMD160, 0xa6);
defop!(OP_SHA1, 0xa7);
defop!(OP_SHA256, 0xa8);
defop!(OP_HASH160, 0xa9);
defop!(OP_HASH256, 0xaa);
defop!(OP_CODESEPARATOR, 0xab);
defop!(OP_CHECKSIG, 0xac);
defop!(OP_CHECKSIGVERIFY, 0xad);
defop!(OP_CHECKMULTISIG, 0xae);
defop!(OP_CHECKMULTISIGVERIFY, 0xaf);

// expansion
defop!(OP_NOP1, 0xb0); //reserved
defop!(OP_CHECKLOCKTIMEVERIFY, 0xb1); //old NOP2
defop!(OP_CHECKSEQUENCEVERIFY, 0xb2); //old NOP3
defop!(OP_NOP4, 0xb3); //reserved
defop!(OP_NOP5, 0xb4); //reserved
defop!(OP_NOP6, 0xb5); //reserved
defop!(OP_NOP7, 0xb6); //reserved
defop!(OP_NOP8, 0xb7); //reserved
defop!(OP_NOP9, 0xb8); //reserved
defop!(OP_NOP10, 0xb9); //reserved


// template matching params
defop!(OP_SMALLINTEGER, 0xfa);
defop!(OP_PUBKEYS, 0xfb);
defop!(OP_PUBKEYHASH, 0xfd);
defop!(OP_PUBKEY, 0xfe);

defop!(OP_INVALIDOPCODE, 0xff);

defop!(OP_PUSHDATAFIX_01, 0x01); defop!(OP_PUSHDATAFIX_02, 0x02); defop!(OP_PUSHDATAFIX_03, 0x03);
defop!(OP_PUSHDATAFIX_04, 0x04); defop!(OP_PUSHDATAFIX_05, 0x05); defop!(OP_PUSHDATAFIX_06, 0x06); defop!(OP_PUSHDATAFIX_07, 0x07);
defop!(OP_PUSHDATAFIX_08, 0x08); defop!(OP_PUSHDATAFIX_09, 0x09); defop!(OP_PUSHDATAFIX_0A, 0x0A); defop!(OP_PUSHDATAFIX_0B, 0x0B);
defop!(OP_PUSHDATAFIX_0C, 0x0C); defop!(OP_PUSHDATAFIX_0D, 0x0D); defop!(OP_PUSHDATAFIX_0E, 0x0E); defop!(OP_PUSHDATAFIX_0F, 0x0F);
defop!(OP_PUSHDATAFIX_10, 0x10); defop!(OP_PUSHDATAFIX_11, 0x11); defop!(OP_PUSHDATAFIX_12, 0x12); defop!(OP_PUSHDATAFIX_13, 0x13);
defop!(OP_PUSHDATAFIX_14, 0x14); defop!(OP_PUSHDATAFIX_15, 0x15); defop!(OP_PUSHDATAFIX_16, 0x16); defop!(OP_PUSHDATAFIX_17, 0x17);
defop!(OP_PUSHDATAFIX_18, 0x18); defop!(OP_PUSHDATAFIX_19, 0x19); defop!(OP_PUSHDATAFIX_1A, 0x1A); defop!(OP_PUSHDATAFIX_1B, 0x1B);
defop!(OP_PUSHDATAFIX_1C, 0x1C); defop!(OP_PUSHDATAFIX_1D, 0x1D); defop!(OP_PUSHDATAFIX_1E, 0x1E); defop!(OP_PUSHDATAFIX_1F, 0x1F);
defop!(OP_PUSHDATAFIX_20, 0x20); defop!(OP_PUSHDATAFIX_21, 0x21); defop!(OP_PUSHDATAFIX_22, 0x22); defop!(OP_PUSHDATAFIX_23, 0x23);
defop!(OP_PUSHDATAFIX_24, 0x24); defop!(OP_PUSHDATAFIX_25, 0x25); defop!(OP_PUSHDATAFIX_26, 0x26); defop!(OP_PUSHDATAFIX_27, 0x27);
defop!(OP_PUSHDATAFIX_28, 0x28); defop!(OP_PUSHDATAFIX_29, 0x29); defop!(OP_PUSHDATAFIX_2A, 0x2A); defop!(OP_PUSHDATAFIX_2B, 0x2B);
defop!(OP_PUSHDATAFIX_2C, 0x2C); defop!(OP_PUSHDATAFIX_2D, 0x2D); defop!(OP_PUSHDATAFIX_2E, 0x2E); defop!(OP_PUSHDATAFIX_2F, 0x2F);
defop!(OP_PUSHDATAFIX_30, 0x30); defop!(OP_PUSHDATAFIX_31, 0x31); defop!(OP_PUSHDATAFIX_32, 0x32); defop!(OP_PUSHDATAFIX_33, 0x33);
defop!(OP_PUSHDATAFIX_34, 0x34); defop!(OP_PUSHDATAFIX_35, 0x35); defop!(OP_PUSHDATAFIX_36, 0x36); defop!(OP_PUSHDATAFIX_37, 0x37);
defop!(OP_PUSHDATAFIX_38, 0x38); defop!(OP_PUSHDATAFIX_39, 0x39); defop!(OP_PUSHDATAFIX_3A, 0x3A); defop!(OP_PUSHDATAFIX_3B, 0x3B);
defop!(OP_PUSHDATAFIX_3C, 0x3C); defop!(OP_PUSHDATAFIX_3D, 0x3D); defop!(OP_PUSHDATAFIX_3E, 0x3E); defop!(OP_PUSHDATAFIX_3F, 0x3F);
defop!(OP_PUSHDATAFIX_40, 0x40); defop!(OP_PUSHDATAFIX_41, 0x41); defop!(OP_PUSHDATAFIX_42, 0x42); defop!(OP_PUSHDATAFIX_43, 0x43);
defop!(OP_PUSHDATAFIX_44, 0x44); defop!(OP_PUSHDATAFIX_45, 0x45); defop!(OP_PUSHDATAFIX_46, 0x46); defop!(OP_PUSHDATAFIX_47, 0x47);
defop!(OP_PUSHDATAFIX_48, 0x48); defop!(OP_PUSHDATAFIX_49, 0x49); defop!(OP_PUSHDATAFIX_4A, 0x4A); defop!(OP_PUSHDATAFIX_4B, 0x4B);

const CONTEXT_SOURCE:u32  = 0x01;
const CONTEXT_EXECUTE:u32 = 0x02;
const CONTEXT_NONE:u32    = 0x00;
const CONTEXT_ALL:u32     = CONTEXT_SOURCE | CONTEXT_EXECUTE;

#[derive(Debug,Clone)]
pub struct OpCodeInfo {
   pub code: u8,
   pub name: &'static str,
   pub validity: u32,
}

pub const OPCODE_INFO:[OpCodeInfo; 256] = [
   // push value
   OpCodeInfo{ code:0x00, name:"OP_0",    validity:CONTEXT_ALL },

   OpCodeInfo{ code:0x01, name:"OP_PUSHDATAFIX_01", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x02, name:"OP_PUSHDATAFIX_02", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x03, name:"OP_PUSHDATAFIX_03", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x04, name:"OP_PUSHDATAFIX_04", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x05, name:"OP_PUSHDATAFIX_05", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x06, name:"OP_PUSHDATAFIX_06", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x07, name:"OP_PUSHDATAFIX_07", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x08, name:"OP_PUSHDATAFIX_08", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x09, name:"OP_PUSHDATAFIX_09", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0a, name:"OP_PUSHDATAFIX_0A", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0b, name:"OP_PUSHDATAFIX_0B", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0c, name:"OP_PUSHDATAFIX_0C", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0d, name:"OP_PUSHDATAFIX_0D", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0e, name:"OP_PUSHDATAFIX_0E", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x0f, name:"OP_PUSHDATAFIX_0F", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x10, name:"OP_PUSHDATAFIX_10", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x11, name:"OP_PUSHDATAFIX_11", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x12, name:"OP_PUSHDATAFIX_12", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x13, name:"OP_PUSHDATAFIX_13", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x14, name:"OP_PUSHDATAFIX_14", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x15, name:"OP_PUSHDATAFIX_15", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x16, name:"OP_PUSHDATAFIX_16", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x17, name:"OP_PUSHDATAFIX_17", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x18, name:"OP_PUSHDATAFIX_18", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x19, name:"OP_PUSHDATAFIX_19", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1a, name:"OP_PUSHDATAFIX_1A", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1b, name:"OP_PUSHDATAFIX_1B", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1c, name:"OP_PUSHDATAFIX_1C", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1d, name:"OP_PUSHDATAFIX_1D", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1e, name:"OP_PUSHDATAFIX_1E", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x1f, name:"OP_PUSHDATAFIX_1F", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x20, name:"OP_PUSHDATAFIX_20", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x21, name:"OP_PUSHDATAFIX_21", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x22, name:"OP_PUSHDATAFIX_22", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x23, name:"OP_PUSHDATAFIX_23", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x24, name:"OP_PUSHDATAFIX_24", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x25, name:"OP_PUSHDATAFIX_25", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x26, name:"OP_PUSHDATAFIX_26", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x27, name:"OP_PUSHDATAFIX_27", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x28, name:"OP_PUSHDATAFIX_28", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x29, name:"OP_PUSHDATAFIX_29", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2a, name:"OP_PUSHDATAFIX_2A", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2b, name:"OP_PUSHDATAFIX_2B", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2c, name:"OP_PUSHDATAFIX_2C", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2d, name:"OP_PUSHDATAFIX_2D", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2e, name:"OP_PUSHDATAFIX_2E", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x2f, name:"OP_PUSHDATAFIX_2F", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x30, name:"OP_PUSHDATAFIX_30", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x31, name:"OP_PUSHDATAFIX_31", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x32, name:"OP_PUSHDATAFIX_32", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x33, name:"OP_PUSHDATAFIX_33", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x34, name:"OP_PUSHDATAFIX_34", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x35, name:"OP_PUSHDATAFIX_35", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x36, name:"OP_PUSHDATAFIX_36", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x37, name:"OP_PUSHDATAFIX_37", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x38, name:"OP_PUSHDATAFIX_38", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x39, name:"OP_PUSHDATAFIX_39", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3a, name:"OP_PUSHDATAFIX_3A", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3b, name:"OP_PUSHDATAFIX_3B", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3c, name:"OP_PUSHDATAFIX_3C", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3d, name:"OP_PUSHDATAFIX_3D", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3e, name:"OP_PUSHDATAFIX_3E", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x3f, name:"OP_PUSHDATAFIX_3F", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x40, name:"OP_PUSHDATAFIX_40", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x41, name:"OP_PUSHDATAFIX_41", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x42, name:"OP_PUSHDATAFIX_42", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x43, name:"OP_PUSHDATAFIX_43", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x44, name:"OP_PUSHDATAFIX_44", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x45, name:"OP_PUSHDATAFIX_45", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x46, name:"OP_PUSHDATAFIX_46", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x47, name:"OP_PUSHDATAFIX_47", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x48, name:"OP_PUSHDATAFIX_48", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x49, name:"OP_PUSHDATAFIX_49", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4a, name:"OP_PUSHDATAFIX_4A", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4b, name:"OP_PUSHDATAFIX_4B", validity:CONTEXT_ALL,  },
   
   OpCodeInfo{ code:0x4c, name:"OP_PUSHDATA1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4d, name:"OP_PUSHDATA2", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4e, name:"OP_PUSHDATA4", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x4f, name:"OP_1NEGATE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x50, name:"OP_RESERVED", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x51, name:"OP_1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x52, name:"OP_2", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x53, name:"OP_3", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x54, name:"OP_4", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x55, name:"OP_5", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x56, name:"OP_6", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x57, name:"OP_7", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x58, name:"OP_8", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x59, name:"OP_9", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5a, name:"OP_10", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5b, name:"OP_11", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5c, name:"OP_12", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5d, name:"OP_13", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5e, name:"OP_14", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x5f, name:"OP_15", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x60, name:"OP_16", validity:CONTEXT_ALL,  },

   // control
   OpCodeInfo{ code:0x61, name:"OP_NOP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x62, name:"OP_VER", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x63, name:"OP_IF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x64, name:"OP_NOTIF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x65, name:"OP_VERIF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0x66, name:"OP_VERNOTIF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0x67, name:"OP_ELSE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x68, name:"OP_ENDIF", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x69, name:"OP_VERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6a, name:"OP_RETURN", validity:CONTEXT_ALL,  },
   // stack ops
   OpCodeInfo{ code:0x6b, name:"OP_TOALTSTACK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6c, name:"OP_FROMALTSTACK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6d, name:"OP_2DROP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6e, name:"OP_2DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x6f, name:"OP_3DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x70, name:"OP_2OVER", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x71, name:"OP_2ROT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x72, name:"OP_2SWAP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x73, name:"OP_IFDUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x74, name:"OP_DEPTH", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x75, name:"OP_DROP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x76, name:"OP_DUP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x77, name:"OP_NIP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x78, name:"OP_OVER", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x79, name:"OP_PICK", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7a, name:"OP_ROLL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7b, name:"OP_ROT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7c, name:"OP_SWAP", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x7d, name:"OP_TUCK", validity:CONTEXT_ALL,  },

   // splice ops
   OpCodeInfo{ code:0x7e, name:"OP_CAT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x7f, name:"OP_SUBSTR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x80, name:"OP_LEFT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x81, name:"OP_RIGHT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x82, name:"OP_SIZE", validity:CONTEXT_ALL,  },
   // bit logic
   OpCodeInfo{ code:0x83, name:"OP_INVERT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x84, name:"OP_AND", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x85, name:"OP_OR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x86, name:"OP_XOR", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x87, name:"OP_EQUAL", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x88, name:"OP_EQUALVERIFY", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0x89, name:"OP_RESERVED1", validity:CONTEXT_EXECUTE, },
   OpCodeInfo{ code:0x8a, name:"OP_RESERVED2", validity:CONTEXT_EXECUTE, },

   // numeric
   OpCodeInfo{ code:0x8b, name:"OP_1ADD", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x8c, name:"OP_1SUB", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x8d, name:"OP_2MUL", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x8e, name:"OP_2DIV", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x8f, name:"OP_NEGATE", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x90, name:"OP_ABS", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x91, name:"OP_NOT", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x92, name:"OP_0NOTEQUAL", validity:CONTEXT_ALL,  },

   OpCodeInfo{ code:0x93, name:"OP_ADD", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x94, name:"OP_SUB", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x95, name:"OP_MUL", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x96, name:"OP_DIV", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x97, name:"OP_MOD", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x98, name:"OP_LSHIFT", validity:CONTEXT_NONE,  },
   OpCodeInfo{ code:0x99, name:"OP_RSHIFT", validity:CONTEXT_NONE,  },

   OpCodeInfo{ code:0x9a, name:"OP_BOOLAND", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9b, name:"OP_BOOLOR", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9c, name:"OP_NUMEQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9d, name:"OP_NUMEQUALVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9e, name:"OP_NUMNOTEQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0x9f, name:"OP_LESSTHAN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa0, name:"OP_GREATERTHAN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa1, name:"OP_LESSTHANOREQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa2, name:"OP_GREATERTHANOREQUAL", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa3, name:"OP_MIN", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa4, name:"OP_MAX", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa5, name:"OP_WITHIN", validity:CONTEXT_ALL,  },
   // crypto
   OpCodeInfo{ code:0xa6, name:"OP_RIPEMD160", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa7, name:"OP_SHA1", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa8, name:"OP_SHA256", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xa9, name:"OP_HASH160", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xaa, name:"OP_HASH256", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xab, name:"OP_CODESEPARATOR", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xac, name:"OP_CHECKSIG", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xad, name:"OP_CHECKSIGVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xae, name:"OP_CHECKMULTISIG", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xaf, name:"OP_CHECKMULTISIGVERIFY", validity:CONTEXT_ALL,  },

   // expansion
   OpCodeInfo{ code:0xb0, name:"OP_NOP1", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb1, name:"OP_CHECKLOCKTIMEVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xb2, name:"OP_CHECKSEQUENCEVERIFY", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xb3, name:"OP_NOP4", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb4, name:"OP_NOP5", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb5, name:"OP_NOP6", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb6, name:"OP_NOP7", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb7, name:"OP_NOP8", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb8, name:"OP_NOP9", validity:CONTEXT_ALL, },
   OpCodeInfo{ code:0xb9, name:"OP_NOP10", validity:CONTEXT_ALL, },

   OpCodeInfo{ code:0xba, name:"OP_UNUSED_BA", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbb, name:"OP_UNUSED_BB", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbc, name:"OP_UNUSED_BC", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbd, name:"OP_UNUSED_BD", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbe, name:"OP_UNUSED_BE", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xbf, name:"OP_UNUSED_BF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc0, name:"OP_UNUSED_C0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc1, name:"OP_UNUSED_C1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc2, name:"OP_UNUSED_C2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc3, name:"OP_UNUSED_C3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc4, name:"OP_UNUSED_C4", validity:CONTEXT_NONE, },

   OpCodeInfo{ code:0xc5, name:"OP_UNUSED_C5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc6, name:"OP_UNUSED_C6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc7, name:"OP_UNUSED_C7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc8, name:"OP_UNUSED_C8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xc9, name:"OP_UNUSED_C9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xca, name:"OP_UNUSED_CA", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcb, name:"OP_UNUSED_CB", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcc, name:"OP_UNUSED_CC", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcd, name:"OP_UNUSED_CD", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xce, name:"OP_UNUSED_CE", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xcf, name:"OP_UNUSED_CF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd0, name:"OP_UNUSED_D0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd1, name:"OP_UNUSED_D1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd2, name:"OP_UNUSED_D2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd3, name:"OP_UNUSED_D3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd4, name:"OP_UNUSED_D4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd5, name:"OP_UNUSED_D5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd6, name:"OP_UNUSED_D6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd7, name:"OP_UNUSED_D7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd8, name:"OP_UNUSED_D8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xd9, name:"OP_UNUSED_D9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xda, name:"OP_UNUSED_DA", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdb, name:"OP_UNUSED_DB", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdc, name:"OP_UNUSED_DC", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdd, name:"OP_UNUSED_DD", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xde, name:"OP_UNUSED_DE", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xdf, name:"OP_UNUSED_DF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe0, name:"OP_UNUSED_E0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe1, name:"OP_UNUSED_E1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe2, name:"OP_UNUSED_E2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe3, name:"OP_UNUSED_E3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe4, name:"OP_UNUSED_E4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe5, name:"OP_UNUSED_E5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe6, name:"OP_UNUSED_E6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe7, name:"OP_UNUSED_E7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe8, name:"OP_UNUSED_E8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xe9, name:"OP_UNUSED_E9", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xea, name:"OP_UNUSED_EA", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xeb, name:"OP_UNUSED_EB", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xec, name:"OP_UNUSED_EC", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xed, name:"OP_UNUSED_ED", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xee, name:"OP_UNUSED_EE", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xef, name:"OP_UNUSED_EF", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf0, name:"OP_UNUSED_F0", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf1, name:"OP_UNUSED_F1", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf2, name:"OP_UNUSED_F2", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf3, name:"OP_UNUSED_F3", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf4, name:"OP_UNUSED_F4", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf5, name:"OP_UNUSED_F5", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf6, name:"OP_UNUSED_F6", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf7, name:"OP_UNUSED_F7", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf8, name:"OP_UNUSED_F8", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xf9, name:"OP_UNUSED_F9", validity:CONTEXT_NONE, },
   
   // template matching params
   OpCodeInfo{ code:0xfa, name:"OP_SMALLINTEGER", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfb, name:"OP_PUBKEYS", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfc, name:"OP_UNUSED_FC", validity:CONTEXT_NONE, },
   OpCodeInfo{ code:0xfd, name:"OP_PUBKEYHASH", validity:CONTEXT_ALL,  },
   OpCodeInfo{ code:0xfe, name:"OP_PUBKEY", validity:CONTEXT_ALL,  },
   
   OpCodeInfo{ code:0xff, name:"OP_INVALIDOPCODE", validity:CONTEXT_ALL,  },
];

/*
use std::ops::Index);
impl Index<u8> for OpCodeInfo {
   type Output = OpCodeInfo);
   fn index(&self, index: u8) -> &Self::Output { self.index(index as usize) }
}
*/

#[test]
fn test_infoarray() {
   assert_eq!(256, OPCODE_INFO.len());
}
