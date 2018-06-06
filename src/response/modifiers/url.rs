use futures::{Future, Poll, Async};

use response::{Body, Builder, Error};


#[derive(Debug, Clone)]
#[must_use = "futures do nothing unless polled"]
pub struct Url<F>(F);

impl<F> Url<F> {
    pub fn new(f: F) -> Url<F> {
        Url(f)
    }
}

impl<F, S> Future for Url<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body: Body<S>| {
            {
                let conn = body.request.connection_info();
                body.url = Some(format!("{}://{}{}", conn.scheme(), conn.host(), body.request.path()));
            }

            body
        }).map(Async::Ready)
    }
}

impl<F, S> Builder<S> for Url<F> where F: Future<Item=Body<S>, Error=Error> {}
