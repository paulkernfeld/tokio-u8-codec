//! A Tokio codec that just reads and writes u8s.
//!
//! This is probably not a very performant way to process streams of bytes because it only
//! looks at one byte at a time.
//!
//! ```
//! extern crate bytes;
//! extern crate tokio_io;
//! extern crate tokio_u8_codec;
//!
//! use tokio_io::codec::{Decoder, Encoder};
//! use tokio_u8_codec::U8Codec;
//! use bytes::BytesMut;
//!
//! fn main() {
//!     let mut buf: BytesMut = Default::default();
//!     let mut codec: U8Codec = Default::default();
//!     codec.encode(1, &mut buf).unwrap();
//!     codec.encode(2, &mut buf).unwrap();
//!     assert_eq!(codec.decode(&mut buf).unwrap(), Some(1));
//!     assert_eq!(codec.decode(&mut buf).unwrap(), Some(2));
//!     assert_eq!(codec.decode(&mut buf).unwrap(), None);
//! }
//! ```
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate bytes;
extern crate tokio_io;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};


#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

#[derive(Default)]
pub struct U8Codec {}

impl Encoder for U8Codec {
    type Item = u8;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(dst.extend(&[item]))
    }
}
impl Decoder for U8Codec {
    type Item = u8;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            Ok(Some(src.split_to(1)[0]))
        }
    }
}
