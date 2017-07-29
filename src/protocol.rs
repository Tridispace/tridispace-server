use std::io;
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

use codec::LineCodec;

pub struct LineProto;

impl<T> ServerProto<T> for LineProto 
where T: AsyncRead + AsyncWrite + 'static
{
    type Request = String;
    type Response = String;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}
