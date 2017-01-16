#[macro_use]
pub mod error;
pub use self::error::SerializeError;

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
pub mod serializer;
pub use self::serializer::{Serializer,
                           SliceSerializer, FixedSerializer, SizeSerializer, DHash256Serializer };

#[macro_use]
pub mod deserializer;
pub use self::deserializer::{Deserializer,
                             SliceDeserializer, FixedDeserializer};
pub mod limited_string;
pub mod limited_sequence;

mod impls;
