use std::convert::From;

use futures::{Future, Poll, Async};
use serde_json;

use response::{Body, Error};

pub type JsonValueFuture = Future<Item=serde_json::Value, Error=Error>;


#[derive(Debug)]
pub struct JsonValueFormatter<F>(F);

impl<F> JsonValueFormatter<F> {
    pub fn new(f: F) -> JsonValueFormatter<F> {
        JsonValueFormatter(f)
    }
}

impl<F, S> Future for JsonValueFormatter<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = serde_json::Value;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self.0.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(body)) => {
                match serde_json::to_value(&body) {
                    Ok(value) => Ok(Async::Ready(value)),
                    Err(e) => Err(From::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }
}
