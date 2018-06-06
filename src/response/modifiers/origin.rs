use futures::{Future, Poll, Async};

use response::{Body, Builder, Error};

/// Attach origin IP to `Body`.
#[derive(Debug, Clone)]
#[must_use = "futures do nothing unless polled"]
pub struct Origin<F>(F);

impl<F> Origin<F> {
    pub fn new(f: F) -> Origin<F> {
        Origin(f)
    }
}

impl<F, S> Future for Origin<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body: Body<S>| {
            let origin = match body.request.connection_info().remote() {
                Some(remote) => remote.split(':').next().unwrap_or("").to_string(),
                None => "".to_string()
            };

            body.origin = Some(origin);

            body
        }).map(Async::Ready)
    }
}

impl<'r, F, S> Builder<S> for Origin<F> where F: Future<Item=Body<S>, Error=Error> {}
