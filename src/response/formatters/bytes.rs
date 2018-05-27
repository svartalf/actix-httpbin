use std::convert::From;

use serde_json;
use bytes::Bytes;
use futures::{Future, Poll, Async};

use response::{Body, Error};

pub type BytesFuture = Future<Item=Bytes, Error=Error>;


pub struct BytesFormatter<F>(F);

impl<F> BytesFormatter<F> {
    pub fn new(f: F) -> BytesFormatter<F> {
        BytesFormatter(f)
    }
}

impl<'r, F> Future for BytesFormatter<F> where F: Future<Item=Body, Error=Error> {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self.0.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(body)) => {
                match serde_json::to_vec(&body) {
                    Ok(bytes) => Ok(Async::Ready(Bytes::from(bytes))),
                    Err(e) => Err(From::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }
}
