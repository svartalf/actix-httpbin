use std::convert::{From, Into};

use actix_web;
use serde_json;

pub enum Error {
    Empty,
    Serde(serde_json::Error),
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Empty
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl Into<actix_web::Error> for Error {
    fn into(self) -> actix_web::Error {
        unimplemented!()
    }
}
