use std::mem;

use serde_json::Value;
use futures::{Future, Poll, Async};
use actix_web::{HttpRequest, HttpMessage};
use actix_web::dev::JsonBody;

use response::{Builder, Body, Error};


#[must_use = "futures do nothing unless polled"]
pub enum Json<F, S> {
    Body(F),
    Json((Option<Body<S>>, JsonBody<HttpRequest<S>, Value>)),
    Ready(Option<Body<S>>),
}

impl<F, S> Json<F, S> {
    pub fn new(f: F) -> Json<F, S> {
        Json::Body(f)
    }
}

impl<F, S> Future for Json<F, S> where F: Future<Item=Body<S>, Error=Error>, S: 'static {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        match self {
            Json::Body(ref mut f) => match f.poll() {
                Ok(Async::Ready(body)) => {
                    let req = body.request.clone();
                    match mem::replace(self, Json::Json((Some(body), req.json()))) {
                        Json::Body(_) => (),
                        _ => panic!(),
                    }

                    Ok(Async::NotReady)
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e.into()),
            },
            Json::Json((None, _)) => panic!(),
            Json::Json((body, ref mut f)) => match f.poll() {
                Ok(Async::Ready(json)) => {
                    let mut body = body.take().unwrap();
                    body.json = Some(json);
                    match mem::replace(self, Json::Ready(Some(body))) {
                        Json::Json(_) => (),
                        _ => panic!(),
                    }
                    Ok(Async::NotReady)
                },
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(e) => Err(e.into())
            },
            Json::Ready(body) => Ok(Async::Ready(body.take().unwrap()))
        }
    }
}

impl<F, S> Builder<S> for Json<F, S> where F: Future<Item=Body<S>, Error=Error>, S: 'static {}
