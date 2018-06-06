use std::collections::HashMap;

use actix_web::HttpRequest;
use futures::{Future, Poll, Async};
use serde::Serialize;
use serde_json::Value;

mod modifiers;
mod formatters;
mod errors;

pub use self::formatters::{JsonFuture, JsonValueFuture, ResponseFuture};
pub use self::errors::Error;
use map::PickyHashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Body<S> {
    #[serde(skip)]
    request: HttpRequest<S>,

    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<PickyHashMap<Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<PickyHashMap<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    json: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,

    #[serde(flatten)]
    extra: HashMap<&'static str, Value>,
}

impl<S> Body<S> {
    #[cfg_attr(feature = "cargo-clippy", allow(new_ret_no_self))]
    pub fn new(req: HttpRequest<S>) -> New<S> {
        New(Some(Body{
            request: req,
            method: None,
            url: None,
            origin: None,
            args: None,
            headers: None,
            user_agent: None,
            json: None,
            extra: HashMap::new(),
        }))
    }
}


pub trait Builder<S>: Future<Item=Body<S>> + Sized {
    fn with_method(self) -> modifiers::Method<Self> {
        modifiers::Method::new(self)
    }

    fn with_url(self) -> modifiers::Url<Self> {
        modifiers::Url::new(self)
    }

    fn with_args(self) -> modifiers::Args<Self> {
        modifiers::Args::new(self)
    }

    fn with_headers(self) -> modifiers::Headers<Self> {
        modifiers::Headers::new(self)
    }

    fn with_origin(self) -> modifiers::Origin<Self> {
        modifiers::Origin::new(self)
    }

    fn with_user_agent(self) -> modifiers::UserAgent<Self> {
        modifiers::UserAgent::new(self)
    }

    fn with_extra<V: Serialize>(self, key: &'static str, value: V) -> modifiers::Extra<Self, V> {
        modifiers::Extra::new(self, key, value)
    }

    fn into_json(self) -> formatters::JsonFormatter<Self> {
        formatters::JsonFormatter::new(self)
    }

    fn into_json_value(self) -> formatters::JsonValueFormatter<Self> {
        formatters::JsonValueFormatter::new(self)
    }

//    fn into_bytes(self) -> formatters::BytesFormatter<Self> {
//        formatters::BytesFormatter::new(self)
//    }

    fn into_response(self) -> formatters::ResponseFormatter<Self> {
        formatters::ResponseFormatter::new(self)
    }
}


/// Future that will be resolved into a `Body` instance.
#[derive(Debug, Clone)]
pub struct New<S>(Option<Body<S>>);

impl<S> Future for New<S> {
    type Item = Body<S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<<Self as Future>::Item, <Self as Future>::Error> {
        Ok(Async::Ready(self.0.take().expect("Can't poll twice")))
    }
}

impl<S> Builder<S> for New<S> {}
