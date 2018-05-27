use bytes::Bytes;
use serde_json::{self, Value};
use actix_web::{HttpRequest, HttpResponse, Path};
use actix_web::http::StatusCode;
use futures::{Future, Stream, Poll, Async};

use application::State;
use response::{Body, Builder, Error};

#[derive(Serialize)]
struct StreamLine<'l> {
    #[serde(flatten)]
    value: &'l Value,
    id: usize,
}

/// In order not to generate `Body` instance each time, we are going to await it's completion
/// and then just yield a `StreamLine` instance few times with a reference to a generated body.
struct BodyStream<F> {
    future: F,
    value: Option<Value>,
    iteration: usize,
    limit: usize,
}

impl<F> BodyStream<F> where F: Future {
    pub fn new(f: F, limit: u8) -> BodyStream<F> {
        BodyStream {
            future: f,
            value: None,
            iteration: 0,
            limit: limit as usize,
        }
    }
}

impl<F> Stream for BodyStream<F> where F: Future<Item=Value, Error=Error> {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Bytes>, Error> {
        if self.value.is_none() {
            // Awaiting until body future resolves at first
            match self.future.poll() {
                Err(e) => return Err(e),
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(value)) => {
                    self.value = Some(value)
                }
            }
        }

        if self.iteration >= self.limit {
            Ok(Async::Ready(None))
        } else {
            let line = StreamLine {
                value: self.value.as_ref().expect("Iteration started before inner future resolved"),
                id: self.iteration,
            };
            let mut bytes = serde_json::to_vec(&line)?;
            bytes.push(b'\n');
            self.iteration += 1;

            Ok(Async::Ready(Some(Bytes::from(bytes))))
        }
    }
}

/// Stream `n` JSON responses.
pub fn stream((req, path): (HttpRequest<State>, Path<u8>)) -> HttpResponse {
    let body = Body::new(req)
        .with_url()
        .with_args()
        .with_headers()
        .with_origin()
        .into_json_value();

    HttpResponse::build(StatusCode::OK)
        .streaming(BodyStream::new(body, *path))
}
