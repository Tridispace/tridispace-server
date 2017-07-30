extern crate futures;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

mod codec;
mod service;

use std::io;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_service::{Service, NewService};
use tokio_io::AsyncRead;
use futures::{Future, Stream, Sink};

pub fn run_server() -> io::Result<()> {
    serve(|| Ok(service::ClientService))
}

fn serve<S>(s: S) -> io::Result<()> 
    where S: NewService<Request = (u32, String),
                        Response = (u32, String),
                        Error = io::Error> + 'static
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let address = "0.0.0.0:12345".parse().expect("Could not parse address");
    let listener = TcpListener::bind(&address, &handle)?;

    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(codec::LineCodec).split();
        let service = s.new_service()?;
        
        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses)
            .then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server)
}

