use futures::{Future, Poll, Async};

use response::{Body, Builder, Error};


#[derive(Debug, Clone)]
#[must_use = "futures do nothing unless polled"]
pub struct Method<F>(F);

impl<F> Method<F> {
    pub fn new(f: F) -> Method<F> {
        Method(f)
    }
}

impl<F, S> Future for Method<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body: Body<S>| {
            body.method = Some(body.request.method().as_str().to_string());
            body
        }).map(Async::Ready)
    }
}

impl<F, S> Builder<S> for Method<F> where F: Future<Item=Body<S>, Error=Error> {}
