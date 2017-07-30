use std::io;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};

pub struct ClientService;

impl Service for ClientService {
    type Request = (u32, String);
    type Response = (u32, String);

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // TODO: This is where we put what we want to do

        future::ok(req).boxed()
    }
}

