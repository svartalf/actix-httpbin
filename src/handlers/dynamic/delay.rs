use std::cmp;
use std::time::{Duration, Instant};

use actix_web::{HttpResponse, Path, error};
use actix_web::http::StatusCode;
use futures::Future;
use tokio_timer;

/// Returns a delayed response (max of 10 seconds).
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn delay(info: Path<u64>) -> Box<Future<Item=HttpResponse, Error=error::InternalError<tokio_timer::Error>>> {
    let n = cmp::min(*info, 10);
    let when = Instant::now() + Duration::new(n, 0);
    let future = tokio_timer::Delay::new(when)
        .and_then(|_| Ok(HttpResponse::build(StatusCode::OK).finish()))
        .map_err(|e| error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR));

    Box::new(future)
}
