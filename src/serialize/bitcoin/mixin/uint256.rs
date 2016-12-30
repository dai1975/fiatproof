use ::{Error, UInt256};
use super::super::super::{Encoder, WriteStream};
use super::super::{BitcoinEncoder, BitcoinEncodee};

impl BitcoinEncodee for UInt256 {
   fn encode<E:BitcoinEncoder, W:WriteStream>(&self, e:&mut E, w:&mut W, ep:&<E as Encoder>::P) -> Result<usize, Error> {
      e.encode_uint256(self, w, ep)
   }
}

