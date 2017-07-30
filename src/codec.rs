use std::io;
use std::str;
use bytes::{BytesMut, BufMut, BigEndian, IntoBuf, Buf};
use tokio_io::codec::{Encoder, Decoder};

// Handles raw TCP packets and
// translates them to what we need.
pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = (u32, String);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if buf.len() < 5 {
            return Ok(None);
        }

        let newline = buf[4..].iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            // Remove the serialized from from the buffer
            let mut line = buf.split_to(n+4);

            // Remove '\n'
            buf.split_to(1);

            let id = line.split_to(4).into_buf().get_u32::<BigEndian>();

            match str::from_utf8(&line[..]) {
                Ok(s) => Ok(Some((id, s.to_string()))),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = (u32, String);
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        let (id, msg) = msg;

        buf.put_u32::<BigEndian>(id);
        buf.put(msg.as_bytes());
        buf.put("\n");
        Ok(())
    }
}
