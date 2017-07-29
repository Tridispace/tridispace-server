use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if buf.len() < 5 {
            return Ok(None);
        }

        let newline = buf.iter().position(|b| *b == b'\n');
        if let Some(i) = newline {
            let line = buf.split_to(i);
            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.put(msg.as_bytes());
        buf.put("\n");
        Ok(())
    }
}
