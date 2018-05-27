use uuid;
use base64;
use actix_web::{Json, HttpRequest, HttpResponse, Path, Result, error};

use application::State;
use templates::{Context, render};

mod drip;
mod bytes;
mod delay;
mod stream;
mod stream_bytes;

pub use self::bytes::bytes;
pub use self::delay::delay;
pub use self::stream::stream;
pub use self::stream_bytes::stream_bytes;
pub use self::drip::drip;

#[derive(Serialize)]
pub struct UUID {
    uuid: uuid::Uuid,
}

/// Returns a generated `UUID4`.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn uuid(_req: HttpRequest<State>) -> Json<UUID> {
    Json(UUID{
        uuid: uuid::Uuid::new_v4(),
    })
}

/// Returns base64url-encoded string.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn base64(info: Path<String>) -> Result<String> {
    base64::decode_config(&*info, base64::URL_SAFE)
        .map_err(error::ErrorBadRequest)
        .map(String::from_utf8)?
        .map_err(error::ErrorBadRequest)
}


pub fn links((req, info): (HttpRequest<State>, Path<(u8, u8)>)) -> Result<HttpResponse> {
    let (total, current) = *info;
    let mut context = Context::new();
    context.add("links", &(0..total).collect::<Vec<_>>());
    context.add("total", &total);
    context.add("current", &current);

    render(req, "links.html", &context)
}
