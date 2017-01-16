#[macro_use]
pub mod error;
pub use self::error::EncodeError;

pub mod write_stream;
pub use self::write_stream::{WriteStream, SliceWriteStream, FixedWriteStream};
pub mod hash_write_stream;
pub use self::hash_write_stream::HashWriteStream;

pub mod read_stream;
pub use self::read_stream::{ReadStream, SliceReadStream, FixedReadStream};

pub mod codec;
pub use self::codec::{CodecParam};
pub mod encode;
pub use self::encode::{Encoder, Encodee};
pub mod decode;
pub use self::decode::{Decoder, Decodee};

#[macro_use]
pub mod encode_stream;
pub use self::encode_stream::{
   EncodeStream, SliceEncodeStream, FixedEncodeStream, SizeEncodeStream, DHash256EncodeStream
};

#[macro_use]
pub mod decode_stream;
pub use self::decode_stream::{
   DecodeStream, SliceDecodeStream, FixedDecodeStream
};

pub mod limited_string;
pub mod limited_sequence;

mod impls;
