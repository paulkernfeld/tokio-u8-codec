# tokio-u8-codec

A Tokio codec that just reads and writes u8s.

This is probably not a very performant way to process streams of bytes because it only
looks at one byte at a time.

```rust
extern crate bytes;
extern crate tokio_io;
extern crate tokio_u8_codec;

use tokio_io::codec::{Decoder, Encoder};
use tokio_u8_codec::U8Codec;
use bytes::BytesMut;

fn main() {
    let mut buf: BytesMut = Default::default();
    let mut codec: U8Codec = Default::default();
    codec.encode(1, &mut buf).unwrap();
    codec.encode(2, &mut buf).unwrap();
    assert_eq!(codec.decode(&mut buf).unwrap(), Some(1));
    assert_eq!(codec.decode(&mut buf).unwrap(), Some(2));
    assert_eq!(codec.decode(&mut buf).unwrap(), None);
}
```

License: MIT/Apache-2.0
