use url::form_urlencoded;
use futures::{Future, Poll, Async};

use response::{Body, Builder, Error};
use map::PickyHashMap;


/// Attach query arguments to `Body`.
#[derive(Debug, Clone)]
#[must_use = "futures do nothing unless polled"]
pub struct Args<F>(F);

impl<F> Args<F> {
    pub fn new(f: F) -> Args<F> {
        Args(f)
    }
}

impl<F, S> Future for Args<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body| {
            let mut args = PickyHashMap::new();
            for (key, value) in form_urlencoded::parse(body.request.query_string().as_ref()) {
                let mut entry = args.entry(key.into_owned()).or_insert_with(|| {
                    Vec::with_capacity(1)
                });

                (*entry).push(value.into_owned());
            }
            body.args = Some(args);

            body
        }).map(Async::Ready)
    }
}

impl<F, S> Builder<S> for Args<F> where F: Future<Item=Body<S>, Error=Error> {}
