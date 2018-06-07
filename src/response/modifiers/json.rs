use serde_json::Value;
use futures::{Future, Poll, Async};
use actix_web::{HttpRequest, HttpMessage};
use actix_web::dev::JsonBody;

use response::{Builder, Body, Error};

enum State {
    Body,
    Json,
}

#[must_use = "futures do nothing unless polled"]
pub struct Json<F, S> {
    state: State,
    body_f: F,
    body: Option<Body<S>>,
    json: Option<JsonBody<HttpRequest<S>, Value>>,
}

impl<F, S> Json<F, S> {
    pub fn new(f: F) -> Json<F, S> {
        Json{
            state: State::Body,
            body_f: f,
            body: None,
            json: None,
        }
    }
}

impl<F, S> Future for Json<F, S> where F: Future<Item=Body<S>, Error=Error>, S: 'static {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self.state {
            State::Body => match self.body_f.poll() {
                Ok(Async::Ready(body)) => {
                    let req = body.request.clone();
                    let f2 = req.json();
                    self.state = State::Json;
                    self.body = Some(body);
                    self.json = Some(f2);
                    Ok(Async::NotReady)
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e)
            },
            State::Json => match self.json.as_mut().unwrap().poll() {
                Ok(Async::Ready(json)) => {
                    let mut body = self.body.take().unwrap();
                    body.json = Some(json);
                    Ok(Async::Ready(body))
                },
                // If there was error during JSON parsing, just ignoring it.
                Err(_) => {
                    let mut body = self.body.take().unwrap();
                    Ok(Async::Ready(body))
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
            }
        }
    }
}

impl<F, S> Builder<S> for Json<F, S> where F: Future<Item=Body<S>, Error=Error>, S: 'static {}
