use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use futures::{Future, Poll, Async};
use serde_json;

use response::{Body, Error};

pub type ResponseFuture = Future<Item=HttpResponse, Error=Error>;


pub struct ResponseFormatter<F>(F);

impl<F> ResponseFormatter<F> {
    pub fn new(f: F) -> ResponseFormatter<F> {
        ResponseFormatter(f)
    }
}

impl<'r, F, S> Future for ResponseFormatter<F> where F: Future<Item=Body<S>, Error=Error> {
    type Item = HttpResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self.0.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(body)) => {
                let resp = HttpResponse::build(StatusCode::OK)
                    .content_type("application/json")
                    .body(serde_json::to_string(&body)?);

                Ok(Async::Ready(resp))
            },
            Err(e) => Err(e),
        }
    }
}
