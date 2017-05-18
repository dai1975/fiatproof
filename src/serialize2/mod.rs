#[macro_use]
pub mod error;
pub use self::error::{SerializeError, DeserializeError};

mod medium {
   pub enum Net { }
   pub enum Disk { }
}

pub mod write_stream;
pub use self::write_stream::{WriteStream, SliceWriteStream, VecWriteStream, SizeWriteStream};
pub mod hash_write_stream;
pub use self::hash_write_stream::HashWriteStream;

pub mod read_stream;
pub use self::read_stream::{ReadStream, SliceReadStream, SizeReadStream};

pub mod types;
pub use self::types::{VarInt, FixedOctets, SizedOctets, LimitedSequence, LimitedString};

pub mod serializer;
pub use self::serializer::{Serializer};

/*
//pub mod media;
//pub use self::media::{Media};
pub mod encode;
pub use self::encode::{Encoder, EncodeStream, Encodee};
pub mod decode;
pub use self::decode::{Decoder, DecodeStream, Decodee};
pub mod bitcoin;
pub use self::bitcoin::{BitcoinEncoder, BitcoinEncodeStream, BitcoinDecoder, BitcoinDecodeStream, BitcoinCodec};

pub mod limited_string;
pub mod limited_sequence;

#[macro_use]
pub mod dump;
pub use self::dump::{ToBytes, FromBytes, WithBytes, ToDigest, FromBytesError, FromHexError};
 */
