use std::fmt;
use std::convert::{From, Into};

use actix_web::{error, self};
use serde_json;

#[derive(Debug)]
pub enum Error {
    Empty,
    Serde(serde_json::Error),
    JsonPayload(error::JsonPayloadError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Empty => f.write_str("Unknown error"),
            Error::Serde(_) => f.write_str("Failed to serialize"),
            Error::JsonPayload(_) => f.write_str("Failed to parse request payload as JSON"),
        }
    }
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

impl From<error::JsonPayloadError> for Error {
    fn from(e: error::JsonPayloadError) -> Self {
        Error::JsonPayload(e)
    }
}

impl Into<actix_web::Error> for Error {
    fn into(self) -> actix_web::Error {
        error::ErrorInternalServerError(self)
    }
}
