use actix_web::Json;
use futures::{Future, Poll, Async};

use response::{Body, Error};

pub type JsonFuture<S> = Future<Item=Json<Body<S>>, Error=Error>;


pub struct JsonFormatter<F>(F);

impl<F> JsonFormatter<F> {
    pub fn new(f: F) -> JsonFormatter<F> {
        JsonFormatter(f)
    }
}

impl<'r, F, S> Future for JsonFormatter<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Json<Body<S>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self.0.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(Async::Ready(Json(body))),
            Err(e) => Err(e),
        }
    }
}
