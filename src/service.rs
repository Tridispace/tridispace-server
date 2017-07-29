use std::io;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};

pub struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

