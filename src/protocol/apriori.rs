pub const PROTOCOL_VERSION:i32     = 70012;
pub const SENDHEADERS_VERSION:i32  = 70012;
pub const BIP0031_VERSION:i32      = 60000;
pub const GETHEADERS_VERSION:i32   = 31800;
pub const ADDRESS_TIME_VERSION:i32 = 31402;

pub const INIT_PROTO_VERSION:i32  = 209;
pub const MIN_PEER_PROTO_VERSION:i32 = GETHEADERS_VERSION;

pub const NODE_NONE:u64    = 0;
pub const NODE_FULL:u64    = 1 << 0;
pub const NODE_GETUTXO:u64 = 1 << 1;
pub const NODE_BLOOM:u64   = 1 << 2;
pub const NODE_WITNESS:u64 = 1 << 3;
pub const NODE_XTHIN:u64   = 1 << 4;

pub const MAX_MESSAGE_LENGTH:usize = 4 * 1000 * 1000;
pub const MAX_SUBVERSION_LENGTH:usize = 256;
pub const MAX_ADDR_SIZE:usize = 1000;
pub const MAX_INV_SIZE:usize = 1000;

