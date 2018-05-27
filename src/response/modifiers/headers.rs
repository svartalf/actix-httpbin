use futures::{Future, Poll, Async};
use actix_web::HttpMessage;

use response::{Body, Builder, Error};
use map::PickyHashMap;


/// Attach headers map to `Body`.
#[derive(Debug, Clone)]
pub struct Headers<F>(F);

impl<F> Headers<F> {
    pub fn new(f: F) -> Headers<F> {
        Headers(f)
    }
}

impl<F, S> Future for Headers<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body| {
            let headers: PickyHashMap<_> = body.request.headers().iter().map(|(key, value)| {
                // TODO: Handle `.unwrap()`
                (key.as_str().to_string(), value.to_str().unwrap().to_string())
            }).collect();
            body.headers = Some(headers);

            body
        }).map(Async::Ready)
    }
}

impl<F, S> Builder<S> for Headers<F> where F: Future<Item=Body<S>, Error=Error> {}
