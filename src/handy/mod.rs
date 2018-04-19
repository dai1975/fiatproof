pub mod display;
pub mod hexbyte;
pub use self::hexbyte::{
    h2b, h2b_rev,
    b2h, b2h_rev,
    FromBytesError, FromHexError,
};

pub mod format;
pub use self::format::{FmtVec};
