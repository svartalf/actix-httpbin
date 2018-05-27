use futures::{Future, Poll, Async};
use actix_web::http::header::USER_AGENT;
use actix_web::HttpMessage;

use response::{Body, Builder, Error};


/// Attach query arguments to `Body`.
#[derive(Debug, Clone)]
pub struct UserAgent<F>(F);

impl<F> UserAgent<F> {
    pub fn new(f: F) -> UserAgent<F> {
        UserAgent(f)
    }
}

impl<F, S> Future for UserAgent<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        let e = match self.0.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(body)) => Ok(body),
            Err(e) => Err(e),
        };

        e.map(|mut body| {
            if let Some(header) = body.request.headers().get(USER_AGENT) {
                // TODO: Handle error
                body.user_agent = Some(header.to_str().unwrap_or("").to_string())
            }

            body
        }).map(Async::Ready)
    }
}

impl<F, S> Builder<S> for UserAgent<F> where F: Future<Item=Body<S>, Error=Error> {}
