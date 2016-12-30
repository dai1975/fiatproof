pub mod error;
pub use self::error::SerializeError;

pub mod write_stream;
pub use self::write_stream::{WriteStream, SliceWriteStream, FixedWriteStream};
pub mod hash_write_stream;
pub use self::hash_write_stream::HashWriteStream;

pub mod encode;
pub use self::encode::{ Encoder, Encodee, Serializer, FixedSerializer };
//pub mod serializable;
//pub use self::serializable::Serializable;

pub mod bitcoin;
//pub use self::bitcoin::{ BitcoinEncode, BitcoinEncodeParam, BitcoinSerializer };
pub use self::bitcoin::{ BitcoinEncoder, BitcoinEncodeParam };




