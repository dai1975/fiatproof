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
pub use self::codec::{BitcoinCodecParam};
pub mod encode;
pub use self::encode::{BitcoinEncoder, BitcoinEncodee};
pub mod decode;
pub use self::decode::{BitcoinDecoder, BitcoinDecodee};

#[macro_use]
pub mod serializer;
pub use self::serializer::{BitcoinSerializer,
                           SliceBitcoinSerializer, FixedBitcoinSerializer, SizeBitcoinSerializer, DHash256BitcoinSerializer };

#[macro_use]
pub mod deserializer;
pub use self::deserializer::{BitcoinDeserializer,
                             SliceBitcoinDeserializer, FixedBitcoinDeserializer};

pub mod limited_string;
pub use self::limited_string::LimitedString;

mod impls;
