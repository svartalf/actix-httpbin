use std::convert::From;

use futures::{Future, Poll, Async};
use serde::Serialize;
use serde_json;
use response::{Body, Builder, Error};


pub struct Extra<F, V> where V: Serialize {
    inner: F,
    key: &'static str,
    value: V,
}

impl<F, V> Extra<F, V> where V: Serialize {
    pub fn new(f: F, key: &'static str, value: V) -> Extra<F, V> {
        Extra{
            inner: f,
            key,
            value
        }
    }
}

impl<F, V, S> Future for Extra<F, V> where F: Future<Item=Body<S>, Error=Error>, V: Serialize {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
         match self.inner.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(mut body)) => {
                match serde_json::to_value(&self.value) {
                    Ok(serialized) => {
                        body.extra.insert(self.key, serialized);
                        Ok(Async::Ready(body))
                    },
                    Err(e) => Err(From::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }
}

impl<F, V, S> Builder<S> for Extra<F, V> where F: Future<Item=Body<S>, Error=Error>, V: Serialize {}
