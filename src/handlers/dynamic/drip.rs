use std::cmp;
use std::time::{Instant, Duration};

use bytes::Bytes;
use tokio_timer::Interval;
use actix_web::{self, HttpRequest, HttpResponse, Query, error};
use actix_web::http::StatusCode;
use futures::Stream;
// `actix-web` is not re-exporting this and probably it should
// Create an issue about it.
use http::status::InvalidStatusCode;

use application::State;


#[derive(Debug, Deserialize)]
pub struct DripQuery {
    duration: Option<u8>,  // TODO: duration should be a float
    numbytes: Option<u32>,
    code: Option<u16>,
    delay: Option<u8>,  // TODO: delay should be a float
}

impl DripQuery {
    fn duration(&self) -> Duration {
        let seconds = match self.duration {
            None => 2,
            Some(seconds) => u64::from(seconds)
        };

        Duration::from_secs(seconds)
    }

    fn num_bytes(&self) -> u32 {
        match self.numbytes {
            None => 10,
            Some(bytes) => cmp::min(bytes, 10 * 1024 * 1024),
        }
    }

    fn delay(&self) -> Duration {
        let seconds = match self.delay {
            None => 0,
            Some(seconds) => u64::from(seconds),
        };

        Duration::from_secs(seconds)
    }

    fn status_code(&self) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_u16(self.code.unwrap_or(200))
    }
}

/// Drips data over a duration after an optional initial delay.
pub fn drip((req, query): (HttpRequest<State>, Query<DripQuery>)) -> actix_web::Result<HttpResponse> {
    let duration = query.duration();
    let num_bytes = query.num_bytes();
    let code = query.status_code().map_err(error::ErrorBadRequest)?;
    let interval = duration / num_bytes;

    let stream = Interval::new(Instant::now() + query.delay(), interval)
        .map(|_| Bytes::from_static(b"*"))
        .map_err(error::ErrorInternalServerError)
        .take(num_bytes.into());

    let resp = req.build_response(code)
        .content_type("application/octet-stream")
        .content_length(num_bytes.into())
        .streaming(stream);
    Ok(resp)
}
